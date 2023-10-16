use anstyle::{AnsiColor, Color, Style};
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

pub struct Centered<'a>(pub &'a str, pub usize);

pub struct MonthCalendar {
    pub year: u16,
    pub month: Month,
    pub today: NaiveDate,
    pub options: Options,
}

pub struct Options {
    pub language: Language,
    pub enable_chinese: bool,
    pub start_on_monday: bool,
    pub highlight_today: bool,
    pub color: bool,
}

fn printed_width(s: &str) -> usize {
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

impl<'a> std::fmt::Display for Centered<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = printed_width(self.0);
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
    let weekday_of_1st = NaiveDate::from_ymd_opt(year as i32, month as u32, 1)
        .unwrap()
        .weekday();
    let mut spaces = if options.start_on_monday {
        weekday_of_1st.num_days_from_monday()
    } else {
        weekday_of_1st.num_days_from_sunday()
    } as usize;
    let mut week_size = 7 - spaces as u8;
    let mut start_day = 1u8;
    let days = crate::days_of_month(year, month);
    while start_day <= days {
        let end_day = (start_day + week_size).min(days + 1);
        for _ in 0..spaces * CELL_WIDTH {
            write!(f, " ")?;
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
        writeln!(f)?;
        if options.enable_chinese {
            for _ in 0..spaces * CELL_WIDTH {
                write!(f, " ")?;
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
            writeln!(f)?;
        }
        start_day = end_day;
        week_size = 7;
        spaces = 0;
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
