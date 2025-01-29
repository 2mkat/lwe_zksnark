use crate::{common::*, sap::SquareArithmeticProgram as SAP,  lwe::*};

pub struct CommonReferenceString {
    pub sap: SAP,
    pub pk: PK,
    pub delta_t_s_2: FE,
    pub beta_t_sk: FE,
    pub delta_si: Vec<FE>,
    pub delta_si_t_sk: Vec<FE>,
    pub delta_wi_beta_vi: Vec<FE>,
}

pub fn setup(sap: &SAP) -> (Vec<FE>, CommonReferenceString, Vec<FEp>) {
    let delta = sample_fr_elem_zp();
    let beta= sample_fr_elem_zp();
    let s = sample_fr_elem_zp();

    let td = vec![beta.clone(), delta.clone(), s.clone()];

    let pk =  PK{n: 5, p: 7, q: 218, alfa: 0.29};
    let lwe: LWE = LWE::new(5, 2013265921, 18446744069414584321, 0.000000000000001);
    let sk = lwe.key_gen();

    let vrs = sk.clone();

    println!("s = {}, \ntarget(s) = {}", s.clone(), FEp::from(sap.target.evaluate(&s)));
    // let _:() = sap.target.evaluate(&s);

    let t_s = FEp::from(sap.target.evaluate(&s));

    let temp: FEp = delta.clone() * t_s.clone() * t_s.clone();
    let delta_t_s_2 = lwe.encode(temp, &sk);

    let beta_t_sk  = lwe.encode(beta.clone() * t_s.clone(), &sk);


    let mut delta_si: Vec<FE> = Vec::with_capacity(sap.target.degree() + 1);
    for i in 0..sap.target.degree() {
        delta_si.push(lwe.encode(delta.clone() * pow(&s, i), &sk).1);
    }

    let mut delta_si_t_sk: Vec<FE> = Vec::with_capacity(sap.target.degree());
    for i in 0..sap.target.degree() - 1 {
        delta_si_t_sk.push(lwe.encode(delta.clone() * pow(&s, i) * t_s.clone(), &sk).1);
    }

    let delta_wi_beta_vi: Vec<FE> = Vec::with_capacity(sap.num_r1cs_witness_variables + 1);
    for i in (sap.num_instance_variables - 1)..sap.target.degree(){
        let temp: FEp = delta.clone() * sap.w_polynomials[i].evaluate(&s) + beta.clone() * sap.u_polynomials[i].evaluate(&s);
        delta_si_t_sk.push(lwe.encode(temp, &sk).1);
    }

    let crs = CommonReferenceString{
        sap: sap.clone(), 
        pk: pk, 
        delta_t_s_2: delta_t_s_2.1,
        beta_t_sk: beta_t_sk.1,
        delta_si: delta_si,
        delta_si_t_sk: delta_si_t_sk,
        delta_wi_beta_vi: delta_wi_beta_vi,
    };

    (vrs, crs, td)
}

pub fn pow(s: &FEp, deg: usize) -> FEp {
    let mut res: FEp = FEp::from(1);

    if deg == 0 {
        return res;
    }

    for i in 1..=deg {
        res *= s;
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::test_ex::{new_test_first_constraint, new_test_r1cs, new_test_second_constraint};

    use super::*;
   
    #[test]
    fn test_setup() {
        let _constraints = vec![new_test_first_constraint(), new_test_second_constraint()];
        let r1cs = new_test_r1cs();

        let sap = SAP::r1cs_to_sap(r1cs.clone());

        setup(&sap);
    }
}