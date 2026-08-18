//! Closed-citation retrieval: every returned span must exist in the admitted corpus.

use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub struct Document {
    pub id: String,
    pub text: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Citation {
    pub document_id: String,
    pub byte_start: usize,
    pub byte_end: usize,
    pub quote: String,
    pub score_milli: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextPacket {
    pub query: String,
    pub citations: Vec<Citation>,
    pub corpus_digest: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Eidos {
    documents: Vec<Document>,
}

impl Eidos {
    pub fn insert(&mut self, document: Document) {
        if let Some(old) = self
            .documents
            .iter_mut()
            .find(|item| item.id == document.id)
        {
            *old = document;
        } else {
            self.documents.push(document);
        }
        self.documents.sort_by(|a, b| a.id.cmp(&b.id));
    }

    pub fn retrieve(&self, query: &str, limit: usize) -> ContextPacket {
        let query_tokens = tokens(query);
        let mut citations: Vec<Citation> = self
            .documents
            .iter()
            .filter_map(|doc| {
                let doc_tokens = tokens(&doc.text);
                let overlap = query_tokens.intersection(&doc_tokens).count() as u32;
                if overlap == 0 {
                    return None;
                }
                let score_milli = overlap * 1000 / query_tokens.len().max(1) as u32;
                Some(Citation {
                    document_id: doc.id.clone(),
                    byte_start: 0,
                    byte_end: doc.text.len(),
                    quote: doc.text.clone(),
                    score_milli,
                })
            })
            .collect();
        citations.sort_by(|a, b| {
            b.score_milli
                .cmp(&a.score_milli)
                .then_with(|| a.document_id.cmp(&b.document_id))
        });
        citations.truncate(limit);
        ContextPacket {
            query: query.into(),
            citations,
            corpus_digest: self.corpus_digest(),
        }
    }

    pub fn validate(&self, packet: &ContextPacket) -> Result<(), String> {
        if packet.corpus_digest != self.corpus_digest() {
            return Err("corpus digest changed".into());
        }
        for citation in &packet.citations {
            let doc = self
                .documents
                .iter()
                .find(|doc| doc.id == citation.document_id)
                .ok_or_else(|| format!("missing document {}", citation.document_id))?;
            let span = doc
                .text
                .get(citation.byte_start..citation.byte_end)
                .ok_or_else(|| "citation boundary is invalid".to_string())?;
            if span != citation.quote {
                return Err(format!("quote mismatch for {}", citation.document_id));
            }
        }
        Ok(())
    }

    fn corpus_digest(&self) -> u64 {
        let mut bytes = Vec::new();
        for document in &self.documents {
            bytes.extend_from_slice(document.id.as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(document.text.as_bytes());
            bytes.push(0xff);
        }
        hash(&bytes)
    }
}

fn tokens(text: &str) -> BTreeSet<String> {
    text.split(|ch: char| !ch.is_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| part.to_lowercase())
        .collect()
}

fn hash(bytes: &[u8]) -> u64 {
    let mut value = 0xcbf29ce484222325u64;
    for byte in bytes {
        value ^= *byte as u64;
        value = value.wrapping_mul(0x100000001b3);
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corpus() -> Eidos {
        let mut eidos = Eidos::default();
        eidos.insert(Document {
            id: "a".into(),
            text: "Deterministic receipts can be replayed.".into(),
        });
        eidos.insert(Document {
            id: "b".into(),
            text: "Closed citations point into admitted local text.".into(),
        });
        eidos
    }

    #[test]
    fn retrieval_is_deterministic_and_closed() {
        let eidos = corpus();
        let first = eidos.retrieve("deterministic replay", 2);
        assert_eq!(first, eidos.retrieve("deterministic replay", 2));
        assert!(eidos.validate(&first).is_ok());
    }

    #[test]
    fn fabricated_quote_is_rejected() {
        let eidos = corpus();
        let mut packet = eidos.retrieve("closed citations", 1);
        packet.citations[0].quote.push_str(" invented");
        assert!(eidos.validate(&packet).is_err());
    }

    #[test]
    fn corpus_mutation_invalidates_old_packet() {
        let mut eidos = corpus();
        let packet = eidos.retrieve("receipts", 1);
        eidos.insert(Document {
            id: "c".into(),
            text: "new text".into(),
        });
        assert!(eidos.validate(&packet).is_err());
    }
}
