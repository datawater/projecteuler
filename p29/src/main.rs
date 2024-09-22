use std::collections::{BTreeMap, HashSet};
use utils::prime_factors_of;

fn main() {
    let mut h = HashSet::new();

    for a in 2..=100 {
        let prime_factors = &prime_factors_of(a as u128);

        for b in 2..=100 {
            let mut factor_count = BTreeMap::new();
            for &factor in prime_factors {
                *factor_count.entry(factor as u16).or_insert(0) += b;
            }

            h.insert(factor_count);
        }
    }

    println!("ANSWER P29: {}", h.len());
}
