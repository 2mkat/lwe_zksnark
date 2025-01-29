use crate::common::*;
use rand::Rng;


pub struct PK {
    pub n: usize,
    pub p: u64,
    pub q: u64,
    pub alfa: f64,
}

pub struct LWE {
    pk: PK,
    std_: f64,
}

impl LWE {
    pub fn new(n: usize, p: u64, q: u64, alfa: f64) -> LWE {
        // TODO:
        // - add check gcd(p, q) = 1
        let pk = PK {n, p, q, alfa};
        let std_ = q as f64 * alfa;

        LWE{pk, std_}
    }

    pub fn key_gen(&self) ->  Vec<FE> {
        let s: Vec<FE> = (0..self.pk.n).map(|_| sample_fr_elem_zq()).collect();

        s
    }

    pub fn encode(&self, m: FEp, s: &Vec<FE>) -> (Vec<FE>, FE)  {
        let a: Vec<FE> = (0..self.pk.n).map(|_| sample_fr_elem_zq()).collect();

        // println!("\na vector:\n");
        for i in &a {
            let hex_ = i.clone().representative().to_hex();
            let _z = u64::from_str_radix(&hex_, 16).unwrap();
            // println!("{}", z);
            // println!("s = {}", i);
        }

        let e = discrete_gaussian(self.std_);
        // println!("\nGenerate error vector from normal distribution: e = {}", u64::from_str_radix(&e.clone().representative().to_hex(), 16).unwrap());

        let temp =  inner_product(&a, &s);
        // println!("\na*s = {}", u64::from_str_radix(&temp.clone().representative().to_hex(), 16).unwrap());

        let temp1 =  temp + FE::from(self.pk.p) * e;
        // println!("\na*s + p*e = {}", u64::from_str_radix(&temp1.clone().representative().to_hex(), 16).unwrap());

        let temp2 =  temp1 + FE::from_hex_unchecked(&m.representative().to_hex());
        // println!("\na*s + p*e + m = {}", u64::from_str_radix(&temp2.clone().representative().to_hex(), 16).unwrap());

        let mut neg_c0: Vec<FE> = a.clone();
        for i in 0..a.len() {
            neg_c0[i] = -a[i].clone();
        }

        // (c0, c1)
        (neg_c0, temp2)
    }

    pub fn decode(&self, s: &Vec<FE>, (c0, c1): (&Vec<FE>, &FE)) -> FEp {

        let temp =  inner_product(&c0, &s);
        println!("\n-a*s = {}", u64::from_str_radix(&temp.clone().representative().to_hex(), 16).unwrap());
        
        let c0_u64 = u64::from_str_radix(&c1.clone().representative().to_hex(), 16).unwrap();
        let temp_u64 = u64::from_str_radix(&temp.clone().representative().to_hex(), 16).unwrap();
        let m =   c0_u64 - temp_u64;
        // println!("\nc1 - a * s  = {}", m);

        FEp::from(m)
    }

}

pub fn inner_product(v1: &Vec<FE>, v2: &Vec<FE>) -> FE {
    v1.iter()
        .zip(v2)
        .map(|(x, y)| x * y)
        .fold(FE::from(0), |x, y| x + y)
}

fn discrete_gaussian(std_: f64) -> FE {
    let mut rng = rand::thread_rng();
    let e: FE = FE::from((rng.gen::<f64>() * std_).round() as u64);

    e
}