use num::{BigUint, One};

fn factorial(n: u128) -> BigUint {
    (1..=n).fold(BigUint::one(), |a, b| a * b)
}

// https://en.wikipedia.org/wiki/Lattice_path
#[allow(non_snake_case)]
fn nCr(n: u128, k: u128) -> BigUint {
    factorial(n) / (factorial(k) * factorial(n - k))
}

fn main() {
    println!("ANSWER P15: {}", nCr(40, 20));
}
