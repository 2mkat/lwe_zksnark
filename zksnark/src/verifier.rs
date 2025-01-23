use crate::{common::*, setup::CommonReferenceString};
use lambdaworks_math::polynomial::Polynomial;


pub fn verify(a_proof: FE, b_proof: FE,  u: Vec<FEp>, crs:CommonReferenceString, beta: FEp, delta: FEp) -> bool {
    let mid = u.len();
    let u_x: Polynomial<FEp> = crs.sap.u_polynomials[0].clone()
    + crs.sap.u_polynomials[1..mid]
        .iter()
        .zip(u.clone())
        .map(|(v, c)| v.mul_with_ref(&Polynomial::new_monomial(c.clone(), 0)))
        .reduce(|x, y| x + y)
        .unwrap();

    let w_x: Polynomial<FEp> = crs.sap.w_polynomials[0].clone()
    + crs.sap.w_polynomials[1..mid]
        .iter()
        .zip(u.clone())
        .map(|(v, c)| v.mul_with_ref(&Polynomial::new_monomial(c.clone(), 0)))
        .reduce(|x, y| x + y)
        .unwrap();

    let mut w_plus_u = FE::from(0);

    for i in 0..u.len() {
        w_plus_u += FE::from_hex_unchecked(&u[i].representative().to_hex()) * crs.delta_wi_beta_vi[i]
    }

    // check
    if ( a_proof.clone() * (a_proof + FE::from_hex_unchecked(&beta.representative().to_hex())) ==
    FE::from_hex_unchecked(&delta.representative().to_hex()) * (b_proof + w_plus_u)) {
        return true
    }

    false
}