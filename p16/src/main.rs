use num::{BigUint, Zero};

fn digit_sum(mut n: BigUint) -> BigUint {
    let mut sum = Zero::zero();
    let ten = BigUint::from(10u32);

    while !Zero::is_zero(&n) {
        let k = n.clone() % ten.clone();

        sum += k.clone();

        n = (n.clone() - k.clone()) / ten.clone();
    }

    sum
}

fn main() {
    println!("ANSWER P16: {}", digit_sum(BigUint::from(2u32).pow(1000)));
}
