use anstyle::{AnsiColor, Color, Reset, Style};
use std::fmt::{Display, Formatter, Result as FmtResult};

use chrono::{
    Datelike, Month, NaiveDate,
    Weekday::{Mon, Sun},
};

use crate::{
    calendar::{Calendar, Options},
    chinese_date::ChineseDate,
    days_of_month,
    festivals::Festival,
    is_weekend,
    iter::Weekdays,
    language::{Language::*, MonthTitle, ShortTranslate, StaticTranslate, Translate, YearTitle},
    SolarTerm,
};

use Alignment::*;

pub const CELL_WIDTH_WITH_CHINESE: usize = 6;
pub const CELL_WIDTH_WITHOUT_CHINESE: usize = 4;
pub const WHITE: Color = Color::Ansi(AnsiColor::BrightWhite);
pub const WEEKEND_COLOR: Color = Color::Ansi(AnsiColor::Red);
pub const FESTIVAL_COLOR: Color = Color::Ansi(AnsiColor::Red);
pub const NEW_MONTH_COLOR: Color = Color::Ansi(AnsiColor::Blue);
pub const SOLAR_TERM_COLOR: Color = Color::Ansi(AnsiColor::Green);

#[derive(Clone, Copy, Debug)]
pub enum Alignment {
    Left,
    Center,
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
pub struct BasicMonthCalendar(pub Calendar);

#[derive(Clone, Copy, Debug)]
pub struct ListCalendar(pub Calendar);

#[derive(Clone, Copy, Debug)]
pub struct MonthCalendar(pub Calendar);

#[derive(Clone, Copy, Debug)]
pub struct BasicTripleCalendar(pub Calendar);

#[derive(Clone, Copy, Debug)]
pub struct BasicQuadCalendar(pub Calendar);

#[derive(Clone, Copy, Debug)]
pub struct TripleCalendar(pub Calendar);

#[derive(Clone, Copy, Debug)]
pub struct YearCalendar {
    year: i32,
    pub today: Option<NaiveDate>,
    pub options: Options,
    pub landscape: bool,
}

fn cell_width(options: &Options) -> usize {
    if options.enable_chinese {
        CELL_WIDTH_WITH_CHINESE
    } else {
        CELL_WIDTH_WITHOUT_CHINESE
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
        if self.0.week_number {
            if self.0.color {
                print!("{}", Style::new().invert());
            }
            print!("    ");
        }
        for weekday in Weekdays(if self.0.start_on_monday { Mon } else { Sun }).take(7) {
            let centered = Aligned(
                weekday
                    .short()
                    .translate_adapter(self.0.language)
                    .to_string(),
                Center,
                cell_width(&self.0),
            );
            if self.0.color {
                let style = if is_weekend(weekday) {
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
        let options = self.0.options;
        let cell_width = cell_width(&options);
        let mut lines = 0u8;
        for (week, line) in self.0.iter() {
            if self.0.options.week_number {
                if self.0.options.color {
                    write!(f, "{} {week:02} {}", Style::new().invert(), Reset)?;
                } else {
                    write!(f, "|{week:02}|")?;
                }
            }
            for cell in &line {
                if let Some(cell) = cell {
                    if options.color {
                        let style = if cell.today {
                            if cell.weekend {
                                Style::new()
                                    .fg_color(Some(WEEKEND_COLOR))
                                    .bg_color(Some(WHITE))
                            } else {
                                Style::new().invert()
                            }
                        } else if cell.weekend {
                            Style::new().fg_color(Some(WEEKEND_COLOR))
                        } else {
                            Style::new()
                        };
                        write!(
                            f,
                            "{}{:^cell_width$}{}",
                            style.render(),
                            cell.date.day(),
                            style.render_reset(),
                        )?;
                    } else {
                        #[allow(clippy::collapsible_if)]
                        if cell.today {
                            write!(f, "[{0:^1$}]", cell.date.day(), cell_width - 2)
                        } else {
                            write!(f, "{0:^1$}", cell.date.day(), cell_width)
                        }?;
                    }
                } else {
                    for _ in 0..cell_width {
                        write!(f, " ")?;
                    }
                }
            }
            writeln!(f)?;
            if options.enable_chinese {
                let language = options.language;
                for cell in line {
                    if let Some(cell) = cell {
                        let (string, color) = cell
                            .chinese_date
                            .map(|ch_date| {
                                if let Some(festival) = cell.festival {
                                    (
                                        festival.short().translate_to_string(language),
                                        Some(FESTIVAL_COLOR),
                                    )
                                } else if let Some(solar_term) = cell.solar_term {
                                    (
                                        solar_term.short().translate_to_string(language),
                                        Some(SOLAR_TERM_COLOR),
                                    )
                                } else {
                                    (
                                        ch_date.short().translate_to_string(language),
                                        (ch_date.day() == 1).then_some(NEW_MONTH_COLOR),
                                    )
                                }
                            })
                            .unwrap_or_default();
                        if options.color {
                            let mut style = Style::new();
                            if let Some(color) = color {
                                style = style.fg_color(Some(color))
                            } else if cell.weekend {
                                style = style.fg_color(Some(WEEKEND_COLOR))
                            };
                            if cell.today {
                                style = if color.is_some() || cell.weekend {
                                    style.bg_color(Some(WHITE))
                                } else {
                                    style.invert()
                                }
                            };
                            write!(
                                f,
                                "{}{}{}",
                                style.render(),
                                Aligned(&string, Center, cell_width),
                                style.render_reset(),
                            )
                        } else {
                            write!(f, "{}", Aligned(&string, Center, cell_width))
                        }?;
                    } else {
                        for _ in 0..cell_width {
                            write!(f, " ")?;
                        }
                    }
                }
                writeln!(f)?;
            }
            lines += 1;
        }
        while lines < 6 {
            let spaces = cell_width * 7 + if self.0.options.week_number { 4 } else { 0 };
            for _ in 0..spaces {
                write!(f, " ")?;
            }
            writeln!(f)?;
            if options.enable_chinese {
                for _ in 0..spaces {
                    write!(f, " ")?;
                }
                writeln!(f)?;
            }
            lines += 1;
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
                MonthTitle::from(self.0).translate_to_string(options.language),
                Center,
                cell_width(&options) * 7 + if self.0.options.week_number { 4 } else { 0 },
            ),
            WeekLine(options),
            BasicMonthCalendar(self.0),
        )
    }
}

impl Display for ListCalendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let options = self.0.options;
        let language = options.language;
        let today_day = self
            .0
            .today
            .and_then(|today| (self.0.year() == today.year()).then(|| today.day()));
        writeln!(
            f,
            "{}:",
            MonthTitle {
                year: self.0.year(),
                month: self.0.month,
                enable_chinese: options.enable_chinese,
            }
            .translate_adapter(language)
        )?;
        for day in 1..=days_of_month(self.0.year(), self.0.month) {
            let date = NaiveDate::from_ymd_opt(
                self.0.year(),
                self.0.month.number_from_month(),
                day as u32,
            )
            .unwrap();
            let is_today = today_day.is_some_and(|today| day == today as u8);
            let weekday = date.weekday();
            let weekend = is_weekend(weekday);
            let weekday_string = weekday.translate_to_string(language);
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
                    "{}{:4}    {:}",
                    style.render(),
                    day,
                    Aligned(weekday_string, Left, 12),
                )
            } else {
                write!(
                    f,
                    "{}{:3}    {}",
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
                    if language == English {
                        write!(
                            f,
                            "{}, {}",
                            chinese_date.chinese_day().translate_adapter(language),
                            chinese_date.chinese_month().translate_adapter(language),
                        )
                    } else {
                        write!(
                            f,
                            "{}",
                            Aligned(
                                format!(
                                    "{}{}",
                                    chinese_date.chinese_month().translate_adapter(language),
                                    chinese_date.chinese_day().translate_adapter(language),
                                ),
                                Left,
                                10,
                            )
                        )
                    }?;
                    if let Some(festival) = Festival::from_chinese_date(chinese_date) {
                        if options.color {
                            if is_today {
                                style = Style::new()
                                    .bg_color(Some(WHITE))
                                    .fg_color(Some(FESTIVAL_COLOR))
                            } else {
                                style = style.fg_color(Some(FESTIVAL_COLOR))
                            }
                        }
                        write!(f, "  {}{}", style.render_reset(), style.render())?;
                        if language == English {
                            write!(f, "{:12}", festival.static_translate(language))
                        } else {
                            festival.translate(language, f)
                        }?;
                    }
                }
                if let Some(solar_term) = SolarTerm::from_date(&date) {
                    if options.color {
                        if is_today {
                            style = Style::new()
                                .bg_color(Some(WHITE))
                                .fg_color(Some(SOLAR_TERM_COLOR))
                        } else {
                            style = style.fg_color(Some(SOLAR_TERM_COLOR))
                        }
                    }
                    write!(f, "  {}{}", style.render_reset(), style.render())?;
                    if language == English {
                        write!(f, "{:12}", solar_term.static_translate(language))
                    } else {
                        solar_term.translate(language, f)
                    }
                } else {
                    write!(f, "{:1$}", "", if language == English { 14 } else { 6 })
                }?;
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
        let calendar2 = calendar1.and_then(Calendar::succ);
        let strings = [Some(self.0), calendar1, calendar2].map(|calendar| {
            calendar.map_or_else(String::new, |calendar| {
                BasicMonthCalendar(calendar).to_string()
            })
        });
        let mut separator = String::new();
        for _ in 0..if self.0.options.enable_chinese { 12 } else { 6 } {
            separator.push_str(" \n");
        }
        let strs = [strings[0].as_str(), &separator, &strings[1], &separator, &strings[2]];
        write!(f, "{}", ZipByLine(&strs))
    }
}

impl Display for BasicQuadCalendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let calendar1 = self.0.succ();
        let calendar2 = calendar1.and_then(Calendar::succ);
        let calendar3 = calendar2.and_then(Calendar::succ);
        let strings = [Some(self.0), calendar1, calendar2, calendar3].map(|calendar| {
            calendar.map_or_else(String::new, |calendar| {
                BasicMonthCalendar(calendar).to_string()
            })
        });
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
        let cell_width = cell_width(&options);
        let calendar1 = self.0.succ();
        let calendar2 = calendar1.and_then(Calendar::succ);
        let titles = [Some(self.0), calendar1, calendar2].map(|calendar| {
            Aligned(
                calendar.map_or_else(String::new, |calendar| {
                    MonthTitle::from(calendar).translate_to_string(options.language)
                }),
                Center,
                cell_width * 7 + if self.0.options.week_number { 4 } else { 0 },
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

impl YearCalendar {
    pub fn new(
        year: i32, today: Option<NaiveDate>, options: Options, landscape: bool,
    ) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, 1, 1).map(|_| Self {
            year,
            today,
            options,
            landscape,
        })
    }
}

impl Display for YearCalendar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Month::*;
        let options = self.options;
        let language = options.language;
        let cell_width = cell_width(&options);
        let month_width = cell_width * 7 + if self.options.week_number { 4 } else { 0 };
        writeln!(
            f,
            "{}",
            Aligned(
                YearTitle {
                    year: self.year,
                    enable_chinese: options.enable_chinese,
                }
                .translate_to_string(language),
                Center,
                if self.landscape {
                    month_width * 4 + 3
                } else {
                    month_width * 3 + 2
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
                    BasicQuadCalendar(
                        Calendar::new(self.year, month, self.today, options).unwrap()
                    )
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
                    BasicTripleCalendar(
                        Calendar::new(self.year, month, self.today, options).unwrap()
                    )
                )?;
            }
        }
        Ok(())
    }
}
