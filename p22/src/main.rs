use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let inputs_string = fs::read_to_string("./p22/inputs.txt")?;

    let mut inputs: Vec<&str> = inputs_string
        .split(',')
        .map(|x| x.trim_matches('"'))
        .collect();

    inputs.sort();

    let sum = inputs
        .into_iter()
        .enumerate()
        .map(|x| {
            (x.0 + 1)
                * x.1
                    .as_bytes()
                    .iter()
                    .map(|c| (c - b'A' + 1) as usize)
                    .sum::<usize>()
        })
        .sum::<usize>();

    println!("ANSWER P22: {sum}");

    Ok(())
}
