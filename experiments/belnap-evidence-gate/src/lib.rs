//! Four-valued evidence aggregation with deterministic abstention receipts.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BelnapValue {
    True,
    False,
    Both,
    Neither,
}

impl BelnapValue {
    pub const ALL: [Self; 4] = [Self::True, Self::False, Self::Both, Self::Neither];

    const fn bits(self) -> (bool, bool) {
        match self {
            Self::True => (true, false),
            Self::False => (false, true),
            Self::Both => (true, true),
            Self::Neither => (false, false),
        }
    }

    const fn from_bits(supported: bool, refuted: bool) -> Self {
        match (supported, refuted) {
            (true, false) => Self::True,
            (false, true) => Self::False,
            (true, true) => Self::Both,
            (false, false) => Self::Neither,
        }
    }

    pub const fn not(self) -> Self {
        let (supported, refuted) = self.bits();
        Self::from_bits(refuted, supported)
    }

    /// Meet under the truth ordering: support decreases, refutation increases.
    pub const fn truth_meet(self, other: Self) -> Self {
        let (a_support, a_refute) = self.bits();
        let (b_support, b_refute) = other.bits();
        Self::from_bits(a_support && b_support, a_refute || b_refute)
    }

    /// Join under the truth ordering: support increases, refutation decreases.
    pub const fn truth_join(self, other: Self) -> Self {
        let (a_support, a_refute) = self.bits();
        let (b_support, b_refute) = other.bits();
        Self::from_bits(a_support || b_support, a_refute && b_refute)
    }

    /// Pool information from two sources (componentwise evidence union).
    pub const fn information_join(self, other: Self) -> Self {
        let (a_support, a_refute) = self.bits();
        let (b_support, b_refute) = other.bits();
        Self::from_bits(a_support || b_support, a_refute || b_refute)
    }

    /// Keep only evidence shared by both sources.
    pub const fn information_meet(self, other: Self) -> Self {
        let (a_support, a_refute) = self.bits();
        let (b_support, b_refute) = other.bits();
        Self::from_bits(a_support && b_support, a_refute && b_refute)
    }

    pub const fn is_classical(self) -> bool {
        matches!(self, Self::True | Self::False)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stance {
    Supports,
    Refutes,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Evidence {
    pub id: String,
    pub source_digest: String,
    pub stance: Stance,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateDecision {
    AssertSupported,
    RejectRefuted,
    AbstainConflict,
    AbstainUnknown,
}

impl GateDecision {
    pub const fn from_value(value: BelnapValue) -> Self {
        match value {
            BelnapValue::True => Self::AssertSupported,
            BelnapValue::False => Self::RejectRefuted,
            BelnapValue::Both => Self::AbstainConflict,
            BelnapValue::Neither => Self::AbstainUnknown,
        }
    }

    pub const fn abstains(self) -> bool {
        matches!(self, Self::AbstainConflict | Self::AbstainUnknown)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GateReceipt {
    pub version: u8,
    pub claim_id: String,
    pub evidence: Vec<Evidence>,
    pub value: BelnapValue,
    pub decision: GateDecision,
    pub digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GateError {
    EmptyClaim,
    EmptyEvidenceId,
    EmptySourceDigest,
    DuplicateEvidenceId(String),
}

pub fn evaluate(
    claim_id: impl Into<String>,
    evidence: Vec<Evidence>,
) -> Result<GateReceipt, GateError> {
    let claim_id = claim_id.into();
    if claim_id.trim().is_empty() {
        return Err(GateError::EmptyClaim);
    }
    let mut evidence = evidence;
    evidence.sort_by(|left, right| left.id.cmp(&right.id));
    let mut seen = BTreeSet::new();
    for item in &evidence {
        if item.id.trim().is_empty() {
            return Err(GateError::EmptyEvidenceId);
        }
        if item.source_digest.trim().is_empty() {
            return Err(GateError::EmptySourceDigest);
        }
        if !seen.insert(item.id.clone()) {
            return Err(GateError::DuplicateEvidenceId(item.id.clone()));
        }
    }
    let supported = evidence.iter().any(|item| item.stance == Stance::Supports);
    let refuted = evidence.iter().any(|item| item.stance == Stance::Refutes);
    let value = BelnapValue::from_bits(supported, refuted);
    let decision = GateDecision::from_value(value);
    let mut receipt = GateReceipt {
        version: 1,
        claim_id,
        evidence,
        value,
        decision,
        digest: String::new(),
    };
    receipt.digest = receipt_digest(&receipt);
    Ok(receipt)
}

fn put_field(target: &mut String, field: &str) {
    target.push_str(&field.len().to_string());
    target.push(':');
    target.push_str(field);
    target.push('|');
}

fn receipt_digest(receipt: &GateReceipt) -> String {
    let mut canonical = format!("belnap-gate-v{}|", receipt.version);
    put_field(&mut canonical, &receipt.claim_id);
    canonical.push_str(&format!("{:?}|{:?}|", receipt.value, receipt.decision));
    for item in &receipt.evidence {
        put_field(&mut canonical, &item.id);
        put_field(&mut canonical, &item.source_digest);
        canonical.push_str(&format!("{:?}|", item.stance));
    }
    format!("{:x}", Sha256::digest(canonical.as_bytes()))
}

pub fn verify(receipt: &GateReceipt) -> bool {
    if receipt.version != 1 || receipt.decision != GateDecision::from_value(receipt.value) {
        return false;
    }
    let Ok(rebuilt) = evaluate(receipt.claim_id.clone(), receipt.evidence.clone()) else {
        return false;
    };
    rebuilt.value == receipt.value && rebuilt.digest == receipt.digest
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence(id: &str, stance: Stance) -> Evidence {
        Evidence {
            id: id.into(),
            source_digest: format!("sha256:{id}"),
            stance,
        }
    }

    #[test]
    fn evidence_gate_covers_all_four_values() {
        assert_eq!(evaluate("c", vec![]).unwrap().value, BelnapValue::Neither);
        assert_eq!(
            evaluate("c", vec![evidence("a", Stance::Supports)])
                .unwrap()
                .value,
            BelnapValue::True
        );
        assert_eq!(
            evaluate("c", vec![evidence("a", Stance::Refutes)])
                .unwrap()
                .value,
            BelnapValue::False
        );
        assert_eq!(
            evaluate(
                "c",
                vec![
                    evidence("a", Stance::Supports),
                    evidence("b", Stance::Refutes)
                ]
            )
            .unwrap()
            .value,
            BelnapValue::Both
        );
    }

    #[test]
    fn gate_abstains_on_conflict_and_unknown_only() {
        for value in BelnapValue::ALL {
            assert_eq!(
                GateDecision::from_value(value).abstains(),
                !value.is_classical()
            );
        }
    }

    #[test]
    fn receipt_is_order_independent_and_replayable() {
        let left = evaluate(
            "claim",
            vec![
                evidence("b", Stance::Refutes),
                evidence("a", Stance::Supports),
            ],
        )
        .unwrap();
        let right = evaluate(
            "claim",
            vec![
                evidence("a", Stance::Supports),
                evidence("b", Stance::Refutes),
            ],
        )
        .unwrap();
        assert_eq!(left, right);
        assert!(verify(&left));
    }

    #[test]
    fn tampering_breaks_verification() {
        let mut receipt = evaluate("claim", vec![evidence("a", Stance::Supports)]).unwrap();
        receipt.evidence[0].source_digest = "changed".into();
        assert!(!verify(&receipt));
    }

    #[test]
    fn duplicate_evidence_is_rejected() {
        assert_eq!(
            evaluate(
                "claim",
                vec![
                    evidence("a", Stance::Supports),
                    evidence("a", Stance::Refutes)
                ]
            ),
            Err(GateError::DuplicateEvidenceId("a".into()))
        );
    }

    #[test]
    fn both_lattices_satisfy_core_algebra_laws() {
        for a in BelnapValue::ALL {
            assert_eq!(a.not().not(), a);
            for b in BelnapValue::ALL {
                assert_eq!(a.truth_join(b), b.truth_join(a));
                assert_eq!(a.truth_meet(b), b.truth_meet(a));
                assert_eq!(a.information_join(b), b.information_join(a));
                assert_eq!(a.information_meet(b), b.information_meet(a));
                assert_eq!(a.truth_join(a.truth_meet(b)), a);
                assert_eq!(a.information_join(a.information_meet(b)), a);
                for c in BelnapValue::ALL {
                    assert_eq!(a.truth_join(b).truth_join(c), a.truth_join(b.truth_join(c)));
                    assert_eq!(
                        a.information_join(b).information_join(c),
                        a.information_join(b.information_join(c))
                    );
                }
            }
        }
    }
}
