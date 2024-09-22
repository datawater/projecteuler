fn main() {
    let x = (10..=1000000)
        .map(|x: u64| {
            let mut sum = 0;
            let mut y = x;

            while y != 0 {
                sum += (y % 10).pow(5);
                y /= 10;
            }

            if sum == x {
                x
            } else {
                0
            }
        })
        .sum::<u64>();

    println!("ANSWER P30: {x}");
}
