fn main() {
    let sq = 1001;
    let mut d = 1;
    let mut x = 0;

    let term = |d: u32| -> u32 {
        4 * d * d - 4 * d + 1
    };

    while 2 * d - 1 < sq {
        x += term(d);
        x += term(d) + 2 * d;
        x += term(d) + 4 * d;
        x += term(d) + 6 * d;

        d += 1;
    }

    x += sq * sq;

    println!("ANSWER P28: {x}");
}
