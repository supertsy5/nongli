pub mod calendar;
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

pub fn is_leap_year(year: u16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub fn days_of_year(year: u16) -> u16 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

pub fn days_of_month(year: u16, month: chrono::Month) -> u8 {
    use chrono::Month::*;
    match month {
        January => 31,
        February => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        March => 31,
        April => 30,
        May => 31,
        June => 30,
        July => 31,
        August => 31,
        September => 30,
        October => 31,
        November => 30,
        December => 31,
    }
}
