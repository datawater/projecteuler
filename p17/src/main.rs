const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const GAP: &str = "and";

fn number_to_word(num: usize) -> String {
    if num < 20 {
        return ONES[num].to_owned();
    }

    if num > 19 && num < 100 {
        let _low = num % 10;
        let low = if _low == 0 { "" } else { ONES[_low] };
        let high = num / 10;

        return TENS[high].to_owned() + low;
    }

    if num > 99 && num < 1000 {
        let _low = num % 100;
        let low = if _low == 0 {
            "".to_owned()
        } else {
            GAP.to_owned() + &number_to_word(_low)
        };
        let high = num / 100;

        return number_to_word(high) + "hundred" + &low;
    }

    "onethousand".to_owned()
}

fn main() {
    let mut n = 0;
    (1..=1000).for_each(|x| n += number_to_word(x).len());
    println!("ANSWER P17: {n}");
}
