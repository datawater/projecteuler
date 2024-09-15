use utils::get_primes_until_n;

fn main() {
    println!(
        "ANSWER P10: {}",
        get_primes_until_n(2_000_000).iter().sum::<u128>()
    );
}
