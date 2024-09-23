fn collatz_chain_length(mut n: u64, m: &mut Vec<u64>) -> u64 {
    let mut c = 0;

    while n != 1 {
        if (n as usize) < m.len() {
            c += m[(n - 1) as usize];
            break;
        }

        if n % 2 == 0 {
            n /= 2;
        } else {
            n = (3 * n + 1) / 2;
        }

        c += 1;
    }

    m.push(c);

    c
}

fn main() {
    let mut longest_chain: (u64, u64) = (0, 0);
    let mut hashmap = Vec::with_capacity(1_000_000);

    for x in 1..=1_000_000 {
        let l = collatz_chain_length(x, &mut hashmap);

        if l > longest_chain.0 {
            longest_chain = (l, x);
        }
    }

    println!("ANSWER P14: {}", longest_chain.1);
}
