use chrono::NaiveDate;

pub mod iter;
pub mod language;

pub fn is_weekend(weekday: chrono::Weekday) -> bool {
    use chrono::Weekday::{Sat, Sun};
    [Sun, Sat].contains(&weekday)
}

pub fn days_of_month(date: NaiveDate) -> u32 {
    use chrono::Datelike;
    match date.month() {
        1 => 31,
        2 => {
            if date.leap_year() {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => unreachable!(),
    }
}