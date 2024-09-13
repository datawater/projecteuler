use utils::*;

fn main() {
    let mut largest = 0;

    for n1 in (100..999).rev() {
        for n2 in (100..999).rev() {
            let x = n1 * n2;

            if is_palindrome(x) {
                largest = largest.max(x);
                break;
            }
        }
    }

    println!("ANSWER P4: {largest}");
}
