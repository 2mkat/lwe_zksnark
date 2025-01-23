use crate::{common::*, lwe::*, sap::SquareArithmeticProgram as SAP, setup::CommonReferenceString};
use lambdaworks_math::polynomial::Polynomial;

pub struct Proof {
    pub v: FE,
}

pub fn prove(crs: CommonReferenceString, u: Vec<FEp>, w: Vec<FEp>) -> (FE, FE) {
    let full_instance: Vec<FEp> = vec![u.clone(), w.clone()].concat();

    let u_x: Polynomial<FEp> = crs.sap.u_polynomials[0].clone()
            + crs.sap.u_polynomials[1..]
                .iter()
                .zip(full_instance.clone())
                .map(|(v, c)| v.mul_with_ref(&Polynomial::new_monomial(c.clone(), 0)))
                .reduce(|x, y| x + y)
                .unwrap();

    let w_x: Polynomial<FEp> = crs.sap.w_polynomials[0].clone()
        + crs.sap.w_polynomials[1..]
            .iter()
            .zip(full_instance.clone())
            .map(|(v, c)| v.mul_with_ref(&Polynomial::new_monomial(c.clone(), 0)))
            .reduce(|x, y| x + y)
            .unwrap();

    let mid = w.len();
    let u_x_mid: Polynomial<FEp> = crs.sap.u_polynomials[mid..]
                .iter()
                .zip(u.clone())
                .map(|(v, c)| v.mul_with_ref(&Polynomial::new_monomial(c.clone(), 0)))
                .reduce(|x, y| x + y)
                .unwrap();

    let w_x_mid: Polynomial<FEp> = crs.sap.w_polynomials[mid..]
            .iter()
            .zip(u.clone())
            .map(|(v, c)| v.mul_with_ref(&Polynomial::new_monomial(c.clone(), 0)))
            .reduce(|x, y| x + y)
            .unwrap();
    
    let h_polinomial_from_sap = crs.sap.h_polinomial(&full_instance);

    // calculate f(w)
    let mut f_w: FE = FE::from(0);
    for i in 0..w.clone().len() {
        f_w += FE::from_hex_unchecked(&w[i].representative().to_hex()) * crs.delta_wi_beta_vi[i]
    }
    

    // calculate g(r)
    let r = FE::from_hex_unchecked(&sample_fr_elem_zp().representative().to_hex());
    let mut u_x_coeff = u_x.coefficients();
    let mut part_twp_in_g_w: FE = FE::from(0);

    for i in 0..u_x_coeff.len() {
        part_twp_in_g_w += crs.delta_si_t_sk[i] * FE::from_hex_unchecked(&u_x_coeff[i].representative().to_hex());
    }
    
    let g_r = (r.clone() * r.clone() * crs.delta_t_s_2) + 
    FE::from(2) * r.clone() * part_twp_in_g_w + r * crs.beta_t_sk;


    // calculate A proof
    let mut delta_v_s: FE = FE::from(0);
    for i in 0..u_x_coeff.len() {
        delta_v_s += crs.delta_si[i] * FE::from_hex_unchecked(&u_x_coeff[i].representative().to_hex());
    }

    let mut t_x_coeff = crs.sap.target.coefficients();
    let mut delta_t_s: FE = FE::from(0);
    for i in 0..t_x_coeff.len() {
        delta_t_s += crs.delta_si[i] * FE::from_hex_unchecked(&t_x_coeff[i].representative().to_hex());
    }

    let a_proof = delta_t_s + delta_v_s;

    // calculate B proof
    let mut h_x_coeff = h_polinomial_from_sap.coefficients();
    let mut delta_t_s_h_s: FE = FE::from(0);
    for i in 0..h_x_coeff.len() {
        delta_t_s_h_s += crs.delta_si[i] * FE::from_hex_unchecked(&h_x_coeff[i].representative().to_hex());
    }

    let b_proof = f_w + g_r + delta_t_s_h_s;



    (a_proof, b_proof)
}