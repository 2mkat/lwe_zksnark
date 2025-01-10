use crate::{common::*, lwe::*, sap::SquareArithmeticProgram as SAP, setup::CommonReferenceString};
use lambdaworks_math::polynomial::Polynomial;

pub struct Proof {
    pub v: FE,
}

pub fn prove(crs: CommonReferenceString, u: Vec<FEp>, w: Vec<FEp>) -> Proof {
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
    
    crs.sap.h_polinomial(&full_instance);

    let r = sample_fr_elem_zp();

    // calculate f(w)


    // calculate g(r)
    // let f_w = (r.clone() * r.clone() * crs.delta_t_s_2) + 
    // 2 * r * crs.delta_si_t_sk[0] * u_x.evaluate(r) + r * crs.beta_t_sk;



    Proof{v: FE::from(0)}
}