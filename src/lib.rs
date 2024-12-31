/*!
# Nongli: A Rust library and CLI tool for Chinese calendar
Talk is cheap, let me show the code.
```
fn test() {
    use chrono::NaiveDate;
    use nongli::{ChineseDate, SolarTerm};
    let date = NaiveDate::from_ymd_opt(2023, 10, 30).unwrap();
    let chinese_date = ChineseDate::new(2023, 09, false, 16).unwrap();
    assert_eq!(ChineseDate::from_gregorian(&date), Some(chinese_date));
    assert_eq!(chinese_date.to_gregorian(), date);
    assert_eq!(
        SolarTerm::from_date(&NaiveDate::from_ymd_opt(2023, 10, 25).unwrap()),
        Some(SolarTerm::Shuangjiang)
    );
}
```
*/

pub mod calendar;
pub mod chinese_date;
#[cfg(feature = "cli")]
pub mod cli_calendar;
pub mod data;
pub mod festivals;
pub mod iter;
pub mod language;
pub mod solar_term;

pub use chinese_date::ChineseDate;
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

pub fn days_of_month(year: i32, month: chrono::Month) -> u8 {
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
