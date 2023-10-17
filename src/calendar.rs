use anstyle::{AnsiColor, Color, Style, Reset};
use std::fmt::{Display, Formatter, Result as FmtResult, Write};

use chrono::{
    Datelike, Month, NaiveDate,
    Weekday::{self, Mon, Sun},
};

use crate::{
    chinese_date::ChineseDate,
    language::{
        ChineseDay, ChineseMonth, ChineseYear,
        Language::{self, *},
        ShortTranslateAdapter, Translate, TranslateAdapter,
    },
};

pub const CELL_WIDTH: usize = 6;
pub const WEEKEND_COLOR: Color = Color::Ansi(AnsiColor::Red);

#[derive(Clone, Copy, Debug)]
pub struct Centered<'a>(pub &'a str, pub usize);
#[derive(Clone, Copy, Debug)]
pub struct ZipByLine<'a>(pub &'a [&'a str]);

#[derive(Clone, Copy, Debug)]
pub struct MonthCalendar {
    pub year: u16,
    pub month: Month,
    pub today: NaiveDate,
    pub options: Options,
}

#[derive(Clone, Copy, Debug)]
pub struct TripleCalendar(pub MonthCalendar);

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub language: Language,
    pub enable_chinese: bool,
    pub start_on_monday: bool,
    pub highlight_today: bool,
    pub color: bool,
}

fn rendered_width(s: &str) -> usize {
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

impl MonthCalendar {
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

impl<'a> Display for Centered<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = rendered_width(self.0);
        if width >= self.1 {
            return write!(f, "{}", self.0);
        }
        let padding_spaces = self.1 - width;
        let padding_left = padding_spaces / 2;
        let padding_right = padding_spaces - padding_left;
        for _ in 0..padding_left {
            write!(f, " ")?;
        }
        write!(f, "{}", self.0)?;
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

pub fn write_week_line(options: &Options, f: &mut Formatter) -> FmtResult {
    for weekday in crate::iter::Weekdays(if options.start_on_monday { Mon } else { Sun }).take(7) {
        let adapter = ShortTranslateAdapter(&weekday, options.language).to_string();
        let centered = Centered(&adapter, CELL_WIDTH);
        if options.color {
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

pub fn write_basic_month_calendar(
    year: u16,
    month: Month,
    today: &NaiveDate,
    options: &Options,
    f: &mut Formatter,
) -> FmtResult {
    let month = month.number_from_month() as u8;
    let days = crate::days_of_month(year, month);
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
            for _ in 0..leading_spaces as usize * CELL_WIDTH {
                write!(f, " ")?;
            }
        }
        for day in start_day..end_day {
            let date = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap();
            if options.color {
                let is_weekend = [Weekday::Sun, Weekday::Sat].contains(&date.weekday());
                let style = if options.highlight_today && day == today.day() as u8 {
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
                    CELL_WIDTH
                )?;
            } else {
                #[allow(clippy::collapsible_if)]
                if options.highlight_today && day == today.day() as u8 {
                    write!(f, "[{day:^0$}]", CELL_WIDTH - 2)
                } else {
                    write!(f, "{day:^0$}", CELL_WIDTH)
                }?;
            }
        }
        if end_day == days + 1 {
            for _ in 0..trailing_spaces as usize * CELL_WIDTH {
                write!(f, " ")?;
            }
        }
        writeln!(f)?;
        if options.enable_chinese {
            if start_day == 1 {
                for _ in 0..leading_spaces as usize * CELL_WIDTH {
                    write!(f, " ")?;
                }
            }
            for day in start_day..end_day {
                let date = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap();
                let ch_day = ChineseDate::from_gregorian(&date)
                    .map(|ch_date| {
                        let ch_day = ch_date.day();
                        if ch_day == 1 {
                            let ch_month =
                                ChineseMonth::new(ch_date.month(), ch_date.leap()).unwrap();
                            if options.language == English {
                                format!("(M{})", TranslateAdapter(&ch_month, English))
                            } else {
                                ch_month.translate_to_string(options.language)
                            }
                        } else {
                            let ch_day = ChineseDay::new(ch_day).unwrap();
                            if options.language == English {
                                format!("({})", TranslateAdapter(&ch_day, English))
                            } else {
                                ch_day.translate_to_string(options.language)
                            }
                        }
                    })
                    .unwrap_or_default();
                if options.color {
                    let is_weekend = [Weekday::Sun, Weekday::Sat].contains(&date.weekday());
                    let style = if options.highlight_today && day == today.day() as u8 {
                        if is_weekend {
                            Style::new().fg_color(Some(WEEKEND_COLOR))
                        } else {
                            Style::new().invert()
                        }
                    } else if is_weekend {
                        Style::new().fg_color(Some(WEEKEND_COLOR))
                    } else {
                        Style::new()
                    };
                    write!(
                        f,
                        "{}{}{}",
                        style.render(),
                        Centered(&ch_day, CELL_WIDTH),
                        style.render_reset(),
                    )
                } else {
                    write!(f, "{}", Centered(&ch_day, CELL_WIDTH))
                }?;
            }
            if end_day == days + 1 {
                for _ in 0..trailing_spaces as usize * CELL_WIDTH {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        start_day = end_day;
        week_size = 7;
        lines += 1;
    }
    for _ in 0..6 - lines {
        for _ in 0..CELL_WIDTH * 7 {
            write!(f, " ")?;
        }
        writeln!(f)?;
    }
    Ok(())
}

impl Display for MonthCalendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut title = crate::language::Title(self.year, self.month)
            .translate_to_string(self.options.language);
        if self.options.enable_chinese {
            title.write_fmt(format_args!(
                " {}",
                TranslateAdapter(&ChineseYear(self.year), self.options.language)
            ))?;
        }
        writeln!(f, "{}", Centered(&title, CELL_WIDTH * 7))?;
        write_week_line(&self.options, f)?;
        writeln!(f)?;
        write_basic_month_calendar(self.year, self.month, &self.today, &self.options, f)
    }
}

impl Display for TripleCalendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let calendar1 = self.0.succ();
        let calendar2 = calendar1.succ();
        let strings = [
            self.0.to_string(),
            calendar1.to_string(),
            calendar2.to_string(),
        ];
        let colored_separator;
        let separator = if self.0.options.color {
            colored_separator = format!(
                " \n{}|{}\n \n \n \n \n \n ",
                Style::new().invert().render(),
                Reset.render(),
            );
            &colored_separator
        } else {
            " \n|\n \n \n \n \n \n "
        };  
        let strs = [strings[0].as_str(), separator, &strings[1], separator, &strings[2]];
        write!(f, "{}", ZipByLine(&strs))
    }
}
