//! Deterministic decisions, canonical receipts, and replay verification.

use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Capability {
    Read,
    Write,
    Execute,
    Network,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Action {
    pub capability: Capability,
    pub resource: String,
    pub payload_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Outcome {
    Allowed,
    Denied(&'static str),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Receipt {
    pub sequence: u64,
    pub action: Action,
    pub outcome: Outcome,
    pub previous_hash: u64,
    pub hash: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Policy {
    allowed: BTreeSet<Capability>,
}

impl Policy {
    pub fn allow(mut self, capability: Capability) -> Self {
        self.allowed.insert(capability);
        self
    }

    pub fn admits(&self, action: &Action) -> bool {
        self.allowed.contains(&action.capability)
    }
}

#[derive(Clone, Debug)]
pub struct Kernel {
    policy: Policy,
    receipts: Vec<Receipt>,
}

impl Kernel {
    pub fn new(policy: Policy) -> Self {
        Self {
            policy,
            receipts: Vec::new(),
        }
    }

    pub fn decide(&mut self, action: Action) -> &Receipt {
        let outcome = if self.policy.admits(&action) {
            Outcome::Allowed
        } else {
            Outcome::Denied("capability is outside the admitted set")
        };
        let sequence = self.receipts.len() as u64;
        let previous_hash = self.receipts.last().map(|r| r.hash).unwrap_or(0);
        let hash = receipt_hash(sequence, &action, &outcome, previous_hash);
        self.receipts.push(Receipt {
            sequence,
            action,
            outcome,
            previous_hash,
            hash,
        });
        self.receipts.last().expect("receipt was appended")
    }

    pub fn receipts(&self) -> &[Receipt] {
        &self.receipts
    }
}

pub fn stable_hash(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn capability_code(capability: &Capability) -> &'static str {
    match capability {
        Capability::Read => "read",
        Capability::Write => "write",
        Capability::Execute => "execute",
        Capability::Network => "network",
    }
}

fn outcome_code(outcome: &Outcome) -> String {
    match outcome {
        Outcome::Allowed => "allowed".into(),
        Outcome::Denied(reason) => format!("denied:{reason}"),
    }
}

fn receipt_hash(sequence: u64, action: &Action, outcome: &Outcome, previous: u64) -> u64 {
    let canonical = format!(
        "v1|{sequence}|{}|{}|{:016x}|{}|{previous:016x}",
        capability_code(&action.capability),
        action.resource,
        action.payload_digest,
        outcome_code(outcome)
    );
    stable_hash(canonical.as_bytes())
}

pub fn verify_replay(receipts: &[Receipt]) -> Result<(), String> {
    let mut previous = 0;
    for (index, receipt) in receipts.iter().enumerate() {
        if receipt.sequence != index as u64 {
            return Err(format!("sequence mismatch at {index}"));
        }
        if receipt.previous_hash != previous {
            return Err(format!("chain mismatch at {index}"));
        }
        let expected = receipt_hash(
            receipt.sequence,
            &receipt.action,
            &receipt.outcome,
            previous,
        );
        if receipt.hash != expected {
            return Err(format!("receipt mismatch at {index}"));
        }
        previous = receipt.hash;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn action(capability: Capability, name: &str) -> Action {
        Action {
            capability,
            resource: name.into(),
            payload_digest: stable_hash(name.as_bytes()),
        }
    }

    #[test]
    fn same_inputs_produce_same_chain() {
        let policy = Policy::default()
            .allow(Capability::Read)
            .allow(Capability::Write);
        let mut left = Kernel::new(policy.clone());
        let mut right = Kernel::new(policy);
        for item in [
            action(Capability::Read, "vault/a"),
            action(Capability::Write, "vault/b"),
        ] {
            left.decide(item.clone());
            right.decide(item);
        }
        assert_eq!(left.receipts(), right.receipts());
        assert!(verify_replay(left.receipts()).is_ok());
    }

    #[test]
    fn denied_actions_are_still_receipted() {
        let mut kernel = Kernel::new(Policy::default().allow(Capability::Read));
        let receipt = kernel.decide(action(Capability::Network, "remote"));
        assert!(matches!(receipt.outcome, Outcome::Denied(_)));
        assert!(verify_replay(kernel.receipts()).is_ok());
    }

    #[test]
    fn tampering_breaks_replay() {
        let mut kernel = Kernel::new(Policy::default().allow(Capability::Read));
        kernel.decide(action(Capability::Read, "vault/a"));
        let mut receipts = kernel.receipts().to_vec();
        receipts[0].action.resource = "vault/changed".into();
        assert!(verify_replay(&receipts).is_err());
    }
}
