use crate::data::CHUNJIE;

pub struct ChineseDate {
    year: u16,
    month: u8,
    leap: bool,
    day: u8,
}

impl ChineseDate {
    pub fn new(date: &impl chrono::Datelike) -> Option<Self> {let year = date.year();
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
            ordinal + 365 + crate::days_of_year(year as i32) as u32 - CHUNJIE[index] as u32
        } else {
            return None;
        } as u16;
    
        let data = crate::data::DATA[index];
        let leap_month = data as u8 & 0x0f;
    
        let big_or_small = if leap_month > 0 {
            (data >> 3) & !((1 << leap_month) - 1)
                | (1 << leap_month - 1)
                | (data >> 4) & ((1 << leap_month - 1) - 1)
        } else {
            data >> 3
        };
    
        let mut month = 0u8;
        let mut day = chinese_ordinal;
        for i in 0..=12 {
            let days_of_month = (big_or_small >> 12 - i & 1) as u16 + 29;
            if day < days_of_month {
                month = i + 1;
                break;
            }
            day -= days_of_month;
        }
        let leap = if leap_month > 0 && month > leap_month {
            month -= 1;
            true
        } else {
            false
        };
        Some(ChineseDate { year: year as u16, month, leap, day: day as u8 })
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
