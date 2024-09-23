use std::collections::HashSet;

fn main() {
    let mut nums_hashset = HashSet::new();

    for a in 1..=99 {
        for b in (a + 1)..=9999 / a {
            let product = a * b;

            let combined = format!("{}{}{}", a, b, product);
            if combined.len() != 9 {
                continue;
            }

            let mut digits = [false; 10];

            let mut valid = true;
            for c in combined.chars() {
                let d = c.to_digit(10).unwrap();
                if d == 0 || digits[d as usize] {
                    valid = false;
                    break;
                }
                digits[d as usize] = true;
            }

            if valid {
                nums_hashset.insert(product);
            }
        }
    }

    println!("ANSWER P32: {}", nums_hashset.iter().sum::<u32>());
}
