use utils::*;

fn main() {
    const N: u128 = 600851475143;

    println!("ANSWER P3: {}", prime_factors_of(N).last().unwrap());
}
