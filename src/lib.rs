pub mod chinese_date;
pub mod data;
pub mod iter;
pub mod language;
pub mod solar_term;

pub use solar_term::SolarTerm;

pub fn is_weekend(weekday: chrono::Weekday) -> bool {
    use chrono::Weekday::{Sat, Sun};
    [Sun, Sat].contains(&weekday)
}

pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub fn days_of_year(year: i32) -> u16 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

pub fn days_of_month(date: &impl chrono::Datelike) -> u32 {
    match date.month() {
        1 => 31,
        2 => {
            if is_leap_year(date.year()) {
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
