fn collatz_chain_length(mut n: u64) -> u64 {
    let mut c = 0;

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = (3 * n + 1) / 2;
        }

        c += 1;
    }

    c
}

fn main() {
    let mut longest_chain: (u64, u64) = (0, 0);

    for x in 1..=1_000_000 {
        let l = collatz_chain_length(x);

        if l > longest_chain.0 {
            longest_chain = (l, x);
        }
    }

    println!("ANSWER P14: {}", longest_chain.1);
}
