use chrono::{Month, NaiveDate};
use crate::language::Language;

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub language: Language,
    pub enable_chinese: bool,
    pub start_on_monday: bool,
    pub highlight_today: bool,
    pub color: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Calendar {
    pub year: u16,
    pub month: Month,
    pub today: NaiveDate,
    pub options: Options,
}

impl Calendar {
    pub fn pred(mut self) -> Self {
        if self.month == Month::January {
            self.year -= 1;
        }
        self.month = self.month.pred();
        self
    }
    pub fn succ(mut self) -> Self {
        if self.month == Month::December {
            self.year += 1;
        }
        self.month = self.month.succ();
        self
    }
}