use std::collections::HashMap;
use utils::prime_factors_of;

fn number_to_powers_of_ten(mut n: u128) -> HashMap<u8, u128> {
    let mut power = 0;
    let mut parts = HashMap::new();

    while n > 0 {
        let digit = n % 10;
        if digit != 0 {
            parts.insert(power, digit);
        }

        n /= 10;
        power += 1;
    }

    parts
}

fn multiply_power_of_ten_polynomial(
    lhs: &HashMap<u8, u128>,
    rhs: &HashMap<u8, u128>,
) -> HashMap<u8, u128> {
    let mut r = HashMap::new();

    for (&lhs_power, &lhs_coeff) in lhs {
        for (&rhs_power, &rhs_coeff) in rhs {
            let new_power = lhs_power + rhs_power;
            let new_coeff = lhs_coeff * rhs_coeff;

            *r.entry(new_power).or_insert(0) += new_coeff;
        }
    }

    r
}

fn multiply_n_polynomials(p: &[HashMap<u8, u128>]) -> HashMap<u8, u128> {
    let mut result = p[0].clone(); // Start with the first polynomial

    for poly in &p[1..] {
        result = multiply_power_of_ten_polynomial(&result, poly); // Mutate in-place
    }

    result
}

fn split_term(power: u8, coeff: u128, result: &mut HashMap<u8, u128>) {
    let mut current_coeff = coeff;
    let mut current_power = power;

    while current_coeff > 0 {
        let digit = current_coeff % 10;
        *result.entry(current_power).or_insert(0) += digit;
        current_coeff /= 10;
        current_power += 1;
    }
}

fn simplify_polynomial(poly: &HashMap<u8, u128>) -> HashMap<u8, u128> {
    let mut simplified_poly = poly.clone();

    loop {
        let mut new_poly = HashMap::new();
        let mut needs_check = false;

        for (&power, &coeff) in &simplified_poly {
            if coeff >= 10 {
                needs_check = true;
            }
            split_term(power, coeff, &mut new_poly);
        }

        if !needs_check {
            return new_poly;
        }

        simplified_poly = new_poly;
    }
}

fn main() {
    let factorial_100 = (1..=100)
        .flat_map(prime_factors_of)
        .fold(HashMap::new(), |mut map, x| {
            map.entry(x).and_modify(|frq| *frq += 1).or_insert(1);
            map
        });

    let powers_of_ten = factorial_100
        .into_iter()
        .map(|x| number_to_powers_of_ten((x.0).pow(x.1 as u32)))
        .collect::<Vec<HashMap<u8, u128>>>();

    let powers_of_ten = multiply_n_polynomials(&powers_of_ten);
    let simplified = simplify_polynomial(&powers_of_ten);

    println!("ANSWER P20: {}", simplified.values().sum::<u128>());
}
