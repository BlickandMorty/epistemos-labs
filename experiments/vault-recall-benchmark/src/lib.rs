//! A transparent reconstruction of the Epistemos F-Vault-Recall-50 idea.

use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub struct Note {
    pub id: String,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct Case {
    pub query: String,
    pub relevant: BTreeSet<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CaseResult {
    pub ranked: Vec<String>,
    pub recall: f64,
    pub reciprocal_rank: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Summary {
    pub cases: usize,
    pub mean_recall: f64,
    pub mean_reciprocal_rank: f64,
}

fn tokens(text: &str) -> BTreeSet<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|p| !p.is_empty())
        .map(|p| p.to_lowercase())
        .collect()
}

pub fn rank(notes: &[Note], query: &str, limit: usize) -> Vec<String> {
    let query = tokens(query);
    let mut scored: Vec<(usize, &str)> = notes
        .iter()
        .map(|note| {
            (
                query.intersection(&tokens(&note.text)).count(),
                note.id.as_str(),
            )
        })
        .filter(|(score, _)| *score > 0)
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(b.1)));
    scored
        .into_iter()
        .take(limit)
        .map(|(_, id)| id.to_string())
        .collect()
}

pub fn evaluate(notes: &[Note], case: &Case, limit: usize) -> CaseResult {
    let ranked = rank(notes, &case.query, limit);
    let found = ranked
        .iter()
        .filter(|id| case.relevant.contains(*id))
        .count();
    let recall = if case.relevant.is_empty() {
        1.0
    } else {
        found as f64 / case.relevant.len() as f64
    };
    let reciprocal_rank = ranked
        .iter()
        .position(|id| case.relevant.contains(id))
        .map(|i| 1.0 / (i + 1) as f64)
        .unwrap_or(0.0);
    CaseResult {
        ranked,
        recall,
        reciprocal_rank,
    }
}

pub fn benchmark(notes: &[Note], cases: &[Case], limit: usize) -> Summary {
    let results: Vec<_> = cases
        .iter()
        .map(|case| evaluate(notes, case, limit))
        .collect();
    let count = results.len();
    if count == 0 {
        return Summary {
            cases: 0,
            mean_recall: 1.0,
            mean_reciprocal_rank: 1.0,
        };
    }
    Summary {
        cases: count,
        mean_recall: results.iter().map(|r| r.recall).sum::<f64>() / count as f64,
        mean_reciprocal_rank: results.iter().map(|r| r.reciprocal_rank).sum::<f64>() / count as f64,
    }
}

pub fn synthetic_50() -> (Vec<Note>, Vec<Case>) {
    let mut notes = Vec::new();
    let mut cases = Vec::new();
    for index in 0..50 {
        let id = format!("note-{index:02}");
        let marker = format!("topic{index:02}");
        notes.push(Note {
            id: id.clone(),
            text: format!("research record for {marker} deterministic vault fixture"),
        });
        cases.push(Case {
            query: marker,
            relevant: [id].into_iter().collect(),
        });
    }
    (notes, cases)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn synthetic_fifty_is_perfect() {
        let (notes, cases) = synthetic_50();
        let summary = benchmark(&notes, &cases, 5);
        assert_eq!(summary.cases, 50);
        assert_eq!(summary.mean_recall, 1.0);
        assert_eq!(summary.mean_reciprocal_rank, 1.0);
    }
    #[test]
    fn tie_breaks_by_stable_id() {
        let notes = vec![
            Note {
                id: "b".into(),
                text: "same".into(),
            },
            Note {
                id: "a".into(),
                text: "same".into(),
            },
        ];
        assert_eq!(rank(&notes, "same", 2), vec!["a", "b"]);
    }
    #[test]
    fn missed_relevant_note_is_visible() {
        let result = evaluate(
            &[],
            &Case {
                query: "x".into(),
                relevant: ["a".into()].into_iter().collect(),
            },
            5,
        );
        assert_eq!(result.recall, 0.0);
        assert_eq!(result.reciprocal_rank, 0.0);
    }
}
