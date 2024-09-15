fn main() {
    println!(
        "ANSWER P6: {}",
        (1..=100).sum::<u64>().pow(2) - (1..=100).map(|x| x * x).sum::<u64>()
    );
}
