use utils::*;

fn main() {
    println!("ANSWER P2: {}", get_fib_numbers_until(4_000_000).into_iter().filter(|n| n % 2 == 0).sum::<u128>());
}
