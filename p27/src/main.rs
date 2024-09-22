use std::collections::BTreeSet;

fn f(n: i32, a: i32, b: i32) -> i32 {
    n * n + a * n + b
}

fn evaluate(a: i32, b: i32, primes: &BTreeSet<u128>) -> usize {
    let mut score = 0;
    for n in 0..100 {
        let v = f(n as i32, a, b);
        if v < 0 {
            break;
        }

        if !primes.contains(&(v as u128)) {
            break;
        }

        score += 1;
    }

    score
}

fn main() {
    let mut best_score = 0;
    let mut answer = 0;
    let primes = utils::get_primes_until_n(1000);

    for a in -999..1000 {
        if a % 2 == 0 {
            continue;
        }

        for b in -999..1000 {
            let score = evaluate(a, b, &primes);
            if score > best_score {
                answer = a * b;
                best_score = score;
            }
        }
    }

    println!("ANSWER P27: {answer}");
}
