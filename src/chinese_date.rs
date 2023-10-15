use crate::data::{CHUNJIE, DATA};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChineseDate {
    year: u16,
    month: u8,
    leap: bool,
    day: u8,
}

#[cfg(test)]
fn eprint_segmented(s: &str, size: usize) {
    let mut i = 0usize;
    let mut print_separator = false;
    for ch in s.chars() {
        if print_separator {
            print_separator = false;
            eprint!("-");
        }
        eprint!("{ch}");
        i += 1;
        if i == size {
            i = 0;
            print_separator = true;
        }
    }
}

impl ChineseDate {
    pub fn new(year: u16, month: u8, leap: bool, day: u8) -> Option<Self> {
        (!leap || DATA[year as usize - 1900] as u8 & 0x0f == month).then_some(ChineseDate {
            year,
            month,
            leap,
            day,
        })
    }
    pub fn from_gregorian(date: &impl chrono::Datelike) -> Option<Self> {
        let mut year = date.year();
        if !(1900..=2100).contains(&year) {
            return None;
        }
        let mut index = year as usize - 1900;
        let ordinal = date.ordinal0();
        let chunjie = crate::data::CHUNJIE[index] as u32;

        let chinese_ordinal = if ordinal >= chunjie {
            ordinal - chunjie
        } else if year >= 1901 {
            index -= 1;
            year -= 1;
            ordinal + crate::days_of_year(year as i32) as u32 - CHUNJIE[index] as u32
        } else {
            return None;
        } as u16;

        let data = crate::data::DATA[index];
        #[cfg(test)]
        {
            eprint!("data = ");
            eprint_segmented(&format!("{data:020b}"), 4);
            eprintln!();
        }
        let leap_month = data as u8 & 0x0f;

        let short_or_long = if leap_month > 0 {
            (data >> 3) & !((1 << 13 - leap_month) - 1) // Months before the leap month
                | ((data >> 16 & 1) << 12 - leap_month) // The leap month
                | (data >> 4) & ((1 << 12 - leap_month) - 1) // Monthes after the leap month
        } else {
            data >> 3 & 0x1ffe
        };
        #[cfg(test)]
        {
            eprint!("short_or_long = ");
            eprint_segmented(&format!("{short_or_long:016b}"), 4);
            eprintln!();
        }

        let mut month = 0u8;
        let mut day = chinese_ordinal;
        for i in 0..=12 {
            let days_of_month = (short_or_long >> 12 - i & 1) as u16 + 29;
            if day < days_of_month {
                month = i + 1;
                break;
            }
            day -= days_of_month;
        }
        let leap = if leap_month > 0 && month > leap_month {
            month -= 1;
            month == leap_month
        } else {
            false
        };
        Some(ChineseDate {
            year: year as u16,
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
}

#[cfg(test)]
#[test]
fn test() {
    const EXAMPLES: &[((i32, u32, u32), (u16, u8, bool, u8))] = &[
        ((2023, 1, 1), (2022, 12, false, 10)),
        ((2023, 1, 22), (2023, 1, false, 1)),
        ((2023, 10, 15), (2023, 9, false, 1)),
    ];
    for ((year, month, day), (ch_year, ch_month, ch_leap, ch_day)) in EXAMPLES.iter().copied() {
        assert_eq!(
            ChineseDate::from_gregorian(
                &chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap()
            ),
            ChineseDate::new(ch_year, ch_month, ch_leap, ch_day)
        );
    }
}
