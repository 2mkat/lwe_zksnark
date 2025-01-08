use crate::{common::*, sap::SquareArithmeticProgram as SAP,  lwe::*};

pub fn setup(sap: SAP) -> (crs, vrs, td) {
    let delta: F = sample_fr_elem_zq;
    let beta: F = sample_fr_elem_zq;
    let s: F = sample_fr_elem_zq;

    let pk =  PK{n: 5, p: 7, q: 218, alfa: 0.29};
    let lwe: LWE = LWE::new(5, 2013265921, 18446744069414584321, 0.000000000000001);
    sk = lwe.key_gen();

    let sample1 = lwe.encode(delta * sap.t_x.evaluate(sk), sk);

}