fn main() {
    println!(
        "ANSWER P1: {}",
        (0..1000)
            .filter(|x| { x % 3 == 0 || x % 5 == 0 })
            .sum::<u32>()
    );
}
