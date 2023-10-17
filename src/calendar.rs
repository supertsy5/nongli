use anstyle::{AnsiColor, Color, Style};
use std::fmt::{Display, Formatter, Result as FmtResult};

use chrono::{
    Datelike, Month, NaiveDate,
    Weekday::{self, Mon, Sun},
};

use crate::{
    chinese_date::{ChineseDate, ChineseDay, ChineseMonth},
    days_of_month, is_weekend,
    language::{
        Language::{self, *},
        MonthTitle, ShortTranslateAdapter, Translate, TranslateAdapter, YearTitle,
    },
};

use Alignment::*;

pub const CELL_WIDTH_WITH_CHINESE: usize = 6;
pub const CELL_WIDTH_WITHOUT_CHINESE: usize = 4;
pub const WHITE: Color = Color::Ansi(AnsiColor::White);
pub const NEW_MONTH_COLOR: Color = Color::Ansi(AnsiColor::Blue);
pub const WEEKEND_COLOR: Color = Color::Ansi(AnsiColor::Red);

#[derive(Clone, Copy, Debug)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct Aligned<T: AsRef<str>>(pub T, pub Alignment, pub usize);

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
pub struct QuadWeekLine(Options);

#[derive(Clone, Copy, Debug)]
pub struct BasicMonthCalendar {
    pub year: u16,
    pub month: Month,
    pub today: NaiveDate,
    pub options: Options,
}

#[derive(Clone, Copy, Debug)]
pub struct ListCalendar(pub BasicMonthCalendar);

#[derive(Clone, Copy, Debug)]
pub struct MonthCalendar(pub BasicMonthCalendar);

#[derive(Clone, Copy, Debug)]
pub struct BasicTripleCalendar(pub BasicMonthCalendar);

#[derive(Clone, Copy, Debug)]
pub struct BasicQuadCalendar(pub BasicMonthCalendar);

#[derive(Clone, Copy, Debug)]
pub struct TripleCalendar(pub BasicMonthCalendar);

#[derive(Clone, Copy, Debug)]
pub struct YearCalendar {
    pub year: u16,
    pub today: NaiveDate,
    pub options: Options,
    pub landscape: bool,
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

impl<T: AsRef<str>> Display for Aligned<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = self.0.as_ref();
        let width = rendered_width(s);
        if width >= self.2 {
            return write!(f, "{}", s);
        }
        let padding_spaces = self.2 - width;
        let padding_left = match self.1 {
            Left => 0,
            Center => padding_spaces / 2,
            Right => padding_spaces,
        };
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
            let centered = Aligned(
                ShortTranslateAdapter(&weekday, self.0.language).to_string(),
                Center,
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
                "{0}{1} {0}{1} {0}",
                WeekLine(self.0),
                Style::new().invert().render()
            )
        } else {
            write!(f, "{0} {0} {0}", WeekLine(self.0))
        }
    }
}

impl Display for QuadWeekLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let triple = TripleWeekLine(self.0);
        let single = WeekLine(self.0);
        if self.0.color {
            write!(f, "{}{} {}", triple, Style::new().invert().render(), single,)
        } else {
            write!(f, "{} {}", triple, single)
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
        let days = days_of_month(year, month);
        let highlight_today = options.highlight_today
            && year == today.year() as u16
            && month.number_from_month() == today.month();

        let weekday_of_1st = NaiveDate::from_ymd_opt(year as i32, month.number_from_month(), 1)
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
                let date =
                    NaiveDate::from_ymd_opt(year as i32, month.number_from_month(), day as u32)
                        .unwrap();
                if options.color {
                    let is_weekend = [Weekday::Sun, Weekday::Sat].contains(&date.weekday());
                    let style = if highlight_today && day == today.day() as u8 {
                        if is_weekend {
                            Style::new()
                                .fg_color(Some(WEEKEND_COLOR))
                                .bg_color(Some(WHITE))
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
                        NaiveDate::from_ymd_opt(year as i32, month.number_from_month(), day as u32)
                            .unwrap();
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
                        let mut style = Style::new();
                        if is_new_month {
                            style = style.fg_color(Some(NEW_MONTH_COLOR))
                        } else if is_weekend {
                            style = style.fg_color(Some(WEEKEND_COLOR))
                        };
                        if highlight_today && day == today.day() as u8 {
                            style = if is_new_month || is_weekend {
                                style.bg_color(Some(WHITE))
                            } else {
                                style.invert()
                            }
                        };
                        write!(
                            f,
                            "{}{}{}",
                            style.render(),
                            Aligned(&ch_day, Center, cell_width),
                            style.render_reset(),
                        )
                    } else {
                        write!(f, "{}", Aligned(&ch_day, Center, cell_width))
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
            Aligned(
                self.0.title().translate_to_string(options.language),
                Center,
                options.cell_width() * 7,
            ),
            WeekLine(options),
            self.0,
        )
    }
}

impl Display for ListCalendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let options = self.0.options;
        let highlight_today = options.highlight_today
            && self.0.year as i32 == self.0.today.year()
            && self.0.month.number_from_month() == self.0.today.month();
        writeln!(
            f,
            "{}:",
            TranslateAdapter(
                &MonthTitle(self.0.year, self.0.month, options.enable_chinese),
                self.0.options.language
            )
        )?;
        for day in 1..=days_of_month(self.0.year, self.0.month) {
            let date = NaiveDate::from_ymd_opt(
                self.0.year as i32,
                self.0.month.number_from_month(),
                day as u32,
            )
            .unwrap();
            let is_today = highlight_today && self.0.today.day() == day as u32;
            let weekday = date.weekday();
            let weekend = is_weekend(weekday);
            let weekday_string = weekday.translate_to_string(options.language);
            let mut style = Style::new();
            if options.color {
                if weekend {
                    if is_today {
                        style = style.bg_color(Some(WHITE));
                    }
                    style = style.fg_color(Some(WEEKEND_COLOR));
                } else if is_today {
                    style = style.invert();
                }
                write!(
                    f,
                    "{}{:<8}{:}",
                    style.render(),
                    day,
                    Aligned(weekday_string, Left, 12),
                )
            } else {
                write!(
                    f,
                    "{}{:<7}{}",
                    if is_today { '[' } else { ' ' },
                    day,
                    Aligned(weekday_string, Left, 12),
                )
            }?;
            if options.enable_chinese {
                if let Some(chinese_date) = ChineseDate::from_gregorian(&date) {
                    if options.color && chinese_date.day() == 1 {
                        write!(f, "{}", style.render_reset())?;
                        style = Style::new().fg_color(Some(NEW_MONTH_COLOR));
                        if is_today {
                            style = style.bg_color(Some(WHITE));
                        }
                        write!(f, "{}", style.render())?;
                    }
                    if options.language == English {
                        write!(
                            f,
                            "{:02}{}{:02}",
                            chinese_date.month(),
                            if chinese_date.leap() { '+' } else { '-' },
                            chinese_date.day(),
                        )
                    } else {
                        write!(
                            f,
                            "{}",
                            Aligned(
                                format!(
                                    "{}{}",
                                    TranslateAdapter(
                                        &chinese_date.chinese_month(),
                                        options.language
                                    ),
                                    TranslateAdapter(&chinese_date.chinese_day(), options.language),
                                ),
                                Left,
                                12,
                            )
                        )
                    }?;
                }
            }
            if !options.color && is_today {
                write!(f, "]")?;
            }
            writeln!(f, "{}", style.render_reset())?;
        }
        Ok(())
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

impl Display for BasicQuadCalendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let calendar1 = self.0.succ();
        let calendar2 = calendar1.succ();
        let calendar3 = calendar2.succ();
        let strings =
            [self.0, calendar1, calendar2, calendar3].map(|calendar| calendar.to_string());
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
            &separator,
            &strings[3],
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
            Aligned(
                c.title().translate_to_string(options.language),
                Center,
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
        let language = options.language;
        let cell_width = options.cell_width();
        let month_width = cell_width * 7;
        writeln!(
            f,
            "{}",
            Aligned(
                YearTitle(self.year, options.enable_chinese).translate_to_string(options.language),
                Center,
                if self.landscape {
                    cell_width * 28 + 3
                } else {
                    cell_width * 21 + 2
                },
            )
        )?;
        if self.landscape {
            for month in [January, May, September] {
                let month1 = month.succ();
                let month2 = month1.succ();
                let month3 = month2.succ();
                write!(
                    f,
                    "{} {} {} {}\n{}\n{}",
                    Aligned(month.translate_to_string(language), Center, month_width),
                    Aligned(month1.translate_to_string(language), Center, month_width),
                    Aligned(month2.translate_to_string(language), Center, month_width),
                    Aligned(month3.translate_to_string(language), Center, month_width),
                    QuadWeekLine(options),
                    BasicQuadCalendar(BasicMonthCalendar {
                        year: self.year,
                        month,
                        today: self.today,
                        options,
                    })
                )?;
            }
        } else {
            for month in [January, April, July, October] {
                let month1 = month.succ();
                let month2 = month1.succ();
                write!(
                    f,
                    "{} {} {}\n{}\n{}",
                    Aligned(month.translate_to_string(language), Center, month_width),
                    Aligned(month1.translate_to_string(language), Center, month_width),
                    Aligned(month2.translate_to_string(language), Center, month_width),
                    TripleWeekLine(options),
                    BasicTripleCalendar(BasicMonthCalendar {
                        year: self.year,
                        month,
                        today: self.today,
                        options,
                    })
                )?;
            }
        }
        Ok(())
    }
}
