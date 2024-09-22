fn main() {
    let mut i = 0;

    let check_s = |s: u32, i: &mut u32| -> u8 {
        if s > 200 {
            return 2;
        }
        if s == 200 {
            *i += 1;
            return 1;
        }
        0
    };

    for a in 0..=1 {
        if a == 1 {
            i += 1;
            continue;
        }
        for b in 0..=2 {
            let s = a * 200 + b * 100;
            let r = check_s(s, &mut i);
            if r == 2 {
                break;
            }
            if r == 1 {
                continue;
            }
            for c in 0..=5 {
                let s = a * 200 + b * 100 + c * 50;
                let r = check_s(s, &mut i);
                if r == 2 {
                    break;
                }
                if r == 1 {
                    continue;
                }
                for d in 0..=10 {
                    let s = a * 200 + b * 100 + c * 50 + d * 20;
                    let r = check_s(s, &mut i);
                    if r == 2 {
                        break;
                    }
                    if r == 1 {
                        continue;
                    }
                    for e in 0..=20 {
                        let s = a * 200 + b * 100 + c * 50 + d * 20 + e * 10;
                        let r = check_s(s, &mut i);
                        if r == 2 {
                            break;
                        }
                        if r == 1 {
                            continue;
                        }
                        for f in 0..=50 {
                            let s = a * 200 + b * 100 + c * 50 + d * 20 + e * 10 + f * 5;
                            let r = check_s(s, &mut i);
                            if r == 2 {
                                break;
                            }
                            if r == 1 {
                                continue;
                            }
                            for g in 0..=100 {
                                let s =
                                    a * 200 + b * 100 + c * 50 + d * 20 + e * 10 + f * 5 + g * 2;
                                let r = check_s(s, &mut i);
                                if r == 2 {
                                    break;
                                }
                                if r == 1 {
                                    continue;
                                }
                                for h in 0..=200 {
                                    if a * 200
                                        + b * 100
                                        + c * 50
                                        + d * 20
                                        + e * 10
                                        + f * 5
                                        + g * 2
                                        + h * 1
                                        == 200
                                    {
                                        i += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("ANSWER P31: {}", i);
}
