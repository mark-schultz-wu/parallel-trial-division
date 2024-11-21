use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct PartialFactorization {
    number: u128,
    factors: HashMap<u128, u16>,
}

#[derive(Debug, Clone, Default)]
pub struct Factorization {
    number: u128,
    factors: HashMap<u128, u16>,
}

impl From<PartialFactorization> for Factorization {
    fn from(value: PartialFactorization) -> Self {
        Factorization {
            number: value.number,
            factors: value.factors,
        }
    }
}

impl PartialFactorization {
    pub fn new(number: u128) -> Self {
        let factors = HashMap::new();
        PartialFactorization { number, factors }
    }
    pub fn increment(&mut self, key: u128) {
        *self.factors.entry(key).or_insert(0) += 1;
    }
}

impl Factorization {
    pub fn is_correct(&self) -> bool {
        let mut int: u128 = 1;
        for (&factor, &exponent) in self.factors.iter() {
            let exp_factor = factor.pow(exponent.into());
            int *= exp_factor;
        }
        int == self.number
    }
}

#[inline]
pub fn serial_factorization(int: u128) -> Factorization {
    //
    let mut n = int;
    let sqrt_n: u64 = 1 << ((1 + n.ilog2()) >> 1);
    let mut factors = PartialFactorization::new(n);
    while n % 2 == 0 {
        factors.increment(2);
        n /= 2;
    }

    for i in (3..sqrt_n).step_by(2) {
        let i: u128 = i.into();
        while n % i == 0 {
            factors.increment(i);
            n /= i;
        }
        if n == 1 {
            break;
        }
    }
    // the remaining amount must be prime
    factors.increment(n);
    factors.into()
}

#[inline]
pub fn parallel_factorization(int: u128) -> Factorization {
    //
    let mut n = int;
    let mut factors = PartialFactorization::new(n);
    while n % 2 == 0 {
        factors.increment(2);
        n /= 2;
    }
    let sqrt_n: u64 = 1 << ((1 + n.ilog2()) >> 1);
    const M13: u128 = (1 << 13) - 1;
    let possible_factors: Vec<u64> = (3..sqrt_n)
        .into_iter()
        .filter(|&i| i % 2 == 1)
        .filter(|&i| n % (i as u128) == 0)
        .collect();
    for possible_factor in possible_factors.into_iter() {
        let pf: u128 = possible_factor.into();
        while n % pf == 0 {
            n /= pf;
            factors.increment(pf);
        }
    }
    factors.increment(n);
    factors.into()
}

#[cfg(test)]
mod tests {
    use crate::{parallel_factorization, serial_factorization};

    const M13: u128 = (1 << 13) - 1;
    const M17: u128 = (1 << 17) - 1;
    const M19: u128 = (1 << 19) - 1;
    const M31: u128 = (1 << 31) - 1;

    const N0: (u128, u128, u128) = (M13, M17, M13 * M17);
    const N1: (u128, u128, u128) = (M13, M19, M13 * M19);
    const N2: (u128, u128, u128) = (M13, M31, M13 * M31);
    const N3: (u128, u128, u128) = (M17, M19, M17 * M19);
    const N4: (u128, u128, u128) = (M17, M31, M17 * M31);
    const N5: (u128, u128, u128) = (M19, M31, M19 * M31);
    const N6: (u128, u128, u128) = (M13, M13, M13 * M13);

    fn test_serial(N: (u128, u128, u128)) {
        let result = serial_factorization(N.2);
        assert!(result.is_correct());
        assert_eq!(result.number, N.2);
        assert_eq!(result.factors[&N.0], 1);
        assert_eq!(result.factors[&N.1], 1);
    }

    #[test]
    fn test_m13_m17_serial() {
        test_serial(N0);
    }
    #[test]
    fn test_m13_m19_serial() {
        test_serial(N1);
    }
    #[test]
    fn test_m13_m31_serial() {
        test_serial(N2);
    }
    #[test]
    fn test_m17_m19_serial() {
        test_serial(N3);
    }
    #[test]
    fn test_m17_m31_serial() {
        test_serial(N4);
    }
    #[test]
    fn test_m19_m31_serial() {
        test_serial(N5);
    }

    #[test]
    fn test_m13_m13_serial() {
        let N = N6;
        let result = serial_factorization(N.2);
        assert!(result.is_correct());
        assert_eq!(result.number, N.2);
        assert_eq!(result.factors[&N.0], 2);
    }

    fn test_parallel(N: (u128, u128, u128)) {
        let result = parallel_factorization(N.2);
        assert!(result.is_correct());
        assert_eq!(result.number, N.2);
        assert_eq!(dbg!(result.factors)[&N.0], 1);
        // assert_eq!(result.factors[&N.1], 1);
    }

    #[test]
    fn test_m13_m17_parallel() {
        test_parallel(N0);
    }
    #[test]
    fn test_m13_m19_parallel() {
        test_parallel(N1);
    }
    #[test]
    fn test_m13_m31_parallel() {
        test_parallel(N2);
    }
    #[test]
    fn test_m17_m19_parallel() {
        test_parallel(N3);
    }
    #[test]
    fn test_m17_m31_parallel() {
        test_parallel(N4);
    }
    #[test]
    fn test_m19_m31_parallel() {
        test_parallel(N5);
    }

    #[test]
    fn test_m13_m13_parallel() {
        let N = N6;
        let result = parallel_factorization(N.2);
        assert!(result.is_correct());
        assert_eq!(result.number, N.2);
        assert_eq!(result.factors[&N.0], 2);
    }
}
