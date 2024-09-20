use num::integer::sqrt;
use std::collections::{HashSet, VecDeque};

pub fn nth_fib_number(n: u64) -> u128 {
    fib_fast_double(n).0
}

pub fn get_fib_numbers_until(limit: u128) -> Vec<u128> {
    let to_reserve = (((limit as f64).ln() + 0.8047) / 0.4812) as usize;
    let mut vec = Vec::with_capacity(to_reserve);

    let mut i = 0;
    loop {
        let fib = nth_fib_number(i);

        if fib > limit {
            break;
        }

        vec.push(fib);
        i += 1;
    }

    vec
}

fn fib_fast_double(n: u64) -> (u128, u128) {
    if n == 0 {
        return (0, 1);
    }

    let (a, b) = fib_fast_double(n >> 1);
    let c = a * ((b << 1u32) - a);
    let d = (a * a) + (b * b);

    if n & 1 == 0 {
        (c, d)
    } else {
        (d, c + d)
    }
}

pub fn prime_factors_of(mut num: u128) -> Vec<u128> {
    let mut primes = Vec::new();
    let prime_list = get_primes_until_n((num as f64).sqrt() as u128 + 1);

    for &prime in &prime_list {
        while num % prime == 0 {
            primes.push(prime);
            num /= prime;
        }
        if num == 1 {
            break;
        }
    }

    if num > 1 {
        primes.push(num);
    }

    primes
}

pub fn get_primes_until_n(n: u128) -> Vec<u128> {
    let limit: usize = n as usize;
    if limit < 2 {
        return vec![];
    }

    let mut isprime = VecDeque::from(vec![true; limit]);
    let mut prime = Vec::with_capacity((n as f64 / (n as f64).ln()) as usize + 10);

    isprime[0] = false;
    isprime[1] = false;

    for i in 2..limit {
        if isprime[i] {
            prime.push(i as u128);

            for j in (i * i..limit).step_by(i) {
                isprime[j] = false;
            }
        }
    }

    prime
}

fn estimate_upper_bound(n: u128) -> u128 {
    if n < 6 {
        return 15; // Small value for small n
    }
    let log_n = (n as f64).ln();

    (n as f64 * (log_n + log_n.ln())).ceil() as u128
}

pub fn nth_prime_number(n: usize) -> Option<u128> {
    let upper_bound = estimate_upper_bound(n as u128);
    let primes = get_primes_until_n(upper_bound);

    if n == 0 {
        return Some(1);
    }

    primes.get(n - 1).cloned()
}

fn create_palindrome(input: u128, b: u128, is_odd: bool) -> u128 {
    let mut n = input;
    let mut palindrome = input;

    if is_odd {
        n /= b;
    }

    while n > 0 {
        palindrome = palindrome * b + (n % b);
        n /= b;
    }

    palindrome
}

pub fn get_palindromes_until_n(limit: u128) -> Vec<u128> {
    let mut vec = Vec::with_capacity(limit.ilog10() as usize);

    for j in 0..2 {
        let mut i = 1;
        let mut palindrome = 0;

        while palindrome < limit {
            palindrome = create_palindrome(i, 10, (j % 2) != 0);

            vec.push(palindrome);
            i += 1;
        }
    }

    vec.sort_unstable();
    vec
}

pub fn is_palindrome(n: u128) -> bool {
    let mut reverse = 0;
    let mut temp = n;

    while temp != 0 {
        reverse = reverse * 10 + temp % 10;
        temp /= 10;
    }

    reverse == n
}

pub fn generate_proper_divisors(num: u128) -> HashSet<u128> {
    let mut divisors = HashSet::new();

    let sqrt_num = sqrt(num);
    for i in 1..=sqrt_num {
        if num % i == 0 {
            divisors.insert(i);
            if i != num / i {
                divisors.insert(num / i);
            }
        }
    }

    divisors.remove(&num);

    divisors
}
