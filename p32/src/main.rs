fn main() {
    let mut nums_hashset: [u16; 16] = [0; 16];
    let mut nums_hashet_i = 0;

    #[rustfmt::skip]
    const PO2: [u16; 10] = [
        1 << 0, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5, 1 << 6, 1 << 7, 1 << 8, 1 << 9,
    ];

    for a in 1..=99 {
        for b in (a + 1)..=9999 / a {
            let product = a * b;

            let combined = format!("{}{}{}", a, b, product);
            if combined.len() != 9 {
                continue;
            }

            if combined
            .as_bytes()
            .iter()
            .map(|x| PO2[(*x - b'0') as usize])
            .reduce(|acc, e| acc | e)
            .unwrap()
            == 1022
            {
                // This is faster than a HashSet
                let mut to_insert = true;
                for x in nums_hashset.iter().take(nums_hashet_i) {
                    if *x == product {
                        to_insert = false;
                        break;
                    }
                }

                if to_insert {
                    nums_hashset[nums_hashet_i] = product;
                    nums_hashet_i += 1;
                }
            }
        }
    }

    println!("ANSWER P32: {}", nums_hashset.iter().sum::<u16>());
}
