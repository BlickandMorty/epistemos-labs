//! A compact WBO register with explicit terms and fail-closed validation.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResidencyTier {
    Hot,
    Warm,
    Cold,
    Archive,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub id: String,
    pub tier: ResidencyTier,
    pub distortion: f64,
    pub rate: f64,
    pub side_information: f64,
    pub lambda: f64,
    pub mu: f64,
    pub budget: f64,
}

impl Entry {
    pub fn weighted_bound(&self) -> f64 {
        self.distortion + self.lambda * self.rate + self.mu * self.side_information
    }

    pub fn validate(&self) -> Result<(), String> {
        let values = [
            self.distortion,
            self.rate,
            self.side_information,
            self.lambda,
            self.mu,
            self.budget,
        ];
        if values
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err("all accounting terms must be finite and non-negative".into());
        }
        if self.weighted_bound() > self.budget {
            return Err(format!(
                "{} exceeds budget by {:.6}",
                self.id,
                self.weighted_bound() - self.budget
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Ledger {
    entries: Vec<Entry>,
}

impl Ledger {
    pub fn admit(&mut self, entry: Entry) -> Result<(), String> {
        entry.validate()?;
        if self.entries.iter().any(|item| item.id == entry.id) {
            return Err("duplicate ledger id".into());
        }
        self.entries.push(entry);
        self.entries.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(())
    }
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
    pub fn total_bound(&self) -> f64 {
        self.entries.iter().map(Entry::weighted_bound).sum()
    }
    pub fn tier_bound(&self, tier: ResidencyTier) -> f64 {
        self.entries
            .iter()
            .filter(|entry| entry.tier == tier)
            .map(Entry::weighted_bound)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: &str, budget: f64) -> Entry {
        Entry {
            id: id.into(),
            tier: ResidencyTier::Warm,
            distortion: 0.1,
            rate: 2.0,
            side_information: 0.5,
            lambda: 0.2,
            mu: 0.4,
            budget,
        }
    }

    #[test]
    fn bound_is_fully_accounted() {
        assert!((entry("x", 1.0).weighted_bound() - 0.7).abs() < 1e-12);
    }

    #[test]
    fn budget_violation_is_rejected() {
        assert!(entry("x", 0.6).validate().is_err());
    }

    #[test]
    fn duplicate_ids_are_rejected() {
        let mut ledger = Ledger::default();
        ledger.admit(entry("x", 1.0)).unwrap();
        assert!(ledger.admit(entry("x", 1.0)).is_err());
    }

    #[test]
    fn order_does_not_change_totals() {
        let mut a = Ledger::default();
        a.admit(entry("b", 1.0)).unwrap();
        a.admit(entry("a", 1.0)).unwrap();
        let mut b = Ledger::default();
        b.admit(entry("a", 1.0)).unwrap();
        b.admit(entry("b", 1.0)).unwrap();
        assert_eq!(a.entries(), b.entries());
        assert_eq!(a.total_bound(), b.total_bound());
    }
}
