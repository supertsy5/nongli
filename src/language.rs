use std::fmt::{Display, Formatter, Result as FmtResult};

use chrono::{Month, Weekday};
use Language::*;

use crate::{
    calendar::Calendar,
    chinese_date::{ChineseDate, ChineseDay, ChineseMonth, ChineseYear},
    SolarTerm,
};

pub const TIANGAN_EN: &[&str] =
    &["Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui"];
pub const TIANGAN: &str = "甲乙丙丁戊己庚辛壬癸";
pub const DIZHI: &str = "子丑寅卯辰巳午未申酉戌亥";
pub const DIZHI_EN: &[&str] =
    &["zi", "chou", "yin", "mao", "chen", "si", "wu", "wei", "shen", "you", "xu", "hai"];
pub const SHENGXIAO_S: &str = "鼠牛虎兔龙蛇马羊猴鸡狗猪";
pub const SHENGXIAO_T: &str = "鼠牛虎兔龍蛇馬羊猴雞狗豬";
pub const SHENGXIAO_EN: &[&str] = &[
    "Rat", "Ox", "Tiger", "Rabbit", "Dragon", "Snake", "Horse", "Goat", "Monkey", "Rooster", "Dog",
    "Pig",
];
pub const NUMBER: &str = "一二三四五六七八九十";

pub fn get_char(s: &str, index: usize) -> Option<char> {
    s.get(index * 3..).and_then(|sub| sub.chars().next())
}

pub fn get_char_as_str(s: &str, index: usize) -> Option<&str> {
    s.get(index * 3..index * 3 + 3)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    English,
    ChineseSimplified,
    ChineseTraditional,
}

#[derive(Clone, Copy, Debug)]
pub struct YearTitle {
    pub year: i32,
    pub enable_chinese: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct MonthTitle {
    pub year: i32,
    pub month: Month,
    pub enable_chinese: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct TranslateAdapter<'a, T: Translate>(pub &'a T, pub Language);

#[derive(Clone, Copy, Debug)]
pub struct Short<'a, T: ShortTranslate>(pub &'a T);

pub trait Translate {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult;
    fn translate_to_string(&self, language: Language) -> String
    where
        Self: Sized,
    {
        TranslateAdapter(self, language).to_string()
    }
}

pub trait ShortTranslate {
    fn short_translate(&self, language: Language, f: &mut Formatter) -> FmtResult;
}

pub trait StaticTranslate {
    fn static_translate(&self, language: Language) -> &'static str;
}

impl<T: Translate> Display for TranslateAdapter<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.translate(self.1, f)
    }
}

impl From<Calendar> for MonthTitle {
    fn from(value: Calendar) -> Self {
        Self {
            year: value.year,
            month: value.month,
            enable_chinese: value.options.enable_chinese,
        }
    }
}

impl<T: ShortTranslate> Translate for Short<'_, T> {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        self.0.short_translate(language, f)
    }
}

impl Translate for ChineseYear {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        let relative_year = self.0 as i16 - 1984;
        let tiangan = relative_year.rem_euclid(10) as usize;
        let dizhi = relative_year.rem_euclid(12) as usize;
        match language {
            English => write!(
                f,
                "{}{} Year of the {}",
                TIANGAN_EN[tiangan], DIZHI_EN[dizhi], SHENGXIAO_EN[dizhi]
            ),
            _ => write!(
                f,
                "{}{}{}年",
                get_char(TIANGAN, tiangan).unwrap(),
                get_char(DIZHI, dizhi).unwrap(),
                get_char(
                    if language == ChineseTraditional {
                        SHENGXIAO_T
                    } else {
                        SHENGXIAO_S
                    },
                    dizhi,
                )
                .unwrap(),
            ),
        }
    }
}

impl Translate for ChineseMonth {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        match language {
            English => write!(f, "{}{}", self.0, if self.1 { "+" } else { "" }),
            _ => write!(
                f,
                "{}{}月",
                if self.1 {
                    if language == ChineseTraditional {
                        "閏"
                    } else {
                        "闰"
                    }
                } else {
                    ""
                },
                match self.0 {
                    1 => "正",
                    2..=10 => get_char_as_str(NUMBER, self.0 as usize - 1).unwrap(),
                    11 => "十一",
                    12 => "十二",
                    _ => unreachable!(),
                },
            ),
        }
    }
}

impl Translate for ChineseDay {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        match language {
            English => self.0.fmt(f),
            _ => match self.0 {
                1..=10 => write!(f, "初{}", get_char(NUMBER, self.0 as usize - 1).unwrap()),
                11..=19 => write!(f, "十{}", get_char(NUMBER, self.0 as usize - 11).unwrap()),
                20 => write!(f, "二十"),
                21..=29 => write!(f, "廿{}", get_char(NUMBER, self.0 as usize - 21).unwrap()),
                30 => write!(f, "三十"),
                _ => unreachable!(),
            },
        }
    }
}

impl Translate for MonthTitle {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        match language {
            English => write!(f, "{} {}", self.month.name(), self.year),
            chinese => write!(
                f,
                "{}年 {}",
                self.year,
                TranslateAdapter(&self.month, chinese)
            ),
        }?;
        if self.enable_chinese {
            write!(
                f,
                " {}",
                TranslateAdapter(&ChineseYear(self.year), language)
            )
        } else {
            Ok(())
        }
    }
}

impl Translate for YearTitle {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        match language {
            English => write!(f, "{}", self.year),
            _ => write!(f, "{}年", self.year),
        }?;
        if self.enable_chinese {
            write!(
                f,
                " {}",
                TranslateAdapter(&ChineseYear(self.year), language)
            )
        } else {
            Ok(())
        }
    }
}

impl Translate for Month {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        if language == English {
            write!(f, "{}", self.name())
        } else {
            write!(f, "{}月", TranslateAdapter(&Short(self), language))
        }
    }
}

impl ShortTranslate for Month {
    fn short_translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match language {
                English => &self.name()[..3],
                _ => self.static_translate(language),
            }
        )
    }
}

impl StaticTranslate for Month {
    fn static_translate(&self, language: Language) -> &'static str {
        let number = self.number_from_month();
        match language {
            English => self.name(),
            _ => match number {
                1..=10 => get_char_as_str("一二三四五六七八九十", number as usize - 1).unwrap(),
                11 => "十一",
                12 => "十二",
                _ => unreachable!(),
            },
        }
    }
}

impl Translate for Weekday {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        match language {
            English => write!(
                f,
                "{}",
                ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]
                    [self.num_days_from_sunday() as usize],
            ),
            _ => write!(f, "星期{}", TranslateAdapter(&Short(self), language)),
        }
    }
}

impl ShortTranslate for Weekday {
    fn short_translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.static_translate(language))
    }
}

impl StaticTranslate for Weekday {
    fn static_translate(&self, language: Language) -> &'static str {
        let index = self.num_days_from_sunday() as usize * 3;
        &(match language {
            English => "SunMonTueWedThuFriSat",
            _ => "日一二三四五六",
        })[index..index + 3]
    }
}

impl Translate for ChineseDate {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        let relative_year = self.year() as i16 - 1984;
        write!(
            f,
            "{}{}{}年{}{}月{}",
            get_char(TIANGAN, relative_year.rem_euclid(10) as usize).unwrap(),
            get_char(DIZHI, relative_year.rem_euclid(12) as usize).unwrap(),
            get_char(SHENGXIAO_S, relative_year.rem_euclid(12) as usize).unwrap(),
            if self.leap() { "闰" } else { "" },
            TranslateAdapter(&ChineseMonth(self.month(), self.leap()), language),
            TranslateAdapter(&ChineseDay(self.day()), language),
        )
    }
}

impl StaticTranslate for SolarTerm {
    fn static_translate(&self, language: Language) -> &'static str {
        let ordinal = self.as_ordinal() as usize;
        if language == English {
            [
                "Xiaohan",
                "Dahan",
                "Lichun",
                "Yushui",
                "Jingzhe",
                "Chunfen",
                "Qingming",
                "Guyu",
                "Lixia",
                "Xiaoman",
                "Mangzhong",
                "Xiazhi",
                "Xiaoshu",
                "Dashu",
                "Liqiu",
                "Chushu",
                "Bailu",
                "Qiufen",
                "Hanlu",
                "Shuangjiang",
                "Lidong",
                "Xiaoxue",
                "Daxue",
                "Dongzhi",
            ][ordinal]
        } else {
            &(match language {
                ChineseSimplified => "小寒大寒立春雨水惊蛰春分清明谷雨立夏小满芒种夏至小暑大暑立秋处暑白露秋分寒露霜降立冬小雪大雪冬至",
                ChineseTraditional => "小寒大寒立春雨水驚蟄春分清明穀雨立夏小滿芒種夏至小暑大暑立秋處暑白露秋分寒露霜降立冬小雪大雪冬至",
                English => unreachable!(),
            })[ordinal * 6..ordinal * 6 + 6]
        }
    }
}

impl Translate for SolarTerm {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.static_translate(language))
    }
}

impl ShortTranslate for SolarTerm {
    fn short_translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        if language == English && self.static_translate(English).len() > 6 {
            write!(f, "{}.", &self.static_translate(English)[..5])
        } else {
            self.translate(language, f)
        }
    }
}
