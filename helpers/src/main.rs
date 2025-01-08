/// Крейт для вычисления подходящи параметров 

fn main() {
    // генерируем простые числа и проверяем условие p, q = 1 mod 2N
    let mut primes = vec![2];
    let maximum: u64 = 110_000;

    for candidate in 30000..maximum {
        let square_root = (candidate as f64).sqrt() as u64 + 1;
        let is_prime = primes
            .iter()
            .take_while(|p| p <= &&square_root)
            .all(|p| candidate % p != 0);
        if is_prime && candidate % 128 == 1{
            primes.push(candidate);
        }
    }

    println!("{:?}", primes);
}