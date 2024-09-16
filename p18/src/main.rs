use std::{collections::HashMap, error::Error, fs};

unsafe fn max_sum(
    i: u32,
    j: u32,
    list: &Vec<Vec<u32>>,
    dictionary: *mut HashMap<(u32, u32), u32>,
) -> u32 {
    let dictionary_result = (*dictionary).get(&(i, j));

    if dictionary_result.is_some() {
        return *dictionary_result.unwrap_unchecked();
    }

    if i as usize == list.len() - 1 {
        return list[i as usize][j as usize];
    }

    (*dictionary).insert(
        (i, j),
        list[i as usize][j as usize]
            + max_sum(i + 1, j, list, dictionary).max(max_sum(i + 1, j + 1, list, dictionary)),
    );

    (*dictionary)[&(i, j)]
}

fn main() -> Result<(), Box<dyn Error>> {
    let inputs_as_string = fs::read_to_string("./p18/input.txt")?;
    let inputs_vec = inputs_as_string
        .split('\n')
        .map(|x| x.split(' ').collect::<Vec<&str>>())
        .map(|x| {
            x.iter()
                .map(|x| str::parse::<u32>(x).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut hashmap = HashMap::with_capacity(1024);

    let x = unsafe { max_sum(0, 0, &inputs_vec, &mut hashmap) };
    println!("ANSWER P18: {x}");

    Ok(())
}
