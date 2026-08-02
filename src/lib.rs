//! Binary16 conversion and proof-by-witness ULP accounting.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Witness {
    pub reference: u16,
    pub candidate: u16,
    pub ulps: u32,
}

impl Witness {
    pub fn new(reference: u16, candidate: u16) -> Self {
        Self {
            reference,
            candidate,
            ulps: ulp_distance(reference, candidate),
        }
    }
    pub fn within(self, tolerance: u32) -> bool {
        self.ulps <= tolerance
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Report {
    pub tolerance: u32,
    pub witnesses: Vec<Witness>,
}

impl Report {
    pub fn run(tolerance: u32, pairs: &[(u16, u16)]) -> Self {
        Self {
            tolerance,
            witnesses: pairs
                .iter()
                .map(|(reference, candidate)| Witness::new(*reference, *candidate))
                .collect(),
        }
    }
    pub fn accepted(&self) -> bool {
        self.witnesses
            .iter()
            .all(|witness| witness.within(self.tolerance))
    }
    pub fn worst_case(&self) -> Option<Witness> {
        self.witnesses
            .iter()
            .copied()
            .max_by_key(|witness| witness.ulps)
    }
}

fn ordered(bits: u16) -> u16 {
    if bits & 0x8000 != 0 {
        !bits
    } else {
        bits ^ 0x8000
    }
}

pub fn ulp_distance(left: u16, right: u16) -> u32 {
    ordered(left).abs_diff(ordered(right)) as u32
}

pub fn f16_to_f32(bits: u16) -> f32 {
    let sign = ((bits & 0x8000) as u32) << 16;
    let exponent = ((bits >> 10) & 0x1f) as u32;
    let fraction = (bits & 0x03ff) as u32;
    let out = match exponent {
        0 if fraction == 0 => sign,
        0 => {
            let mut frac = fraction;
            let mut shift = 0u32;
            while frac & 0x0400 == 0 {
                frac <<= 1;
                shift += 1;
            }
            let exp = 113u32 - shift;
            sign | (exp << 23) | ((frac & 0x03ff) << 13)
        }
        0x1f => sign | 0x7f80_0000 | (fraction << 13),
        _ => sign | ((exponent + 112) << 23) | (fraction << 13),
    };
    f32::from_bits(out)
}

pub fn f32_to_f16(value: f32) -> u16 {
    let bits = value.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exponent = ((bits >> 23) & 0xff) as i32;
    let fraction = bits & 0x7f_ffff;
    if exponent == 0xff {
        return sign | 0x7c00 | if fraction == 0 { 0 } else { 0x0200 };
    }
    let half_exp = exponent - 127 + 15;
    if half_exp >= 0x1f {
        return sign | 0x7c00;
    }
    if half_exp <= 0 {
        if half_exp < -10 {
            return sign;
        }
        let mantissa = fraction | 0x80_0000;
        let shift = (14 - half_exp) as u32;
        let mut half = (mantissa >> shift) as u16;
        let round_bit = 1u32 << (shift - 1);
        if mantissa & round_bit != 0 && (mantissa & (round_bit - 1) != 0 || half & 1 != 0) {
            half += 1;
        }
        return sign | half;
    }
    let mut half = sign | ((half_exp as u16) << 10) | (fraction >> 13) as u16;
    let remainder = fraction & 0x1fff;
    if remainder > 0x1000 || (remainder == 0x1000 && half & 1 != 0) {
        half = half.wrapping_add(1);
    }
    half
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_binary16_values_decode() {
        assert_eq!(f16_to_f32(0x3c00), 1.0);
        assert_eq!(f16_to_f32(0xc000), -2.0);
        assert!(f16_to_f32(0x7c00).is_infinite());
    }

    #[test]
    fn common_values_round_trip() {
        for bits in [0x0000, 0x8000, 0x3c00, 0xc000, 0x3555, 0x7bff] {
            assert_eq!(f32_to_f16(f16_to_f32(bits)), bits);
        }
    }

    #[test]
    fn adjacent_values_are_one_ulp_apart() {
        assert_eq!(ulp_distance(0x3c00, 0x3c01), 1);
    }

    #[test]
    fn signed_order_crosses_zero_cleanly() {
        assert_eq!(ulp_distance(0x8000, 0x0000), 1);
    }

    #[test]
    fn report_enforces_shipping_bar() {
        let report = Report::run(2, &[(0x3c00, 0x3c01), (0x4000, 0x4002)]);
        assert!(report.accepted());
        assert_eq!(report.worst_case().unwrap().ulps, 2);
    }
}
