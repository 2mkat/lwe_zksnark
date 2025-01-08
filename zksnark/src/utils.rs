// use qfall_math::integer_mod_q::ModulusPolynomialRingZq;
use qfall_math::{
    error::MathError,
    integer_mod_q::{Modulus, ModulusPolynomialRingZq, PolyOverZq, PolynomialRingZq, Zq},
    traits::SetCoefficient,
};
use std::fmt::Display;

pub fn new_anticyclic(
    n: impl TryInto<i64> + Display,
    q: impl Into<Modulus>,
) -> Result<ModulusPolynomialRingZq, MathError> {
    let mut poly = PolyOverZq::from((1, q));
    poly.set_coeff(n, 1)?;
    Ok(ModulusPolynomialRingZq::from(&poly))
}

// создает моном вида X^degree
pub fn new_monomial(z: Zq, degre: u64) -> PolynomialRingZq {
    // let mut coefficients = vec![Z::ZERO; degree];
    // coefficients.push(coefficient);
    // Self::new(&coefficients)
    todo!()
}

pub fn new(z: Zq, degre: u64) -> PolynomialRingZq {
    todo!()
}

pub fn interpolate(xs: &Vec<Zq>, ys: &Vec<Zq>) -> Result<PolynomialRingZq, InterpolateError> {
    if xs.len() != ys.len() {
        return Err(InterpolateError::UnequalLengths(xs.len(), ys.len()));
    }
    if xs.is_empty() {
        return Ok(Z::ZERO);
    }

    let mut denominators = Vec::with_capacity(xs.len() * (xs.len() - 1) / 2);
    let mut indexes = Vec::with_capacity(xs.len());

    let mut idx = 0;

    for (i, xi) in xs.iter().enumerate().skip(1) {
        indexes.push(idx);
        for xj in xs.iter().take(i) {
            if xi == xj {
                return Err(InterpolateError::NonUniqueXs);
            }
            denominators.push(xi - xj);
            idx += 1;
        }
    }

    

    unimplemented!()
}

#[derive(Debug)]
pub enum InterpolateError {
    UnequalLengths(usize, usize),
    NonUniqueXs,
}

impl Display for InterpolateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InterpolateError::UnequalLengths(x, y) => {
                write!(f, "xs and ys must be the same length. Got: {x} != {y}")
            }
            InterpolateError::NonUniqueXs => write!(f, "xs values should be unique."),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InterpolateError {}

#[cfg(test)]
mod test_new_anticyclic {
    use crate::utils::new_anticyclic;
    use qfall_math::{integer::Z, integer_mod_q::PolyOverZq, traits::GetCoefficient};

    /// Ensure that the modulus polynomial has the specified degree.
    #[test]
    fn degree() {
        let degrees = [1, 4, 7, 16, 32, 128];
        for degree in degrees {
            let poly_mod = new_anticyclic(degree, 7).unwrap();

            assert_eq!(degree, poly_mod.get_degree());
        }
    }

    /// Check whether the method outputs the correct polynomial.
    #[test]
    fn correct_polynomial() {
        let degrees = [1, 4, 7, 16, 32, 128];
        for degree in degrees {
            let poly_mod = new_anticyclic(degree, 7).unwrap();
            let poly_zq = PolyOverZq::from(&poly_mod);

            assert_eq!(Z::ONE, poly_zq.get_coeff(degree).unwrap());
            assert_eq!(Z::ONE, poly_zq.get_coeff(0).unwrap());
            for i in 1..degree {
                assert_eq!(Z::ZERO, poly_zq.get_coeff(i).unwrap());
            }
        }
    }

    /// Ensures that the correct modulus is set as
    /// the integer modulus of the output modulus polynomial.
    #[test]
    fn correct_modulus() {
        let moduli = [7, 10, i64::MAX];
        for modulus in moduli {
            let poly_mod = new_anticyclic(2, modulus).unwrap();

            assert_eq!(Z::from(modulus), poly_mod.get_q());
        }
    }

    /// Ensures that an invalid degree for the modulus polynomial results in an error.
    #[test]
    fn invalid_n() {
        let res = new_anticyclic(-1, 7);

        assert!(res.is_err());
    }

    /// Ensures that an invalid modulus for the modulus polynomial results in a panic.
    #[test]
    #[should_panic]
    fn invalid_modulus() {
        let _ = new_anticyclic(2, 0);
    }
}