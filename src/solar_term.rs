use crate::data::SOLAR_TERMS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolarTerm {
    Xiaohan,
    Dahan,
    Lichun,
    Yushui,
    Jingzhe,
    Chunfen,
    Qingming,
    Guyu,
    Lixia,
    Xiaoman,
    Mangzhong,
    Xiazhi,
    Xiaoshu,
    Dashu,
    Liqiu,
    Chushu,
    Bailu,
    Qiufen,
    Hanlu,
    Shuangjiang,
    Lidong,
    Xiaoxue,
    Daxue,
    Dongzhi,
}

impl SolarTerm {
    pub fn as_ordinal(self) -> u8 {
        self as u8
    }
    pub fn from_ordinal(ordinal: u8) -> Option<Self> {
        use SolarTerm::*;
        match ordinal {
            0 => Some(Xiaohan),
            1 => Some(Dahan),
            2 => Some(Lichun),
            3 => Some(Yushui),
            4 => Some(Jingzhe),
            5 => Some(Chunfen),
            6 => Some(Qingming),
            7 => Some(Guyu),
            8 => Some(Lixia),
            9 => Some(Xiaoman),
            10 => Some(Mangzhong),
            11 => Some(Xiazhi),
            12 => Some(Xiaoshu),
            13 => Some(Dashu),
            14 => Some(Liqiu),
            15 => Some(Chushu),
            16 => Some(Bailu),
            17 => Some(Qiufen),
            18 => Some(Hanlu),
            19 => Some(Shuangjiang),
            20 => Some(Lidong),
            21 => Some(Xiaoxue),
            22 => Some(Daxue),
            23 => Some(Dongzhi),
            _ => None,
        }
    }
    pub fn from_date(date: &impl chrono::Datelike) -> Option<SolarTerm> {
        let year = date.year();
        if year < 1900 {
            return None;
        }
        let solar_terms = SOLAR_TERMS.get(year as usize - 1900)?;
        let ordinal0 = date.month0() as u8 * 2;
        let ordinal1 = ordinal0 + 1;
        let day = date.day() as u8;
        if day == solar_terms[ordinal0 as usize] {
            SolarTerm::from_ordinal(ordinal0)
        } else if day == solar_terms[ordinal1 as usize] {
            SolarTerm::from_ordinal(ordinal1)
        } else {
            None
        }
    }
    pub fn is_midterm(self) -> bool {
        self.as_ordinal() % 2 > 0
    }
}

#[cfg(test)]
#[test]
fn test() {
    for (i, solar_terms) in SOLAR_TERMS.iter().enumerate() {
        let year = i + 1900;
        for (j, day) in solar_terms.iter().enumerate() {
            let month = j as u32 / 2 + 1;
            let date = chrono::NaiveDate::from_ymd_opt(year as i32, month, *day as u32).unwrap();
            dbg!(date);
            assert_eq!(
                SolarTerm::from_date(&date),
                SolarTerm::from_ordinal(j as u8)
            );
        }
    }
}
