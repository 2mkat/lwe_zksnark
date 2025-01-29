use lambdaworks_math::field::{
    fields::fft_friendly::
        {babybear::Babybear31PrimeField,
        u64_goldilocks::U64GoldilocksPrimeField},
    element::FieldElement,
};

use rand::Rng;

//Babybear Prime p = 2^31 - 2^27 + 1 = 0x78000001 = 2013265921
// for encode message space

pub type Fp = Babybear31PrimeField;
pub type FEp = FieldElement::<Fp>;

//Babybear Prime p = 2^64 - 2^32 + 1 = 18446744069414584321
// for message space

pub type F = U64GoldilocksPrimeField;
pub type FE = FieldElement::<F>;

pub fn sample_fr_elem_zp() -> FEp  {
    let mut rng = rand::thread_rng();
    // let random_u64: u64 = rng.gen();
    let random_u64: u64 = rng.gen_range(1..=1000000);

    FEp::from(random_u64)
}

pub fn sample_fr_elem_zq() -> FE {
    let mut rng = rand::thread_rng();
    // let random_u64: u64 = rng.gen();
    let random_u64: u64 = rng.gen_range(1..=1000000);

    FE::from(random_u64)
}