use chrono::NaiveDate;

use crate::data::{CHUNJIE, DATA};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChineseDate {
    year: u16,
    month: u8,
    leap: bool,
    day: u8,
}

pub fn data(year: u16) -> Option<u32> {
    (1900..=2100)
        .contains(&year)
        .then(|| DATA[year as usize - 1900])
}

pub fn short_or_long(year: u16) -> Option<u16> {
    data(year).map(|data| {
        let leap_month = data as u8 & 0x0f;
        (if leap_month > 0 {
            (data >> 3) & !((1 << (13 - leap_month)) - 1) // Months before the leap month
                    | ((data >> 16 & 1) << (12 - leap_month)) // The leap month
                    | (data >> 4) & ((1 << (12 - leap_month)) - 1) // Monthes after the leap month
        } else {
            data >> 3 & 0x1ffe
        }) as u16
    })
}

pub fn ordinal_month(year: u16, month: u8, leap: bool) -> Option<u8> {
    let data = data(year)?;
    if !(1..=12).contains(&month) {
        return None;
    }
    let leap_month = data as u8 & 0x0f;
    if leap && month != leap_month {
        return None;
    }
    Some(if leap_month > 0 && month >= leap_month {
        month
    } else {
        month - 1
    })
}

pub fn is_long_month(year: u16, month: u8, leap: bool) -> Option<bool> {
    ordinal_month(year, month, leap).and_then(|ord_month| {
        short_or_long(year).map(|short_long| short_long >> (12 - ord_month) & 1 > 0)
    })
}

pub fn leap_month(year: u16) -> u8 {
    match data(year) {
        Some(data) => data as u8 & 0x0f,
        None => 0,
    }
}

impl ChineseDate {
    pub fn new(year: u16, month: u8, leap: bool, day: u8) -> Option<Self> {
        let data = data(year)?;
        (!leap || data as u8 & 0x0f == month).then_some(ChineseDate {
            year,
            month,
            leap,
            day,
        })
    }
    pub fn from_gregorian(date: &impl chrono::Datelike) -> Option<Self> {
        let year = date.year();
        if !(1900..=2100).contains(&year) {
            return None;
        }
        let mut year = year as u16;
        let mut index = year as usize - 1900;
        let ordinal = date.ordinal0();
        let chunjie = crate::data::CHUNJIE[index] as u32;

        let chinese_ordinal = if ordinal >= chunjie {
            ordinal - chunjie
        } else if year >= 1901 {
            index -= 1;
            year -= 1;
            ordinal + crate::days_of_year(year) as u32 - CHUNJIE[index] as u32
        } else {
            return None;
        } as u16;

        Self::from_ordinal(year, chinese_ordinal)
    }
    pub fn from_ordinal(year: u16, ordinal: u16) -> Option<Self> {
        let mut month = 0u8;
        let mut day = ordinal;
        let leap_month = leap_month(year);
        let short_long = short_or_long(year)?;
        for i in 0..=12 {
            let days_of_month = (short_long >> (12 - i) & 1) as u16 + 29;
            if day < days_of_month {
                month = i + 1;
                break;
            }
            day -= days_of_month;
            if i == 12 {
                return None;
            }
        }
        let leap = if leap_month > 0 && month > leap_month {
            month -= 1;
            month == leap_month
        } else {
            false
        };
        Some(ChineseDate {
            year,
            month,
            leap,
            day: day as u8 + 1,
        })
    }
    pub fn year(&self) -> u16 {
        self.year
    }
    pub fn month(&self) -> u8 {
        self.month
    }
    pub fn leap(&self) -> bool {
        self.leap
    }
    pub fn day(&self) -> u8 {
        self.day
    }
    pub fn ordinal(&self) -> u16 {
        let mut ord = 0u16;
        let leap_month = leap_month(self.year);
        let data = data(self.year).unwrap();
        for i in 1..self.month {
            ord += 29 + (data >> (16 - i) & 1) as u16;
            if i == leap_month {
                ord += 29 + (data >> 16 & 1) as u16;
            }
        }
        if self.leap {
            ord += 29 + (data >> (16 - self.month) & 1) as u16;
        }
        ord += self.day as u16 - 1;
        ord
    }
    pub fn to_gregorian(&self) -> NaiveDate {
        let mut ordinal = self.ordinal() + 1;
        ordinal += CHUNJIE[self.year as usize - 1900] as u16;
        let days_of_year = crate::days_of_year(self.year);
        dbg!(ordinal);
        let year = if ordinal < days_of_year {
            self.year as i32
        } else {
            ordinal -= days_of_year;
            self.year as i32 + 1
        };
        NaiveDate::from_yo_opt(year, ordinal as u32).unwrap()
    }
}

#[cfg(test)]
#[test]
fn test() {
    #[allow(clippy::type_complexity)]
    const EXAMPLES: &[((i32, u32, u32), (u16, u8, bool, u8))] = &[
        ((1970, 1, 1), (1969, 11, false, 24)),
        ((2023, 1, 1), (2022, 12, false, 10)),
        ((2023, 1, 22), (2023, 1, false, 1)),
        ((2023, 2, 20), (2023, 2, false, 1)),
        ((2023, 3, 22), (2023, 2, true, 1)),
        ((2023, 4, 20), (2023, 3, false, 1)),
        ((2023, 10, 15), (2023, 9, false, 1)),
    ];
    for ((year, month, day), (ch_year, ch_month, ch_leap, ch_day)) in EXAMPLES.iter().copied() {
        let gregorian_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let chinese_date = ChineseDate::new(ch_year, ch_month, ch_leap, ch_day).unwrap();
        assert_eq!(
            ChineseDate::from_gregorian(&gregorian_date).unwrap(),
            chinese_date,
        );
        assert_eq!(chinese_date.to_gregorian(), gregorian_date);
    }
}
