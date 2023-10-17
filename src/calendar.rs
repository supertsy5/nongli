use anstyle::{AnsiColor, Color, Style};
use std::fmt::{Display, Formatter, Result as FmtResult};

use chrono::{
    Datelike, Month, NaiveDate,
    Weekday::{self, Mon, Sun},
};

use crate::{
    chinese_date::ChineseDate,
    language::{
        ChineseDay, ChineseMonth,
        Language::{self, *},
        MonthTitle, ShortTranslateAdapter, Translate, TranslateAdapter, YearTitle,
    },
};

pub const CELL_WIDTH_WITH_CHINESE: usize = 6;
pub const CELL_WIDTH_WITHOUT_CHINESE: usize = 4;
pub const NEW_MONTH_COLOR: Color = Color::Ansi(AnsiColor::Blue);
pub const WEEKEND_COLOR: Color = Color::Ansi(AnsiColor::Red);

#[derive(Clone, Copy, Debug)]
pub struct Centered<T: AsRef<str>>(pub T, pub usize);

pub fn rendered_width(s: &str) -> usize {
    s.chars()
        .map(|ch| {
            if (0x4e00..=0x9fff).contains(&(ch as u32)) {
                2
            } else {
                1
            }
        })
        .sum()
}

#[derive(Clone, Copy, Debug)]
pub struct ZipByLine<'a>(pub &'a [&'a str]);

#[derive(Clone, Copy, Debug)]
pub struct WeekLine(Options);

#[derive(Clone, Copy, Debug)]
pub struct TripleWeekLine(Options);

#[derive(Clone, Copy, Debug)]
pub struct BasicMonthCalendar {
    pub year: u16,
    pub month: Month,
    pub today: NaiveDate,
    pub options: Options,
}

#[derive(Clone, Copy, Debug)]
pub struct MonthCalendar(pub BasicMonthCalendar);

#[derive(Clone, Copy, Debug)]
pub struct BasicTripleCalendar(pub BasicMonthCalendar);

#[derive(Clone, Copy, Debug)]
pub struct TripleCalendar(pub BasicMonthCalendar);

#[derive(Clone, Copy, Debug)]
pub struct YearCalendar {
    pub year: u16,
    pub today: NaiveDate,
    pub options: Options,
}

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub language: Language,
    pub enable_chinese: bool,
    pub start_on_monday: bool,
    pub highlight_today: bool,
    pub color: bool,
}

impl Options {
    pub fn cell_width(&self) -> usize {
        if self.enable_chinese {
            CELL_WIDTH_WITH_CHINESE
        } else {
            CELL_WIDTH_WITHOUT_CHINESE
        }
    }
}

impl BasicMonthCalendar {
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
    pub fn title(&self) -> MonthTitle {
        MonthTitle(self.year, self.month, self.options.enable_chinese)
    }
}

impl<T: AsRef<str>> Display for Centered<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = self.0.as_ref();
        let width = rendered_width(s);
        if width >= self.1 {
            return write!(f, "{}", s);
        }
        let padding_spaces = self.1 - width;
        let padding_left = padding_spaces / 2;
        let padding_right = padding_spaces - padding_left;
        for _ in 0..padding_left {
            write!(f, " ")?;
        }
        write!(f, "{}", s)?;
        for _ in 0..padding_right {
            write!(f, " ")?;
        }
        Ok(())
    }
}

impl<'a> Display for ZipByLine<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut iterators = self.0.iter().map(|s| s.lines()).collect::<Vec<_>>();
        loop {
            let mut go_ahead = false;

            for iterator in &mut iterators {
                if let Some(s) = iterator.next() {
                    f.write_str(s)?;
                    go_ahead = true;
                }
            }
            if !go_ahead {
                return Ok(());
            }
            writeln!(f)?;
        }
    }
}

impl Display for WeekLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for weekday in crate::iter::Weekdays(if self.0.start_on_monday { Mon } else { Sun }).take(7)
        {
            let centered = Centered(
                ShortTranslateAdapter(&weekday, self.0.language).to_string(),
                self.0.cell_width(),
            );
            if self.0.color {
                let style = if crate::is_weekend(weekday) {
                    Style::new().bg_color(Some(WEEKEND_COLOR))
                } else {
                    Style::new()
                }
                .invert();
                write!(f, "{}{}{}", style.render(), centered, style.render_reset())
            } else {
                write!(f, "{centered}")
            }?;
        }
        Ok(())
    }
}

impl Display for TripleWeekLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.0.color {
            write!(
                f,
                "{0}{1}|{0}{1}|{0}",
                WeekLine(self.0),
                Style::new().invert().render()
            )
        } else {
            write!(f, "{0}|{0}|{0}", WeekLine(self.0))
        }
    }
}

impl Display for BasicMonthCalendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let BasicMonthCalendar {
            year,
            month,
            today,
            options,
        } = *self;
        let cell_width = self.options.cell_width();
        let month = month.number_from_month() as u8;
        let days = crate::days_of_month(year, month);
        let highlight_today =
            options.highlight_today && year == today.year() as u16 && month == today.month() as u8;

        let weekday_of_1st = NaiveDate::from_ymd_opt(year as i32, month as u32, 1)
            .unwrap()
            .weekday();

        let leading_spaces = if options.start_on_monday {
            weekday_of_1st.num_days_from_monday()
        } else {
            weekday_of_1st.num_days_from_sunday()
        } as u8;

        let trailing_spaces = (7 - (days + leading_spaces) % 7) % 7;

        let mut week_size = 7 - leading_spaces as u8;
        let mut start_day = 1u8;
        let mut lines = 0u8;

        while start_day <= days {
            let end_day = (start_day + week_size).min(days + 1);
            if start_day == 1 {
                for _ in 0..leading_spaces as usize * cell_width {
                    write!(f, " ")?;
                }
            }
            for day in start_day..end_day {
                let date = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap();
                if options.color {
                    let is_weekend = [Weekday::Sun, Weekday::Sat].contains(&date.weekday());
                    let style = if highlight_today && day == today.day() as u8 {
                        if is_weekend {
                            Style::new().bg_color(Some(WEEKEND_COLOR))
                        } else {
                            Style::new()
                        }
                        .invert()
                    } else if is_weekend {
                        Style::new().fg_color(Some(WEEKEND_COLOR))
                    } else {
                        Style::new()
                    };
                    write!(
                        f,
                        "{}{day:^2$}{}",
                        style.render(),
                        style.render_reset(),
                        cell_width,
                    )?;
                } else {
                    #[allow(clippy::collapsible_if)]
                    if highlight_today && day == today.day() as u8 {
                        write!(f, "[{day:^0$}]", cell_width - 2)
                    } else {
                        write!(f, "{day:^0$}", cell_width)
                    }?;
                }
            }
            if end_day == days + 1 {
                for _ in 0..trailing_spaces as usize * cell_width {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
            if options.enable_chinese {
                if start_day == 1 {
                    for _ in 0..leading_spaces as usize * cell_width {
                        write!(f, " ")?;
                    }
                }
                for day in start_day..end_day {
                    let date =
                        NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap();
                    let (ch_day, is_new_month) = ChineseDate::from_gregorian(&date)
                        .map(|ch_date| {
                            let ch_day = ch_date.day();
                            if ch_day == 1 {
                                let ch_month =
                                    ChineseMonth::new(ch_date.month(), ch_date.leap()).unwrap();
                                (
                                    if options.language == English {
                                        format!("(M{})", TranslateAdapter(&ch_month, English))
                                    } else {
                                        ch_month.translate_to_string(options.language)
                                    },
                                    true,
                                )
                            } else {
                                let ch_day = ChineseDay::new(ch_day).unwrap();
                                (
                                    if options.language == English {
                                        format!("({})", TranslateAdapter(&ch_day, English))
                                    } else {
                                        ch_day.translate_to_string(options.language)
                                    },
                                    false,
                                )
                            }
                        })
                        .unwrap_or_default();
                    if options.color {
                        let is_weekend = [Weekday::Sun, Weekday::Sat].contains(&date.weekday());
                        let style = if highlight_today && day == today.day() as u8 {
                            if is_new_month {
                                Style::new().fg_color(Some(NEW_MONTH_COLOR))
                            } else if is_weekend {
                                Style::new().fg_color(Some(WEEKEND_COLOR))
                            } else {
                                Style::new().invert()
                            }
                        } else if is_new_month {
                            Style::new().fg_color(Some(NEW_MONTH_COLOR))
                        } else if is_weekend {
                            Style::new().fg_color(Some(WEEKEND_COLOR))
                        } else {
                            Style::new()
                        };
                        write!(
                            f,
                            "{}{}{}",
                            style.render(),
                            Centered(&ch_day, cell_width),
                            style.render_reset(),
                        )
                    } else {
                        write!(f, "{}", Centered(&ch_day, cell_width))
                    }?;
                }
                if end_day == days + 1 {
                    for _ in 0..trailing_spaces as usize * cell_width {
                        write!(f, " ")?;
                    }
                }
                writeln!(f)?;
            }
            start_day = end_day;
            week_size = 7;
            lines += 1;
        }
        for _ in 0..(6 - lines) * if options.enable_chinese { 2 } else { 1 } {
            for _ in 0..cell_width * 7 {
                write!(f, " ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for MonthCalendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let options = self.0.options;
        write!(
            f,
            "{}\n{}\n{}",
            Centered(
                self.0.title().translate_to_string(options.language),
                options.cell_width() * 7
            ),
            WeekLine(options),
            self.0,
        )
    }
}

impl Display for BasicTripleCalendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let calendar1 = self.0.succ();
        let calendar2 = calendar1.succ();
        let strings = [self.0, calendar1, calendar2].map(|calendar| calendar.to_string());
        let mut separator = String::new();
        for _ in 0..if self.0.options.enable_chinese { 12 } else { 6 } {
            separator.push_str(" \n");
        }
        let strs = [
            strings[0].as_str(),
            &separator,
            &strings[1],
            &separator,
            &strings[2],
        ];
        write!(f, "{}", ZipByLine(&strs))
    }
}

impl Display for TripleCalendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let options = self.0.options;
        let cell_width = options.cell_width();
        let calendar1 = self.0.succ();
        let calendar2 = calendar1.succ();
        let titles = [self.0, calendar1, calendar2].map(|c| {
            Centered(
                c.title().translate_to_string(options.language),
                cell_width * 7,
            )
        });
        write!(
            f,
            "{} {} {}\n{}\n{}",
            titles[0],
            titles[1],
            titles[2],
            TripleWeekLine(options),
            BasicTripleCalendar(self.0),
        )
    }
}

impl Display for YearCalendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Month::*;
        let options = self.options;
        writeln!(
            f,
            "{}",
            Centered(
                YearTitle(self.year, options.enable_chinese).translate_to_string(options.language),
                options.cell_width() * 21 + 2,
            )
        )?;
        for month in [January, April, July, October] {
            write!(
                f,
                "{}\n{}",
                TripleWeekLine(options),
                BasicTripleCalendar(BasicMonthCalendar {
                    year: self.year,
                    month,
                    today: self.today,
                    options,
                })
            )?;
        }
        Ok(())
    }
}
