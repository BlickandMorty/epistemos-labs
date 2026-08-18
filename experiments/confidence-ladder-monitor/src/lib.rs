//! Deterministic confidence admission with observable health and replayable receipts.

use core::fmt;
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    T1,
    T2,
    T3,
}

impl Tier {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::T1 => "T1",
            Self::T2 => "T2",
            Self::T3 => "T3",
        }
    }

    const fn receipt_code(self) -> u8 {
        match self {
            Self::T1 => 1,
            Self::T2 => 2,
            Self::T3 => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    Accepted(Tier),
    Escalated,
    Empty,
}

impl Decision {
    #[must_use]
    pub const fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted(_))
    }

    const fn receipt_code(self) -> u8 {
        match self {
            Self::Accepted(tier) => tier.receipt_code(),
            Self::Escalated => 4,
            Self::Empty => 5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LadderPolicy {
    pub t1: f64,
    pub t2: f64,
    pub t3: f64,
    pub healthy_high_tier_rate: f64,
    pub failing_unaccepted_rate: f64,
}

impl Default for LadderPolicy {
    fn default() -> Self {
        Self {
            t1: 0.85,
            t2: 0.75,
            t3: 0.70,
            healthy_high_tier_rate: 0.85,
            failing_unaccepted_rate: 0.20,
        }
    }
}

impl LadderPolicy {
    /// Validates probability ranges and tier ordering.
    ///
    /// # Errors
    ///
    /// Returns a [`LadderError`] when a field is non-finite, outside `[0, 1]`,
    /// or when the tier thresholds are not ordered from T1 through T3.
    pub fn validate(self) -> Result<Self, LadderError> {
        let values = [
            self.t1,
            self.t2,
            self.t3,
            self.healthy_high_tier_rate,
            self.failing_unaccepted_rate,
        ];
        if values.iter().any(|value| !value.is_finite()) {
            return Err(LadderError::NonFinitePolicy);
        }
        if values.iter().any(|value| !(0.0..=1.0).contains(value)) {
            return Err(LadderError::PolicyOutOfRange);
        }
        if !(self.t1 >= self.t2 && self.t2 >= self.t3) {
            return Err(LadderError::UnorderedTiers);
        }
        Ok(self)
    }

    fn decide(self, score: f64, escalate_on_empty: bool) -> Decision {
        if score >= self.t1 {
            Decision::Accepted(Tier::T1)
        } else if score >= self.t2 {
            Decision::Accepted(Tier::T2)
        } else if score >= self.t3 {
            Decision::Accepted(Tier::T3)
        } else if escalate_on_empty {
            Decision::Escalated
        } else {
            Decision::Empty
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LadderError {
    NonFinitePolicy,
    PolicyOutOfRange,
    UnorderedTiers,
    NonFiniteScore,
    ScoreOutOfRange,
}

impl fmt::Display for LadderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NonFinitePolicy => "policy values must be finite",
            Self::PolicyOutOfRange => "policy values must be in [0, 1]",
            Self::UnorderedTiers => "tier thresholds must satisfy T1 >= T2 >= T3",
            Self::NonFiniteScore => "score must be finite",
            Self::ScoreOutOfRange => "score must be in [0, 1]",
        })
    }
}

impl std::error::Error for LadderError {}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub sequence: u64,
    pub score: f64,
    pub escalate_on_empty: bool,
    pub decision: Decision,
    pub previous_hash: String,
    pub receipt_hash: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LadderStats {
    pub total: usize,
    pub mean_score: f64,
    pub standard_deviation: f64,
    pub t1_rate: f64,
    pub t2_rate: f64,
    pub t3_rate: f64,
    pub escalated_rate: f64,
    pub empty_rate: f64,
}

impl LadderStats {
    #[must_use]
    pub fn accepted_rate(self) -> f64 {
        self.t1_rate + self.t2_rate + self.t3_rate
    }

    #[must_use]
    pub fn unaccepted_rate(self) -> f64 {
        self.escalated_rate + self.empty_rate
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Health {
    Healthy,
    Degrading,
    Failing,
}

#[derive(Clone, Debug)]
pub struct ConfidenceLadder {
    policy: LadderPolicy,
    entries: Vec<Entry>,
    head: [u8; 32],
}

impl ConfidenceLadder {
    /// Creates an empty ladder from a validated policy.
    ///
    /// # Errors
    ///
    /// Returns a [`LadderError`] when the supplied policy is malformed.
    pub fn new(policy: LadderPolicy) -> Result<Self, LadderError> {
        Ok(Self {
            policy: policy.validate()?,
            entries: Vec::new(),
            head: [0; 32],
        })
    }

    /// Records one valid score and returns its immutable decision receipt.
    ///
    /// # Errors
    ///
    /// Returns a [`LadderError`] for a non-finite score or a score outside
    /// `[0, 1]`. Invalid observations never enter the log.
    pub fn observe(&mut self, score: f64, escalate_on_empty: bool) -> Result<Entry, LadderError> {
        if !score.is_finite() {
            return Err(LadderError::NonFiniteScore);
        }
        if !(0.0..=1.0).contains(&score) {
            return Err(LadderError::ScoreOutOfRange);
        }

        let sequence = u64::try_from(self.entries.len())
            .unwrap_or(u64::MAX)
            .saturating_add(1);
        let decision = self.policy.decide(score, escalate_on_empty);
        let previous_hash = self.head;
        let receipt_hash = receipt(
            previous_hash,
            sequence,
            score,
            escalate_on_empty,
            decision,
            self.policy,
        );
        self.head = receipt_hash;
        let entry = Entry {
            sequence,
            score,
            escalate_on_empty,
            decision,
            previous_hash: hex(previous_hash),
            receipt_hash: hex(receipt_hash),
        };
        self.entries.push(entry.clone());
        Ok(entry)
    }

    #[must_use]
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    #[must_use]
    pub fn head_hash(&self) -> String {
        hex(self.head)
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn stats(&self) -> Option<LadderStats> {
        let total = self.entries.len();
        if total == 0 {
            return None;
        }
        let denominator = total as f64;
        let mean_score = self.entries.iter().map(|entry| entry.score).sum::<f64>() / denominator;
        let variance = self
            .entries
            .iter()
            .map(|entry| (entry.score - mean_score).powi(2))
            .sum::<f64>()
            / denominator;
        let count = |decision: Decision| {
            self.entries
                .iter()
                .filter(|entry| entry.decision == decision)
                .count() as f64
                / denominator
        };
        Some(LadderStats {
            total,
            mean_score,
            standard_deviation: variance.sqrt(),
            t1_rate: count(Decision::Accepted(Tier::T1)),
            t2_rate: count(Decision::Accepted(Tier::T2)),
            t3_rate: count(Decision::Accepted(Tier::T3)),
            escalated_rate: count(Decision::Escalated),
            empty_rate: count(Decision::Empty),
        })
    }

    #[must_use]
    pub fn health(&self) -> Option<Health> {
        let stats = self.stats()?;
        if stats.unaccepted_rate() >= self.policy.failing_unaccepted_rate {
            Some(Health::Failing)
        } else if stats.t1_rate + stats.t2_rate >= self.policy.healthy_high_tier_rate {
            Some(Health::Healthy)
        } else {
            Some(Health::Degrading)
        }
    }

    #[must_use]
    pub fn verify(&self) -> bool {
        let mut previous = [0; 32];
        for (index, entry) in self.entries.iter().enumerate() {
            let Ok(sequence) = u64::try_from(index + 1) else {
                return false;
            };
            if entry.sequence != sequence || entry.previous_hash != hex(previous) {
                return false;
            }
            let expected = receipt(
                previous,
                sequence,
                entry.score,
                entry.escalate_on_empty,
                entry.decision,
                self.policy,
            );
            if entry.receipt_hash != hex(expected) {
                return false;
            }
            previous = expected;
        }
        previous == self.head
    }
}

fn receipt(
    previous: [u8; 32],
    sequence: u64,
    score: f64,
    escalate_on_empty: bool,
    decision: Decision,
    policy: LadderPolicy,
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"confidence-ladder-monitor/v1\0");
    hasher.update(previous);
    hasher.update(sequence.to_be_bytes());
    hasher.update(score.to_bits().to_be_bytes());
    hasher.update([u8::from(escalate_on_empty), decision.receipt_code()]);
    for value in [
        policy.t1,
        policy.t2,
        policy.t3,
        policy.healthy_high_tier_rate,
        policy.failing_unaccepted_rate,
    ] {
        hasher.update(value.to_bits().to_be_bytes());
    }
    hasher.finalize().into()
}

fn hex(bytes: [u8; 32]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(64);
    for byte in bytes {
        output.push(char::from(DIGITS[usize::from(byte >> 4)]));
        output.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ladder() -> ConfidenceLadder {
        ConfidenceLadder::new(LadderPolicy::default()).unwrap()
    }

    #[test]
    fn exact_boundaries_choose_the_named_tier() {
        let mut ladder = ladder();
        assert_eq!(
            ladder.observe(0.85, false).unwrap().decision,
            Decision::Accepted(Tier::T1)
        );
        assert_eq!(
            ladder.observe(0.75, false).unwrap().decision,
            Decision::Accepted(Tier::T2)
        );
        assert_eq!(
            ladder.observe(0.70, false).unwrap().decision,
            Decision::Accepted(Tier::T3)
        );
    }

    #[test]
    fn below_floor_respects_escalation_flag() {
        let mut ladder = ladder();
        assert_eq!(
            ladder.observe(0.69, true).unwrap().decision,
            Decision::Escalated
        );
        assert_eq!(
            ladder.observe(0.69, false).unwrap().decision,
            Decision::Empty
        );
    }

    #[test]
    fn invalid_scores_fail_closed_without_a_log_entry() {
        let mut ladder = ladder();
        assert_eq!(
            ladder.observe(f64::NAN, true),
            Err(LadderError::NonFiniteScore)
        );
        assert_eq!(
            ladder.observe(1.01, true),
            Err(LadderError::ScoreOutOfRange)
        );
        assert!(ladder.entries().is_empty());
    }

    #[test]
    fn malformed_policies_are_rejected() {
        let policy = LadderPolicy {
            t2: 0.90,
            ..LadderPolicy::default()
        };
        assert_eq!(
            ConfidenceLadder::new(policy).unwrap_err(),
            LadderError::UnorderedTiers
        );
        let policy = LadderPolicy {
            t1: f64::INFINITY,
            ..LadderPolicy::default()
        };
        assert_eq!(
            ConfidenceLadder::new(policy).unwrap_err(),
            LadderError::NonFinitePolicy
        );
    }

    #[test]
    fn stats_partition_every_observation() {
        let mut ladder = ladder();
        for (score, escalate) in [
            (0.9, false),
            (0.8, false),
            (0.72, false),
            (0.4, true),
            (0.3, false),
        ] {
            ladder.observe(score, escalate).unwrap();
        }
        let stats = ladder.stats().unwrap();
        let total_rate = stats.accepted_rate() + stats.unaccepted_rate();
        assert!((total_rate - 1.0).abs() < f64::EPSILON);
        assert!((stats.mean_score - 0.624).abs() < 1e-12);
    }

    #[test]
    fn health_has_explicit_healthy_degrading_and_failing_cases() {
        let mut healthy = ladder();
        for _ in 0..10 {
            healthy.observe(0.9, false).unwrap();
        }
        assert_eq!(healthy.health(), Some(Health::Healthy));

        let mut degrading = ladder();
        for _ in 0..10 {
            degrading.observe(0.72, false).unwrap();
        }
        assert_eq!(degrading.health(), Some(Health::Degrading));

        let mut failing = ladder();
        for _ in 0..8 {
            failing.observe(0.9, false).unwrap();
        }
        for _ in 0..2 {
            failing.observe(0.4, true).unwrap();
        }
        assert_eq!(failing.health(), Some(Health::Failing));
    }

    #[test]
    fn receipt_chain_is_deterministic_and_replayable() {
        let mut left = ladder();
        let mut right = ladder();
        for observation in [(0.91, false), (0.73, false), (0.5, true)] {
            left.observe(observation.0, observation.1).unwrap();
            right.observe(observation.0, observation.1).unwrap();
        }
        assert_eq!(left.head_hash(), right.head_hash());
        assert_eq!(left.entries(), right.entries());
        assert!(left.verify());
    }

    #[test]
    fn receipt_commits_to_policy_and_escalation() {
        let mut normal = ladder();
        normal.observe(0.5, false).unwrap();
        let mut escalated = ladder();
        escalated.observe(0.5, true).unwrap();
        assert_ne!(normal.head_hash(), escalated.head_hash());

        let policy = LadderPolicy {
            t3: 0.60,
            ..LadderPolicy::default()
        };
        let mut changed = ConfidenceLadder::new(policy).unwrap();
        changed.observe(0.5, false).unwrap();
        assert_ne!(normal.head_hash(), changed.head_hash());
    }
}
