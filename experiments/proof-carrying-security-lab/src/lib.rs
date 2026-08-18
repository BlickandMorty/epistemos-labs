//! Safe-by-default security lab: authorized scope, read-only inputs, proof-backed reports.

#[derive(Clone, Debug)]
pub struct Scope {
    pub engagement_id: String,
    pub authorized: bool,
    pub allowed_files: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub path: String,
    pub text: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FindingKind {
    EmbeddedCredential,
    SqlStringConstruction,
    InsecureRandomToken,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evidence {
    pub path: String,
    pub line: usize,
    pub excerpt: String,
    pub digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Finding {
    pub kind: FindingKind,
    pub evidence: Evidence,
    pub remediation: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Report {
    pub engagement_id: String,
    pub findings: Vec<Finding>,
}

pub fn analyze(scope: &Scope, files: &[SourceFile]) -> Result<Report, String> {
    if !scope.authorized {
        return Err("written authorization flag is required".into());
    }
    let mut findings = Vec::new();
    for file in files {
        if !scope
            .allowed_files
            .iter()
            .any(|allowed| allowed == &file.path)
        {
            return Err(format!("{} is outside the engagement scope", file.path));
        }
        for (index, line) in file.text.lines().enumerate() {
            let checks = [
                (
                    FindingKind::EmbeddedCredential,
                    looks_like_credential(line),
                    "Move credentials to a secret store and rotate the exposed value.",
                ),
                (
                    FindingKind::SqlStringConstruction,
                    looks_like_sql_concat(line),
                    "Use a parameterized query API.",
                ),
                (
                    FindingKind::InsecureRandomToken,
                    looks_like_weak_token(line),
                    "Use an operating-system cryptographic random source.",
                ),
            ];
            for (kind, matched, remediation) in checks {
                if matched {
                    let excerpt = redact(line);
                    let digest =
                        hash(format!("{}|{}|{}", file.path, index + 1, excerpt).as_bytes());
                    findings.push(Finding {
                        kind,
                        evidence: Evidence {
                            path: file.path.clone(),
                            line: index + 1,
                            excerpt,
                            digest,
                        },
                        remediation,
                    });
                }
            }
        }
    }
    findings.sort_by(|a, b| {
        a.evidence
            .path
            .cmp(&b.evidence.path)
            .then_with(|| a.evidence.line.cmp(&b.evidence.line))
            .then_with(|| format!("{:?}", a.kind).cmp(&format!("{:?}", b.kind)))
    });
    Ok(Report {
        engagement_id: scope.engagement_id.clone(),
        findings,
    })
}

pub fn verify_evidence(file: &SourceFile, evidence: &Evidence) -> bool {
    let Some(line) = file.text.lines().nth(evidence.line.saturating_sub(1)) else {
        return false;
    };
    let excerpt = redact(line);
    excerpt == evidence.excerpt
        && hash(format!("{}|{}|{}", file.path, evidence.line, excerpt).as_bytes())
            == evidence.digest
}

fn looks_like_credential(line: &str) -> bool {
    let lower = line.to_lowercase();
    (lower.contains("api_key") || lower.contains("secret_key"))
        && line.contains('=')
        && !lower.contains("getenv")
        && !lower.contains("env::")
}
fn looks_like_sql_concat(line: &str) -> bool {
    let lower = line.to_lowercase();
    lower.contains("select ") && (line.contains("+") || lower.contains("format!("))
}
fn looks_like_weak_token(line: &str) -> bool {
    let lower = line.to_lowercase();
    lower.contains("token") && (lower.contains("math.random") || lower.contains("rand::random"))
}

fn redact(line: &str) -> String {
    if let Some((left, _)) = line.split_once('=') {
        if left.to_lowercase().contains("key") {
            return format!("{}=<redacted>", left.trim());
        }
    }
    line.trim().chars().take(160).collect()
}
fn hash(bytes: &[u8]) -> u64 {
    let mut value = 0xcbf29ce484222325u64;
    for byte in bytes {
        value ^= *byte as u64;
        value = value.wrapping_mul(0x100000001b3)
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    fn scope() -> Scope {
        Scope {
            engagement_id: "lab-1".into(),
            authorized: true,
            allowed_files: vec!["app.rs".into()],
        }
    }
    #[test]
    fn refuses_unauthorized_runs() {
        let mut denied = scope();
        denied.authorized = false;
        assert!(analyze(&denied, &[]).is_err());
    }
    #[test]
    fn refuses_scope_drift() {
        let file = SourceFile {
            path: "other.rs".into(),
            text: String::new(),
        };
        assert!(analyze(&scope(), &[file]).is_err());
    }
    #[test]
    fn finding_carries_replayable_redacted_evidence() {
        let file = SourceFile {
            path: "app.rs".into(),
            text: "let api_key = \"demo-secret\";".into(),
        };
        let report = analyze(&scope(), std::slice::from_ref(&file)).unwrap();
        assert_eq!(report.findings.len(), 1);
        assert!(!report.findings[0].evidence.excerpt.contains("demo-secret"));
        assert!(verify_evidence(&file, &report.findings[0].evidence));
    }
    #[test]
    fn environment_lookup_is_not_flagged_as_embedded() {
        let file = SourceFile {
            path: "app.rs".into(),
            text: "let api_key = env::var(\"API_KEY\");".into(),
        };
        assert!(analyze(&scope(), &[file]).unwrap().findings.is_empty());
    }
}
