fn factorial(x: u32) -> u32 {
    (2..=x).product()
}

fn main() {
    let mut digits = (0..=9).collect::<Vec<u32>>();

    let mut res = Vec::new();
    let mut n = 999999;

    for i in (0..=9).rev() {
        let f = factorial(i);
        let index = n / f;
        n %= f;

        res.push(digits.remove(index as usize));
    }

    println!("ANSWER P24: {:?}", res);
}
