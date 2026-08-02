//! A dependency-free, testable floor for the EML and typed primitive IR work.

#[derive(Clone, Debug, PartialEq)]
pub enum EmlError {
    NonPositiveLogArgument(f64),
    NonFinite,
}

pub fn eml(x: f64, y: f64) -> Result<f64, EmlError> {
    if y <= 0.0 {
        return Err(EmlError::NonPositiveLogArgument(y));
    }
    if !x.is_finite() || !y.is_finite() {
        return Err(EmlError::NonFinite);
    }
    let value = x.exp() - y.ln();
    if value.is_finite() {
        Ok(value)
    } else {
        Err(EmlError::NonFinite)
    }
}

pub fn eml_inverse_x(z: f64, y: f64) -> Result<f64, EmlError> {
    if y <= 0.0 {
        return Err(EmlError::NonPositiveLogArgument(y));
    }
    let inner = z + y.ln();
    if inner <= 0.0 {
        return Err(EmlError::NonPositiveLogArgument(inner));
    }
    let value = inner.ln();
    if value.is_finite() {
        Ok(value)
    } else {
        Err(EmlError::NonFinite)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveKind {
    Eml,
    Geometry,
    Information,
    Operator,
    Scan,
    Tropical,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Constant(f64),
    Eml(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Min(Box<Expr>, Box<Expr>),
    PrefixSum(Vec<Expr>),
}

impl Expr {
    pub fn evaluate(&self) -> Result<f64, EmlError> {
        match self {
            Expr::Constant(value) if value.is_finite() => Ok(*value),
            Expr::Constant(_) => Err(EmlError::NonFinite),
            Expr::Eml(left, right) => eml(left.evaluate()?, right.evaluate()?),
            Expr::Add(left, right) => finite(left.evaluate()? + right.evaluate()?),
            Expr::Min(left, right) => finite(left.evaluate()?.min(right.evaluate()?)),
            Expr::PrefixSum(items) => items
                .iter()
                .try_fold(0.0, |sum, item| finite(sum + item.evaluate()?)),
        }
    }
}

fn finite(value: f64) -> Result<f64, EmlError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(EmlError::NonFinite)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Certificate {
    pub kind: PrimitiveKind,
    pub fixture: String,
    pub expected_bits: u64,
    pub observed_bits: u64,
}

impl Certificate {
    pub fn evaluate(
        kind: PrimitiveKind,
        fixture: impl Into<String>,
        expression: &Expr,
        expected: f64,
    ) -> Result<Self, EmlError> {
        Ok(Self {
            kind,
            fixture: fixture.into(),
            expected_bits: expected.to_bits(),
            observed_bits: expression.evaluate()?.to_bits(),
        })
    }
    pub fn verifies(&self) -> bool {
        self.expected_bits == self.observed_bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eml_base_case_is_one() {
        assert_eq!(eml(0.0, 1.0).unwrap(), 1.0);
    }

    #[test]
    fn eml_inverse_round_trips() {
        let x = 0.75;
        let y = 2.0;
        let z = eml(x, y).unwrap();
        assert!((eml_inverse_x(z, y).unwrap() - x).abs() < 1e-12);
    }

    #[test]
    fn certificate_requires_bit_exact_result() {
        let expr = Expr::PrefixSum(vec![Expr::Constant(1.0), Expr::Constant(2.0)]);
        let certificate =
            Certificate::evaluate(PrimitiveKind::Scan, "sum-1-2", &expr, 3.0).unwrap();
        assert!(certificate.verifies());
    }

    #[test]
    fn branch_cut_fails_closed() {
        let expr = Expr::Eml(Box::new(Expr::Constant(1.0)), Box::new(Expr::Constant(0.0)));
        assert!(matches!(
            expr.evaluate(),
            Err(EmlError::NonPositiveLogArgument(_))
        ));
    }

    #[test]
    fn tropical_min_is_explicit() {
        let expr = Expr::Min(
            Box::new(Expr::Constant(4.0)),
            Box::new(Expr::Constant(-2.0)),
        );
        assert_eq!(expr.evaluate().unwrap(), -2.0);
    }
}
