fn main() {
    let x = [2, 3, 2, 5, 7, 2, 3, 11, 13, 2, 17, 19];

    println!("ANSWER P5: {}", x.into_iter().product::<u64>());
}
