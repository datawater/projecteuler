// def solution(a, b):
//     n = a % b
//     if n == 0:
//         return 0

//     mem = {}
//     n *= 10
//     pos = 0

//     while True:
//         pos += 1
//         n = n % b
//         if n == 0:
//             return 0
//         if n in mem:
//             i = mem[n]
//             return pos - i
//         else:
//             mem[n] = pos
//         n *= 10

use std::collections::HashMap;

fn l(a: u16) -> u16 {
    let mut mem: HashMap<u16, u16> = HashMap::new();
    let mut n = 10;
    let mut pos = 0;

    loop {
        pos += 1;
        n %= a;
        if n == 0 {
            return 0;
        }

        if mem.contains_key(&n) {
            let i = mem[&n];
            return pos - i;
        }

        mem.insert(n, pos);
        n *= 10;
    }
}

fn main() {
    let mut biggest = 0;
    let mut answer = 0;
    (1..1000).for_each(|x| {
        let l = l(x);

        if l > biggest {
            biggest = l;
            answer = x;
        }
    });

    println!("ANSWER P16: {answer}");
}
