//! Deterministic schema repair with a bounded loop and an auditable witness.

use std::collections::{BTreeMap, BTreeSet};

pub type Document = BTreeMap<String, String>;

#[derive(Clone, Debug)]
pub struct Schema {
    pub required: BTreeSet<String>,
    pub defaults: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Patch {
    pub field: String,
    pub before: Option<String>,
    pub after: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Witness {
    pub rounds: Vec<Vec<Patch>>,
    pub converged: bool,
}

impl Schema {
    pub fn missing(&self, document: &Document) -> Vec<String> {
        self.required
            .iter()
            .filter(|field| !document.contains_key(*field))
            .cloned()
            .collect()
    }

    pub fn repair(&self, document: &mut Document, maximum_rounds: usize) -> Witness {
        let mut rounds = Vec::new();
        for _ in 0..maximum_rounds {
            let mut patches = Vec::new();
            for field in self.missing(document) {
                if let Some(default) = self.defaults.get(&field) {
                    document.insert(field.clone(), default.clone());
                    patches.push(Patch {
                        field,
                        before: None,
                        after: default.clone(),
                    });
                }
            }
            if patches.is_empty() {
                return Witness {
                    rounds,
                    converged: self.missing(document).is_empty(),
                };
            }
            rounds.push(patches);
        }
        Witness {
            rounds,
            converged: self.missing(document).is_empty(),
        }
    }
}

pub fn replay(original: &Document, witness: &Witness) -> Document {
    let mut document = original.clone();
    for round in &witness.rounds {
        for patch in round {
            document.insert(patch.field.clone(), patch.after.clone());
        }
    }
    document
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema() -> Schema {
        Schema {
            required: ["id".into(), "version".into()].into_iter().collect(),
            defaults: [("version".into(), "1".into())].into_iter().collect(),
        }
    }

    #[test]
    fn repair_converges_and_replays() {
        let original: [(String, String); 1] = [("id".into(), "doc-1".into())];
        let mut document: Document = original.clone().into_iter().collect();
        let witness = schema().repair(&mut document, 4);
        assert!(witness.converged);
        assert_eq!(replay(&original.into_iter().collect(), &witness), document);
    }

    #[test]
    fn missing_default_cannot_be_hidden() {
        let mut document = Document::new();
        let witness = schema().repair(&mut document, 4);
        assert!(!witness.converged);
        assert_eq!(schema().missing(&document), vec!["id"]);
    }

    #[test]
    fn zero_round_budget_changes_nothing() {
        let mut document = Document::new();
        let witness = schema().repair(&mut document, 0);
        assert!(witness.rounds.is_empty());
        assert!(!witness.converged);
    }
}
