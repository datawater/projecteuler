fn tau(n: u64) -> u64 {
    let mut cnt = 0;

    let end = (n as f64).sqrt() as u64;

    for i in 1..end {
        if n % i == 0 {
            cnt += 2;
        }
    }

    if end * end == n {
        cnt += 1;
    }

    cnt
}

fn nth_trig(n: u64) -> u64 {
    (n * (n - 1)) / 2
}

fn main() {
    let mut n = 1;

    loop {
        if tau(nth_trig(n)) >= 500 {
            break;
        }

        n += 1;
    }

    println!("ANSWER P12: {}", nth_trig(n));
}
