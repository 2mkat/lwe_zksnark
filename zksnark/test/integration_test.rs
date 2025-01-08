use rand::Rng;
use rlwe::RLWE;
use rq::Rq;
use r1cs::{Constraint, R1CS};
// use zksnark::test_ex::{new_test_r1cs, test_qap_solver};
use pinocchio::test_utils::{new_test_r1cs};

mod rlwe;
mod r1cs;
mod rq;

fn test_zksnark() {
    println!("Running zkSNARK test...");
    let test_qap = new_test_r1cs().into();
    // Setup

    let constraints = vec![
        Constraint {
            a: vec![1, 2],
            b: vec![3, 4],
            c: vec![5, 6],
        },
        Constraint {
            a: vec![7, 8],
            b: vec![9, 10],
            c: vec![11, 12],
        },
    ];

    for i in constraints.clone() {
        println!("{}", i);
    }

    // Число входов и выходов
    let number_of_inputs = 3;
    let number_of_outputs = 1;

    // Создаем R1CS
    let r1cs = R1CS::new(constraints.clone(), number_of_inputs, number_of_outputs);
    

    // инициализирует генератор случайных чисел и ассоциируется с текущим потоком
    let mut rng = rand::thread_rng();

    let n = 4; // power of 2
    // p, q = {17, 97, 113, 1139, 67108289}
    // q > p
    let q = 97; // prime number, q = 1 (mod 2n)
    let p = 17; // prime number p < q, p = 1 mod 2n
    let std_ = 3.14; // standard deviation of the gaussian distribution

    let t = 17; 
    
    let rlwe = RLWE::new(n, q, t, std_); // создаем структуру для семы кодирования
    let sk = rlwe.key_gen();    // генерируем секретный ключ
    println!("Generate secret key for RLWE Encoding Scheme from Rq: sk = {}", sk);

    // создаем полином из Rp, чтобы его закодировать
    // plaintexts is polinomials from Rp
    // Rp - message space
    // Rq - encoding space

    let m = {
        let mut coeffs = vec![0; n];
        for i in 0..n {
            // coeffs[i] = rng.gen_range(p..q); ????
            coeffs[i] = rng.gen_range(0..p);
        }
        Rq::new(coeffs, p)
    };
    println!("Plaintext from Rp: m = {}", m);


    // генерируем полином а from Rq
    //TODO: перенести этот блок кода в кодирование
    // подумать как туда передавать n, q
    let a = {
        let mut coeffs = vec![0; n];
        for i in 0..n {
            coeffs[i] = rng.gen_range(0..q);
        }
        Rq::new(coeffs, q)
    };
    println!("Polinom a from Rq: a = {}", a);


    // Вызываем функцию кодирования
    // let c0 = rlwe.encode(m, s); убрать передачу полинома а
    println!("\nENCODE\n");
    let b = rlwe.encode(m, sk.clone(), a.clone());

    println!("\nDECODE\n");
    rlwe.decode(sk, a, b);

    println!("\nEXAMPLE OF MUL\n");
    let a = {
        let coeffs =  vec![4, 1, 11, 10];
        Rq::new(coeffs, 13)
    };
    println!("Polinom a from Rq: a = {}", a);

    let s = {
        let coeffs =  vec![6, 9, 11, 11];
        Rq::new(coeffs, 13)
    };
    println!("Polinom a from Rq: s = {}", s);

    let res_mul = a * s;
    println!("Polinom a from Rq: a*s = {}\n", res_mul);


    let (sk, pk) = rlwe.generate_keys();
    let m0 = {
        let mut coeffs = vec![0; n];
        for i in 0..n {
            coeffs[i] = rng.gen_range(t..q);
        }
        Rq::new(coeffs, t)
    };
    let m1 = {
        let mut coeffs = vec![0; n];
        for i in 0..n {
            coeffs[i] = rng.gen_range(t..q);
        }
        Rq::new(coeffs, t)
    };

    let c0 = rlwe.encrypt(m0.clone(), pk.clone());
    let c1 = rlwe.encrypt(m1.clone(), pk.clone());

    let m_0 = rlwe.decrypt(vec![c0.0.clone(), c0.1.clone()], sk.clone());
    let m_1 = rlwe.decrypt(vec![c1.0.clone(), c1.1.clone()], sk.clone());

    println!("m0: {}", m0);
    println!("m_0: {}", m_0);

    println!("m1: {}", m1);
    println!("m_1: {}", m_1);

    // Addition
    println!("ADD");
    println!("m0 + m1: {}", m0.clone() + m1.clone());

    let c_add = rlwe.add(
        vec![c0.0.clone(), c0.1.clone()],
        vec![c1.0.clone(), c1.1.clone()],
    );
    let m_add = rlwe.decrypt(c_add, sk.clone());
    println!("m0 + m1: {}", m_add);

    // Multiplication
    println!("MUL");
    println!("m0 * m1: {}", m0 * m1);

    let c_mul = rlwe.mul(
        vec![c0.0.clone(), c0.1.clone()],
        vec![c1.0.clone(), c1.1.clone()],
    );
    let m_mul = rlwe.decrypt(c_mul, sk);
    println!("m0 * m1: {}", m_mul);
}

#[cfg(test)]
#[test]
fn test_pinocchio_random_circuit() {
    test_pinocchio();
}