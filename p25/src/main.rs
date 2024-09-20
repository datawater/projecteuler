fn number_of_digits_in_nth_fib_number(n: u64) -> u64 {
    let log10_phi = 0.20898764024997;
    let log10_fn = (n as f64) * log10_phi - 0.25;

    f64::ceil(log10_fn) as u64
}

fn main() {
    let mut i = 1;
    while number_of_digits_in_nth_fib_number(i) != 1000 {
        i += 1;
    }

    println!("ANSWER P25: {i}");
}
