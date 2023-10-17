use std::fmt::{Display, Formatter, Result as FmtResult};

use chrono::{
    Month::{self, *},
    Weekday::{self, *},
};
use Language::*;

use crate::chinese_date::{ChineseDate, ChineseDay, ChineseMonth, ChineseYear};

pub const TIANGAN_EN: &[&str] = &[
    "Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui",
];
pub const TIANGAN: &str = "甲乙丙丁戊己庚辛壬癸";
pub const DIZHI: &str = "子丑寅卯辰巳午未申酉戌亥";
pub const DIZHI_EN: &[&str] = &[
    "zi", "chou", "yin", "mao", "chen", "si", "wu", "wei", "shen", "you", "xu", "hai",
];
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
pub struct YearTitle(pub u16, pub bool);

#[derive(Clone, Copy, Debug)]
pub struct MonthTitle(pub u16, pub Month, pub bool);

#[derive(Clone, Copy, Debug)]
pub struct TranslateAdapter<'a, T: Translate>(pub &'a T, pub Language);

#[derive(Clone, Copy, Debug)]
pub struct ShortTranslateAdapter<'a, T: ShortTranslate>(pub &'a T, pub Language);

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

impl<'a, T: Translate> Display for TranslateAdapter<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.translate(self.1, f)
    }
}

impl<'a, T: ShortTranslate> Display for ShortTranslateAdapter<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.short_translate(self.1, f)
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
            English => write!(f, "{} {}", self.1.name(), self.0),
            chinese => write!(f, "{}年 {}", self.0, TranslateAdapter(&self.1, chinese)),
        }?;
        if self.2 {
            write!(f, " {}", TranslateAdapter(&ChineseYear(self.0), language))
        } else {
            Ok(())
        }
    }
}

impl Translate for YearTitle {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        match language {
            English => write!(f, "{}", self.0),
            _ => write!(f, "{}年", self.0),
        }?;
        if self.1 {
            write!(f, " {}", TranslateAdapter(&ChineseYear(self.0), language))
        } else {
            Ok(())
        }
    }
}

impl Translate for Month {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        if language == English {
            self.short_translate(language, f)
        } else {
            write!(f, "{}月", ShortTranslateAdapter(self, language))
        }
    }
}

impl ShortTranslate for Month {
    fn short_translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match language {
                English => self.name(),
                _ => match self {
                    January => "一",
                    February => "二",
                    March => "三",
                    April => "四",
                    May => "五",
                    June => "六",
                    July => "七",
                    August => "八",
                    September => "九",
                    October => "十",
                    November => "十一",
                    December => "十二",
                },
            }
        )
    }
}

impl Translate for Weekday {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        match language {
            English => write!(
                f,
                "{}",
                match self {
                    Sun => "Sunday",
                    Mon => "Monday",
                    Tue => "Tuesday",
                    Wed => "Wednesday",
                    Thu => "Thursday",
                    Fri => "Friday",
                    Sat => "Saturday",
                }
            ),
            _ => write!(f, "星期{}", ShortTranslateAdapter(self, language)),
        }
    }
}

impl ShortTranslate for Weekday {
    fn short_translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match language {
                English => match self {
                    Sun => "Sun",
                    Mon => "Mon",
                    Tue => "Tue",
                    Wed => "Wed",
                    Thu => "Thu",
                    Fri => "Fri",
                    Sat => "Sat",
                },
                _ => match self {
                    Sun => "日",
                    Mon => "一",
                    Tue => "二",
                    Wed => "三",
                    Thu => "四",
                    Fri => "五",
                    Sat => "六",
                },
            }
        )
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
