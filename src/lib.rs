//! A compact reconstruction of the Scope-Rex / ACS admission field.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Risk {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Debug)]
pub struct ScopeManifest {
    pub scope_id: String,
    pub allowed_actions: BTreeSet<String>,
    pub allowed_roots: Vec<String>,
    pub maximum_risk: Risk,
}

#[derive(Clone, Debug)]
pub struct Request {
    pub action: String,
    pub resource: String,
    pub risk: Risk,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Verdict {
    Admit,
    Deny(Vec<&'static str>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmissionProof {
    pub scope_id: String,
    pub request_digest: u64,
    pub verdict: Verdict,
    pub receipt_id: u64,
}

impl ScopeManifest {
    pub fn admit(&self, request: &Request) -> AdmissionProof {
        let mut reasons = Vec::new();
        if !self.allowed_actions.contains(&request.action) {
            reasons.push("action is outside scope");
        }
        if !self
            .allowed_roots
            .iter()
            .any(|root| contained(root, &request.resource))
        {
            reasons.push("resource is outside scope");
        }
        if request.risk > self.maximum_risk {
            reasons.push("risk exceeds the admitted ceiling");
        }
        if request.evidence_ids.is_empty() {
            reasons.push("request has no evidence reference");
        }
        let request_digest = hash(&canonical_request(request));
        let verdict = if reasons.is_empty() {
            Verdict::Admit
        } else {
            Verdict::Deny(reasons)
        };
        let receipt_id =
            hash(format!("v1|{}|{request_digest:016x}|{:?}", self.scope_id, verdict).as_bytes());
        AdmissionProof {
            scope_id: self.scope_id.clone(),
            request_digest,
            verdict,
            receipt_id,
        }
    }
}

fn contained(root: &str, resource: &str) -> bool {
    let root = root.trim_end_matches('/');
    resource == root
        || resource
            .strip_prefix(root)
            .is_some_and(|tail| tail.starts_with('/'))
}

fn canonical_request(request: &Request) -> Vec<u8> {
    let mut evidence = request.evidence_ids.clone();
    evidence.sort();
    format!(
        "v1|{}|{}|{:?}|{}",
        request.action,
        request.resource,
        request.risk,
        evidence.join(",")
    )
    .into_bytes()
}

fn hash(bytes: &[u8]) -> u64 {
    let mut value = 0xcbf29ce484222325u64;
    for byte in bytes {
        value ^= *byte as u64;
        value = value.wrapping_mul(0x100000001b3);
    }
    value
}

pub fn verify(manifest: &ScopeManifest, request: &Request, proof: &AdmissionProof) -> bool {
    manifest.admit(request) == *proof
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest() -> ScopeManifest {
        ScopeManifest {
            scope_id: "local-research".into(),
            allowed_actions: ["read".into(), "index".into()].into_iter().collect(),
            allowed_roots: vec!["vault".into()],
            maximum_risk: Risk::Medium,
        }
    }

    #[test]
    fn admits_bounded_evidenced_request() {
        let request = Request {
            action: "read".into(),
            resource: "vault/a.md".into(),
            risk: Risk::Low,
            evidence_ids: vec!["ticket-1".into()],
        };
        let proof = manifest().admit(&request);
        assert_eq!(proof.verdict, Verdict::Admit);
        assert!(verify(&manifest(), &request, &proof));
    }

    #[test]
    fn path_prefix_confusion_is_denied() {
        let request = Request {
            action: "read".into(),
            resource: "vault-escape/a".into(),
            risk: Risk::Low,
            evidence_ids: vec!["ticket-1".into()],
        };
        assert!(matches!(
            manifest().admit(&request).verdict,
            Verdict::Deny(_)
        ));
    }

    #[test]
    fn evidence_order_does_not_change_receipt() {
        let mut one = Request {
            action: "read".into(),
            resource: "vault/a".into(),
            risk: Risk::Low,
            evidence_ids: vec!["b".into(), "a".into()],
        };
        let first = manifest().admit(&one);
        one.evidence_ids.reverse();
        assert_eq!(first, manifest().admit(&one));
    }
}
