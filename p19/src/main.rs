// 1 Jan 1901 -- 31 Dec 2000
// 01 01 1901 -- 31 12 200

// Dreißig Tage hat September,
// April, Juni und November.
// Februar hat achtundzwanzig,
// nur im Schaltjahr neunundzwanzig.
// Alle andere ohne Frage
// haben einunddreißig Tage.

fn get_weekday(d: i16, m: i16, y: i16) -> i16 {
    let k = y % 100;
    let j = y / 100;

    (d + ((13 * (m + 1)) / 5) + k + (k / 4) + (j / 4) - 2 * j) % 7
}

fn main() {
    let mut sunday_count = 0;

    for year in 1901..=2000 {
        for month in 1..=12 {
            for day in 1..=31 {
                let mut month_new = month;
                let mut year_new = year;

                if month == 1 || month == 2 {
                    month_new += 12;
                    year_new -= 1;
                }

                if day == 1 && get_weekday(day, month_new, year_new) == 0 {
                    sunday_count += 1;
                }

                if month == 2 && ((day == 29 && year % 4 == 0) || (day == 28 && year % 4 != 0)) {
                    break;
                }
            }
        }
    }

    println!("ANSWER P19: {sunday_count}");
}
