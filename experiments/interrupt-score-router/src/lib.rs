//! Five-signal escalation routing with deterministic, hash-chained receipts.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Signals {
    pub predictive_entropy: f64,
    pub budget_overrun_risk: f64,
    pub consistency_residual: f64,
    pub tool_need: f64,
    pub component_alarm: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Weights {
    pub predictive_entropy: f64,
    pub budget_overrun_risk: f64,
    pub consistency_residual: f64,
    pub tool_need: f64,
    pub component_alarm: f64,
}

impl Weights {
    pub const UNIFORM: Self = Self {
        predictive_entropy: 0.2,
        budget_overrun_risk: 0.2,
        consistency_residual: 0.2,
        tool_need: 0.2,
        component_alarm: 0.2,
    };

    fn values(self) -> [f64; 5] {
        [
            self.predictive_entropy,
            self.budget_overrun_risk,
            self.consistency_residual,
            self.tool_need,
            self.component_alarm,
        ]
    }
}

impl Signals {
    fn values(self) -> [f64; 5] {
        [
            self.predictive_entropy,
            self.budget_overrun_risk,
            self.consistency_residual,
            self.tool_need,
            self.component_alarm,
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Thresholds {
    pub recall: f64,
    pub escalate: f64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Route {
    Continue,
    Recall,
    Escalate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationError {
    NonFiniteSignal,
    SignalOutsideUnitInterval,
    InvalidWeight,
    WeightsDoNotSumToOne,
    InvalidThresholds,
    InvalidMaximumEscalationRate,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteReceipt {
    pub version: u8,
    pub sequence: u64,
    pub signals: Signals,
    pub weights: Weights,
    pub thresholds: Thresholds,
    pub score: f64,
    pub route: Route,
    pub previous_digest: String,
    pub digest: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Router {
    weights: Weights,
    thresholds: Thresholds,
    receipts: Vec<RouteReceipt>,
}

impl Router {
    pub fn new(weights: Weights, thresholds: Thresholds) -> Result<Self, ValidationError> {
        validate_weights(weights)?;
        validate_thresholds(thresholds)?;
        Ok(Self {
            weights,
            thresholds,
            receipts: Vec::new(),
        })
    }

    pub fn route(&mut self, signals: Signals) -> Result<&RouteReceipt, ValidationError> {
        validate_signals(signals)?;
        let score = signals
            .values()
            .into_iter()
            .zip(self.weights.values())
            .map(|(signal, weight)| signal * weight)
            .sum();
        let route = if score < self.thresholds.recall {
            Route::Continue
        } else if score < self.thresholds.escalate {
            Route::Recall
        } else {
            Route::Escalate
        };
        let sequence = self.receipts.len() as u64;
        let previous_digest = self
            .receipts
            .last()
            .map(|receipt| receipt.digest.clone())
            .unwrap_or_else(|| "0".repeat(64));
        let mut receipt = RouteReceipt {
            version: 1,
            sequence,
            signals,
            weights: self.weights,
            thresholds: self.thresholds,
            score,
            route,
            previous_digest,
            digest: String::new(),
        };
        receipt.digest = digest(&receipt);
        self.receipts.push(receipt);
        Ok(self.receipts.last().expect("receipt was appended"))
    }

    pub fn receipts(&self) -> &[RouteReceipt] {
        &self.receipts
    }
}

fn validate_signals(signals: Signals) -> Result<(), ValidationError> {
    for value in signals.values() {
        if !value.is_finite() {
            return Err(ValidationError::NonFiniteSignal);
        }
        if !(0.0..=1.0).contains(&value) {
            return Err(ValidationError::SignalOutsideUnitInterval);
        }
    }
    Ok(())
}

fn validate_weights(weights: Weights) -> Result<(), ValidationError> {
    let values = weights.values();
    if values
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0)
    {
        return Err(ValidationError::InvalidWeight);
    }
    if (values.iter().sum::<f64>() - 1.0).abs() > 1e-12 {
        return Err(ValidationError::WeightsDoNotSumToOne);
    }
    Ok(())
}

fn validate_thresholds(thresholds: Thresholds) -> Result<(), ValidationError> {
    if !thresholds.recall.is_finite()
        || !thresholds.escalate.is_finite()
        || !(0.0..=1.0).contains(&thresholds.recall)
        || !(0.0..=1.0).contains(&thresholds.escalate)
        || thresholds.recall > thresholds.escalate
    {
        return Err(ValidationError::InvalidThresholds);
    }
    Ok(())
}

fn digest(receipt: &RouteReceipt) -> String {
    let mut canonical = format!(
        "interrupt-router-v{}|{}|{}|",
        receipt.version, receipt.sequence, receipt.previous_digest
    );
    for value in receipt
        .signals
        .values()
        .into_iter()
        .chain(receipt.weights.values())
    {
        canonical.push_str(&format!("{:016x}|", value.to_bits()));
    }
    canonical.push_str(&format!(
        "{:016x}|{:016x}|{:016x}|{:?}",
        receipt.thresholds.recall.to_bits(),
        receipt.thresholds.escalate.to_bits(),
        receipt.score.to_bits(),
        receipt.route
    ));
    format!("{:x}", Sha256::digest(canonical.as_bytes()))
}

pub fn verify(receipts: &[RouteReceipt]) -> bool {
    let mut previous = "0".repeat(64);
    for (sequence, receipt) in receipts.iter().enumerate() {
        if receipt.version != 1
            || receipt.sequence != sequence as u64
            || receipt.previous_digest != previous
        {
            return false;
        }
        if validate_signals(receipt.signals).is_err()
            || validate_weights(receipt.weights).is_err()
            || validate_thresholds(receipt.thresholds).is_err()
        {
            return false;
        }
        let score: f64 = receipt
            .signals
            .values()
            .into_iter()
            .zip(receipt.weights.values())
            .map(|(signal, weight)| signal * weight)
            .sum();
        let route = if score < receipt.thresholds.recall {
            Route::Continue
        } else if score < receipt.thresholds.escalate {
            Route::Recall
        } else {
            Route::Escalate
        };
        if score.to_bits() != receipt.score.to_bits()
            || route != receipt.route
            || digest(receipt) != receipt.digest
        {
            return false;
        }
        previous = receipt.digest.clone();
    }
    true
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EscalationAudit {
    pub total: usize,
    pub escalated: usize,
    pub rate: f64,
    pub maximum_rate: f64,
    pub collapsed_to_static_hybrid: bool,
}

pub fn audit_escalation(
    receipts: &[RouteReceipt],
    maximum_rate: f64,
) -> Result<EscalationAudit, ValidationError> {
    if !maximum_rate.is_finite() || !(0.0..=1.0).contains(&maximum_rate) {
        return Err(ValidationError::InvalidMaximumEscalationRate);
    }
    let escalated = receipts
        .iter()
        .filter(|receipt| receipt.route == Route::Escalate)
        .count();
    let rate = if receipts.is_empty() {
        0.0
    } else {
        escalated as f64 / receipts.len() as f64
    };
    Ok(EscalationAudit {
        total: receipts.len(),
        escalated,
        rate,
        maximum_rate,
        collapsed_to_static_hybrid: rate > maximum_rate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signals(value: f64) -> Signals {
        Signals {
            predictive_entropy: value,
            budget_overrun_risk: value,
            consistency_residual: value,
            tool_need: value,
            component_alarm: value,
        }
    }

    fn router() -> Router {
        Router::new(
            Weights::UNIFORM,
            Thresholds {
                recall: 0.25,
                escalate: 0.70,
            },
        )
        .unwrap()
    }

    #[test]
    fn boundaries_have_unambiguous_routes() {
        let mut router = router();
        assert_eq!(router.route(signals(0.249)).unwrap().route, Route::Continue);
        assert_eq!(router.route(signals(0.25)).unwrap().route, Route::Recall);
        assert_eq!(router.route(signals(0.699)).unwrap().route, Route::Recall);
        assert_eq!(router.route(signals(0.70)).unwrap().route, Route::Escalate);
    }

    #[test]
    fn invalid_inputs_fail_closed() {
        let mut router = router();
        assert_eq!(
            router.route(signals(f64::NAN)),
            Err(ValidationError::NonFiniteSignal)
        );
        assert_eq!(
            router.route(signals(1.1)),
            Err(ValidationError::SignalOutsideUnitInterval)
        );
        assert!(router.receipts().is_empty());
    }

    #[test]
    fn same_inputs_produce_same_receipt_chain() {
        let mut left = router();
        let mut right = router();
        for value in [0.1, 0.4, 0.9] {
            left.route(signals(value)).unwrap();
            right.route(signals(value)).unwrap();
        }
        assert_eq!(left.receipts(), right.receipts());
        assert!(verify(left.receipts()));
    }

    #[test]
    fn mutation_breaks_replay() {
        let mut router = router();
        router.route(signals(0.9)).unwrap();
        let mut receipts = router.receipts().to_vec();
        receipts[0].signals.tool_need = 0.0;
        assert!(!verify(&receipts));
    }

    #[test]
    fn escalation_rate_is_an_explicit_falsifier() {
        let mut router = router();
        for value in [0.9, 0.9, 0.1, 0.1] {
            router.route(signals(value)).unwrap();
        }
        let audit = audit_escalation(router.receipts(), 0.20).unwrap();
        assert_eq!(audit.rate, 0.5);
        assert!(audit.collapsed_to_static_hybrid);
    }

    #[test]
    fn malformed_policy_is_rejected() {
        assert_eq!(
            Router::new(
                Weights {
                    predictive_entropy: 1.0,
                    ..Weights::UNIFORM
                },
                Thresholds {
                    recall: 0.2,
                    escalate: 0.8
                }
            ),
            Err(ValidationError::WeightsDoNotSumToOne)
        );
        assert_eq!(
            Router::new(
                Weights::UNIFORM,
                Thresholds {
                    recall: 0.8,
                    escalate: 0.2
                }
            ),
            Err(ValidationError::InvalidThresholds)
        );
    }
}
