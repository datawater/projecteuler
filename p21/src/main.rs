use std::collections::HashMap;
use utils::generate_proper_divisors;

fn d(n: u128) -> u128 {
    generate_proper_divisors(n).iter().sum()
}

fn main() {
    let mut sum = 0;
    let mut lookup: HashMap<u128, u128> = HashMap::new();

    for i in 1..10_000 {
        let d_n = *lookup.entry(i).or_insert_with(|| d(i));
        let d_n_n = *lookup.entry(d_n).or_insert_with(|| d(d_n));

        if i == d_n_n && i < d_n {
            sum += d_n + d_n_n;
        }
    }

    println!("ANSWER P21: {sum}");
}
