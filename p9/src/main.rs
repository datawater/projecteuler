fn calculate_b(a: i32) -> i32 {
    1000 * (a - 500) / (a - 1000)
}

fn calcualte_c(a: i32) -> i32 {
    (1000 * a - 500_000 - (a * a)) / (a - 1000)
}

fn main() {
    for a in 1..500 {
        let b = calculate_b(a);
        let c = calcualte_c(a);
        let sum_of_squares = a * a + b * b;
        let sum = a + b + c;

        if sum_of_squares == c * c && sum == 1000 {
            println!("ANSWER P9: {}", a * b * c);
            break;
        }
    }
}
