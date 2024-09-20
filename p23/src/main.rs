use std::collections::HashSet;

use utils::*;

fn main() {
    let divisors_sum = (1..=28123)
        .map(|x| {
            (
                x,
                generate_proper_divisors(x)
                    .into_iter()
                    .collect::<Vec<u128>>(),
            )
        })
        .filter(|x| x.1.iter().sum::<u128>() > x.0)
        .map(|x| x.0)
        .collect::<Vec<u128>>();

    let mut x = (1..=28123).collect::<HashSet<u128>>();

    for i in 0..divisors_sum.len() {
        for j in i..divisors_sum.len() {
            let sum = divisors_sum[i] + divisors_sum[j];
            if sum > 28123 {
                break;
            }

            x.remove(&sum);
        }
    }

    println!("ANSWER P23: {}", x.iter().sum::<u128>());
}
