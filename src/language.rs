use chrono::{Month, Weekday};
use Language::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    English,
    Chinese,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Title(pub i32, pub Month, pub Language);

pub trait Translate {
    fn translate(&self, language: Language) -> &'static str;
}

pub trait ShortTranslate {
    fn short_translate(&self, language: Language) -> &'static str;
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.2 {
            English => write!(f, "{} {}", self.1.name(), self.0),
            Chinese => write!(f, "{}年 {}", self.0, self.1.translate(Chinese)),
        }
    }
}

impl Translate for Month {
    fn translate(&self, language: Language) -> &'static str {
        use Month::*;
        match language {
            English => self.name(),
            Chinese => match self {
                January => "一月",
                February => "二月",
                March => "三月",
                April => "四月",
                May => "五月",
                June => "六月",
                July => "七月",
                August => "八月",
                September => "九月",
                October => "十月",
                November => "十一月",
                December => "十二月",
            },
        }
    }
}

impl ShortTranslate for Month {
    fn short_translate(&self, language: Language) -> &'static str {
        match language {
            English => &self.translate(English)[..3],
            Chinese => {
                let chinese = self.translate(Chinese);
                &chinese[..chinese.len() - 3]
            }
        }
    }
}

impl Translate for Weekday {
    fn translate(&self, language: Language) -> &'static str {
        use Weekday::*;
        match language {
            English => match self {
                Sun => "Sunday",
                Mon => "Monday",
                Tue => "Tuesday",
                Wed => "Wednesday",
                Thu => "Thursday",
                Fri => "Friday",
                Sat => "Saturday",
            },
            Chinese => match self {
                Sun => "星期日",
                Mon => "星期一",
                Tue => "星期二",
                Wed => "星期三",
                Thu => "星期四",
                Fri => "星期五",
                Sat => "星期六",
            },
        }
    }
}

impl ShortTranslate for Weekday {
    fn short_translate(&self, language: Language) -> &'static str {
        match language {
            English => &self.translate(English)[..3],
            Chinese => {
                let chinese = self.translate(Chinese);
                &chinese[chinese.len() - 3..]
            }
        }
    }
}
