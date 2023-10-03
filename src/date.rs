use std::fmt::{
    Display, Formatter, Result as FmtResult, Debug,
};

use Day::*;
use Month::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

pub struct Date {
    year: u16,
    month: Month,
    day: u8,
}

pub const DAYS: [Day; 7] = [
    Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday,
];
pub const MONTHS: [Month; 12] = [
    January, February, March, April, May, June, July, August, September, October, November,
    December,
];
pub const UNIX_EPOCH: Date = Date {
    year: 1970,
    month: January,
    day: 1,
};
pub const DAY_OF_UNIX_EPOCH: Day = Thursday;

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

pub fn days_of_month(year: u16, month: Month) -> u8 {
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

impl Month {
    pub fn from_number(number: u8) -> Option<Month> {
        number
            .checked_sub(1)
            .and_then(|number| MONTHS.get(number as usize))
            .copied()
    }
    pub fn as_number(self) -> u8 {
        self as u8 + 1
    }
}

impl Display for Month {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(self, f)
    }
}

impl Day {
    pub fn from_number(number: u8) -> Option<Day> {
        if number == 7 {
            Some(Sunday)
        } else {
            DAYS.get(number as usize).copied()
        }
    }
    pub fn as_number(self) -> u8 {
        self as u8
    }
    pub fn offset(self, days: i32) -> Day {
        DAYS[(self as i32 + days.rem_euclid(7)).rem_euclid(7) as usize]
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(self, f)
    }
}

impl Date {
    pub fn new(year: u16, month: Month, day: u8) -> Option<Self> {
        (year >= 1970 && day <= days_of_month(year, month)).then_some(Self { year, month, day })
    }
    pub fn year(&self) -> u16 {
        self.year
    }
    pub fn month(&self) -> Month {
        self.month
    }
    pub fn day(&self) -> u8 {
        self.day
    }
    pub fn day_of_week(&self) -> Day {
        DAY_OF_UNIX_EPOCH
            .offset((1970..self.year).map(days_of_year).sum::<u16>() as i32)
            .offset(
                (1..self.month.as_number())
                    .map(|month| {
                        days_of_month(self.year, Month::from_number(month).unwrap()) as i32
                    })
                    .sum::<i32>(),
            )
            .offset(self.day as i32 - 1)
    }
    pub fn from_unix_epoch(mut days: u32) -> Self {
        let mut year = 1970u16;
        loop {
            let days_of_this_year = days_of_year(year) as u32;
            if days < days_of_this_year {
                break;
            }
            days -= days_of_this_year;
            year += 1;
        }
        let mut months = MONTHS.iter().copied();
        let mut month;
        loop {
            month = months.next().unwrap();
            let days_of_this_month = days_of_month(year, month) as u32;
            if days < days_of_this_month {
                break;
            }
            days -= days_of_this_month;
            year += 1;
        }
        Self { year, month, day: days as u8 + 1 }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}-{}-{}", self.year, self.month.as_number(), self.day)
    }
}

#[cfg(test)]
#[test]
fn tests() {
    assert_eq!(UNIX_EPOCH.day_of_week(), Thursday);
    assert_eq!(Date::new(1970, February, 1).unwrap().day_of_week(), Sunday);
    assert_eq!(Date::new(1971, January, 1).unwrap().day_of_week(), Friday);
    assert_eq!(Date::new(2023, October, 3).unwrap().day_of_week(), Tuesday);
}
