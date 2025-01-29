use crate::{common::*, 
    r1cs::{Constraint, R1CS}, 
    lwe::*};
use lambdaworks_math::polynomial::Polynomial;
use std::convert::From;

pub fn new_test_r1cs() -> R1CS {
    let constraints = vec![new_test_first_constraint(), new_test_second_constraint()];
    R1CS::new(constraints, 4, 1).unwrap()
}

pub fn new_test_first_constraint() -> Constraint {
    Constraint {
        a: vec![
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(1),
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
        ],
        b: vec![
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(1),
            FEp::from(0),
            FEp::from(0),
        ],
        c: vec![
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(1),
            FEp::from(0),
        ],
    }
}

pub fn new_test_second_constraint() -> Constraint {
    Constraint {
        a: vec![
            FEp::from(0),
            FEp::from(1),
            FEp::from(1),
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
        ],
        b: vec![
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(1),
            FEp::from(0),
        ],
        c: vec![
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(1),
        ],
    }
}

pub fn gen_sap_constraints() {
    
    let m: u64 = 7; 
    let number_of_constraints: u64  = 4;

    // generate r_q
    let rs: Vec<FEp> = (0..number_of_constraints)
        .map(|i| FEp::new(i.into()))
        .collect();


    // Create t(x)
    let mut t_x: Polynomial<FEp> = Polynomial::new_monomial(FEp::from(1), 0);
    
    for r in &rs {
        t_x = t_x * Polynomial::new(&[-r, FEp::from(1)]);
    }

    // generate a_i
    // a_i[0] = 1
    let a: Vec<FEp> = (1..=m).map(|_| sample_fr_elem_zp()).collect();
    println!("len vector a_m = {}", a.len());

    // v_i(x)
    let mut v_x: Vec<Polynomial<FEp>> = Vec::new();
    for _ in 0..=m {
        let coeffs: Vec<FEp> = (0..number_of_constraints)
            .map(|_| sample_fr_elem_zp()) 
            .collect();
        v_x.push(Polynomial::new(&coeffs));
    }

    // calculate p_polinomial v^2 - w = p 
    let v: Polynomial<FEp> = v_x[0].clone()
            + v_x[1..]
                .iter()
                .zip(a)
                .map(|(v, c)| v.mul_with_ref(&Polynomial::new_monomial(c.clone(), 0)))
                .reduce(|x, y| x + y)
                .unwrap();

    // w_i(x) = v_i(x)^2 mod t(x)
    let w: Polynomial<FEp> = (&v * &v).div_with_ref(&t_x);
    // w_i(x)


    // calculate h(x)
    let h: Polynomial<FEp> = ((&v * &v) - &w).div_with_ref(&t_x);

    println!("deg(v(x)) = {}", v.degree());
    println!("deg(w(x)) = {}", w.degree());
    println!("deg(t(x)) = {}", t_x.degree());
    println!("deg(h(x)) = {}", h.degree());

    // check condition
    if (&v * &v) == (&w + &h * &t_x) {
        println!("Correct SAP");
    } else {
        println!("Incorrect SAP"); 
    }
}


pub fn check_lwe() {

    // let pk =  PK{n: 5, p: 7, q: 218, alfa: 0.29};
    let lwe: LWE = LWE::new(5, 2013265921, 18446744069414584321, 0.000000000000001);

    let s = lwe.key_gen();
    
    println!("secret vector:\n");
    for i in &s {
        let hex_ = i.clone().representative().to_hex();
        let z = u64::from_str_radix(&hex_, 16).unwrap();
        println!("{}", z);
        // println!("s = {}", i);
    }


    let m = sample_fr_elem_zp();
    let (c0, c1) = lwe.encode(m.clone(), &s);

    let mut c0_test = c0.clone();

    for i in 0..c0.len() {
        c0_test[i] = -(c0[i].clone());
    }

    let decode_m = lwe.decode(&s, (&c0_test, &c1));

    // let test: FE = FE::from(3);
    // let z = u64::from_str_radix(&test.representative().to_hex(), 16).unwrap();
    // println!("z = {}", test.representative().to_hex());

    println!("encode message = {}", c1);
    println!("m = {}", m);
    println!("decode m = {}", decode_m);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_gen() {
        gen_sap_constraints();
    }

    #[test]
    fn run_check_lwe() {
        check_lwe();
    }

}