use crate::{days_of_month, language::Language};
use chrono::{Datelike, Month, NaiveDate, Weekday};

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub language: Language,
    pub enable_chinese: bool,
    pub start_on_monday: bool,
    pub color: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Calendar {
    pub year: u16,
    pub month: Month,
    pub today: Option<NaiveDate>,
    pub options: Options,
}

#[derive(Clone, Debug)]
pub struct Iter<'a> {
    calendar: &'a Calendar,
    day: u8,
}

impl Calendar {
    pub fn pred(mut self) -> Self {
        if self.month == Month::January {
            self.year -= 1;
        }
        self.month = self.month.pred();
        self
    }
    pub fn succ(mut self) -> Self {
        if self.month == Month::December {
            self.year += 1;
        }
        self.month = self.month.succ();
        self
    }
    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter {
            calendar: self,
            day: 1,
        }
    }
}

impl Iterator for Iter<'_> {
    type Item = [Option<NaiveDate>; 7];
    fn next(&mut self) -> Option<Self::Item> {
        if self.day >= days_of_month(self.calendar.year, self.calendar.month) {
            return None;
        }
        let start_on_monday = self.calendar.options.start_on_monday;
        let (start_of_week, end_of_week) = if start_on_monday {
            (Weekday::Mon, Weekday::Sun)
        } else {
            (Weekday::Sun, Weekday::Sat)
        };
        let mut array = [Option::<NaiveDate>::None; 7];
        while self.day <= days_of_month(self.calendar.year, self.calendar.month) {
            let date = NaiveDate::from_ymd_opt(
                self.calendar.year as i32,
                self.calendar.month.number_from_month(),
                self.day as u32,
            )
            .unwrap();
            let weekday = date.weekday();
            array[weekday.days_since(start_of_week) as usize] = Some(date);
            self.day += 1;
            if weekday == end_of_week {
                break;
            }
        }
        Some(array)
    }
}

#[cfg(test)]
#[test]
fn test() {
    let calendar = Calendar {
        year: 2025,
        month: Month::January,
        today: NaiveDate::from_ymd_opt(2025, 1, 1),
        options: Options {
            color: false,
            enable_chinese: false,
            language: Language::English,
            start_on_monday: false,
        },
    };
    assert_eq!(
        calendar.iter().collect::<Vec<_>>(),
        &[
            [
                None,
                None,
                None,
                NaiveDate::from_ymd_opt(2025, 1, 1),
                NaiveDate::from_ymd_opt(2025, 1, 2),
                NaiveDate::from_ymd_opt(2025, 1, 3),
                NaiveDate::from_ymd_opt(2025, 1, 4),
            ],
            [
                NaiveDate::from_ymd_opt(2025, 1, 5),
                NaiveDate::from_ymd_opt(2025, 1, 6),
                NaiveDate::from_ymd_opt(2025, 1, 7),
                NaiveDate::from_ymd_opt(2025, 1, 8),
                NaiveDate::from_ymd_opt(2025, 1, 9),
                NaiveDate::from_ymd_opt(2025, 1, 10),
                NaiveDate::from_ymd_opt(2025, 1, 11),
            ],
            [
                NaiveDate::from_ymd_opt(2025, 1, 12),
                NaiveDate::from_ymd_opt(2025, 1, 13),
                NaiveDate::from_ymd_opt(2025, 1, 14),
                NaiveDate::from_ymd_opt(2025, 1, 15),
                NaiveDate::from_ymd_opt(2025, 1, 16),
                NaiveDate::from_ymd_opt(2025, 1, 17),
                NaiveDate::from_ymd_opt(2025, 1, 18),
            ],
            [
                NaiveDate::from_ymd_opt(2025, 1, 19),
                NaiveDate::from_ymd_opt(2025, 1, 20),
                NaiveDate::from_ymd_opt(2025, 1, 21),
                NaiveDate::from_ymd_opt(2025, 1, 22),
                NaiveDate::from_ymd_opt(2025, 1, 23),
                NaiveDate::from_ymd_opt(2025, 1, 24),
                NaiveDate::from_ymd_opt(2025, 1, 25),
            ],
            [
                NaiveDate::from_ymd_opt(2025, 1, 26),
                NaiveDate::from_ymd_opt(2025, 1, 27),
                NaiveDate::from_ymd_opt(2025, 1, 28),
                NaiveDate::from_ymd_opt(2025, 1, 29),
                NaiveDate::from_ymd_opt(2025, 1, 30),
                NaiveDate::from_ymd_opt(2025, 1, 31),
                None,
            ]
        ]
    );
}
