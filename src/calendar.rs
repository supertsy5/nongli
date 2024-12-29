use crate::{
    is_weekend,
    language::{Language, MonthTitle},
    ChineseDate, SolarTerm,
};
use chrono::{Datelike, Month, NaiveDate, Weekday};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Options {
    pub language: Language,
    pub enable_chinese: bool,
    pub start_on_monday: bool,
    pub week_number: bool,
    pub color: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Calendar {
    year: i32,
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
    pub fn new(
        year: i32, month: Month, today: Option<NaiveDate>, options: Options,
    ) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month.number_from_month(), 1).map(|_date| Self {
            year,
            month,
            today,
            options,
        })
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn pred(mut self) -> Option<Self> {
        if self.month == Month::January {
            self.year -= 1;
        }
        self.month = self.month.pred();
        Self::new(self.year, self.month, self.today, self.options)
    }

    pub fn succ(mut self) -> Option<Self> {
        if self.month == Month::December {
            self.year += 1;
        }
        self.month = self.month.succ();
        Self::new(self.year, self.month, self.today, self.options)
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            calendar: self,
            day: 1,
        }
    }

    pub fn title(&self) -> MonthTitle {
        MonthTitle {
            year: self.year,
            month: self.month,
            enable_chinese: self.options.enable_chinese,
        }
    }
}

impl Iterator for Iter<'_> {
    type Item = (u32, [Option<Cell>; 7]);
    fn next(&mut self) -> Option<Self::Item> {
        let mut date = NaiveDate::from_ymd_opt(
            self.calendar.year,
            self.calendar.month.number_from_month(),
            self.day as u32,
        )?;

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
        loop {
            let weekday = date.weekday();
            let (chinese_date, solar_term) = if self.calendar.options.enable_chinese {
                (
                    ChineseDate::from_gregorian(&date),
                    SolarTerm::from_date(&date),
                )
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
            date = if let Some(date) = date
                .succ_opt()
                .filter(|new_date| new_date.month() == date.month())
            {
                date
            } else {
                break;
            };
            if weekday == end_of_week {
                break;
            }
        }
        let week = if self.calendar.options.week_number {
            if start_on_monday {
                array
                    .iter()
                    .find_map(|cell| cell.map(|cell| cell.date.iso_week().week()))
                    .unwrap_or_default()
            } else if let Some(monday) = array[1] {
                monday.date.iso_week().week()
            } else if let Some(sunday) = array[0] {
                if let Some(monday) = sunday.date.succ_opt() {
                    monday.iso_week().week()
                } else {
                    0
                }
            } else {
                array
                    .iter()
                    .find_map(|cell| cell.map(|cell| cell.date.iso_week().week()))
                    .unwrap_or_default()
            }
        } else {
            0
        };
        Some((week, array))
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
            week_number: true,
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
    for (a, b) in calendar.iter().zip(
        [1u32, 2, 3, 4, 5].into_iter().zip(
            array
                .chunks(7)
                .map(|chunk| <[Option<Cell>; 7]>::try_from(chunk).unwrap()),
        ),
    ) {
        assert_eq!(a, b);
    }
}
