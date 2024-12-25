use crate::{days_of_month, is_weekend, language::Language, ChineseDate, SolarTerm};
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
    pub year: i32,
    pub month: Month,
    pub today: Option<NaiveDate>,
    pub options: Options,
}

#[derive(Clone, Debug)]
pub struct Iter<'a> {
    calendar: &'a Calendar,
    day: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Cell {
    pub date: NaiveDate,
    pub today: bool,
    pub weekend: bool,
    pub chinese_date: Option<ChineseDate>,
    pub solar_term: Option<SolarTerm>,
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
    type Item = [Option<Cell>; 7];
    fn next(&mut self) -> Option<Self::Item> {
        if self.day > days_of_month(self.calendar.year, self.calendar.month) {
            return None;
        }
        let start_on_monday = self.calendar.options.start_on_monday;
        let (start_of_week, end_of_week) = if start_on_monday {
            (Weekday::Mon, Weekday::Sun)
        } else {
            (Weekday::Sun, Weekday::Sat)
        };
        let today_day = self.calendar.today.and_then(|today| {
            (self.calendar.year == today.year()
                && self.calendar.month.number_from_month() == today.month())
            .then(|| today.day())
        });
        let mut array = [Option::<Cell>::None; 7];
        while self.day <= days_of_month(self.calendar.year, self.calendar.month) {
            let date = NaiveDate::from_ymd_opt(
                self.calendar.year,
                self.calendar.month.number_from_month(),
                self.day as u32,
            )
            .unwrap();
            let weekday = date.weekday();
            let (chinese_date, solar_term) = if self.calendar.options.enable_chinese {
                (ChineseDate::from_gregorian(&date), SolarTerm::from_date(&date))
            } else {
                (None, None)
            };
            array[weekday.days_since(start_of_week) as usize] = Some(Cell {
                date,
                today: today_day.is_some_and(|today| today == date.day()),
                weekend: is_weekend(date.weekday()),
                chinese_date,
                solar_term,
            });
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
            enable_chinese: true,
            language: Language::English,
            start_on_monday: false,
        },
    };
    let mut array = [Option::<Cell>::None; 35];
    for day in 1u32..=31 {
        let date = NaiveDate::from_ymd_opt(2025, 1, day).unwrap();
        array[day as usize + 2] = Some(Cell {
            date,
            weekend: is_weekend(date.weekday()),
            today: day == 1,
            chinese_date: ChineseDate::from_gregorian(&date),
            solar_term: SolarTerm::from_date(&date),
        });
    }
    for (a, b) in calendar.iter().zip(array.chunks(7)) {
        assert_eq!(a, b);
    }
}
