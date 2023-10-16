use std::fmt::{Formatter, Result as FmtResult, Display};

use chrono::{Month::{self, *}, Weekday::{self, *}};
use Language::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    English,
    Chinese,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Title(pub i32, pub Month, pub Language);

#[derive(Clone, Copy, Debug)]
pub struct TranslateAdapter<'a, T: Translate>(pub &'a T, pub Language);

#[derive(Clone, Copy, Debug)]
pub struct ShortTranslateAdapter<'a, T: ShortTranslate>(pub &'a T, pub Language);

pub trait Translate {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult;
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

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.2 {
            English => write!(f, "{} {}", self.1.name(), self.0),
            Chinese => write!(f, "{}年  {}", self.0, TranslateAdapter(&self.1, Chinese)),
        }
    }
}

impl Translate for Month {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        write!(f, "{}月", ShortTranslateAdapter(self, language))
    }
}

impl ShortTranslate for Month {
    fn short_translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match language {
            English => self.name(),
            Chinese => match self {
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
        })
    }
}

impl Translate for Weekday {
    fn translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        match language {
            English => write!(f, "{}", match self {
                Sun => "Sunday",
                Mon => "Monday",
                Tue => "Tuesday",
                Wed => "Wednesday",
                Thu => "Thursday",
                Fri => "Friday",
                Sat => "Saturday",
            }),
            Chinese => write!(f, "{}", ShortTranslateAdapter(self, language)),
        }
    }
}

impl ShortTranslate for Weekday {
    fn short_translate(&self, language: Language, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match language {
            English => match self {
                Sun => "Sun",
                Mon => "Mon",
                Tue => "Tue",
                Wed => "Wed",
                Thu => "Thu",
                Fri => "Fri",
                Sat => "Sat",
            },
            Chinese => match self {
                Sun => "日",
                Mon => "一",
                Tue => "二",
                Wed => "三",
                Thu => "四",
                Fri => "五",
                Sat => "六",
            }
        })
    }
}
