use num::integer::Roots;

pub fn nth_fib_number(n: u64) -> u128 {
    return fib_fast_double(n).0;
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

    return vec;
}

fn fib_fast_double(n: u64) -> (u128, u128) {
    if n == 0 {
        return (0, 1);
    }

    let (a, b) = fib_fast_double(n >> 1);
    let c = a * ((b << 1u32) - a);
    let d = (a * a) + (b * b);

    return if n & 1 == 0 {
        (c, d)
    } else {
        (d, c + d)
    };
}

pub fn prime_factors_of(num: u128) -> Vec<u128> {
    let sqrt = num.sqrt();

    return get_primes_until_n(sqrt).into_iter().filter(|x| num % x == 0).collect();
}

pub fn get_primes_until_n(n: u128) -> Vec<u128> {
    let limit: usize = n as usize;
    if limit < 2 {
        return vec![];
    }

    let mut isprime = vec![true; limit];
    let mut prime = Vec::with_capacity((n as f64 / (n as f64).ln()) as usize + 10);

    isprime[0] = false; // 0 is not a prime number
    isprime[1] = false; // 1 is not a prime number

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
    let estimate = (n as f64 * (log_n + log_n.ln())).ceil() as u128;
    estimate
}

pub fn nth_prime(n: usize) -> Option<u128> {
    let upper_bound = estimate_upper_bound(n as u128);
    let primes = get_primes_until_n(upper_bound);

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

    return palindrome;
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
    return vec;
}

pub fn is_palindrome(n: u128) -> bool {
    let mut reverse = 0;
    let mut temp = n;

    while temp != 0 {
        reverse = reverse * 10 + temp % 10;
        temp /= 10; 
    }

    return reverse == n;
}