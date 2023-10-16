use std::io::IsTerminal;

use anstyle::{AnsiColor, Color, Reset, Style};
use chrono::{
    Datelike, Month, NaiveDate,
    Weekday::{self, *},
};
use clap::{arg, value_parser};
use nongli::{
    chinese_date::ChineseDate,
    language::{
        ChineseDay, ChineseMonth, Language::*, ShortTranslateAdapter, Translate, TranslateAdapter,
    },
};

pub const CELL_WIDTH: usize = 6;
pub const WEEKEND_COLOR: Color = Color::Ansi(AnsiColor::Red);

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

struct Centered<'a>(&'a str, usize);

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

fn main() {
    let matches = clap::command!()
        .arg(arg!(-C --chinese "Enable Chinese calendar"))
        .arg(arg!(-c --"no-chinese" "Disable Chinese calendar").conflicts_with("chinese"))
        .arg(arg!(-M --"start-on-monday" "Start on monday"))
        .arg(arg!(-n --"no-highlight-today" "Don't highlight today"))
        .arg(
            arg!(--color <color> "Whether to enable colors")
                .value_parser(["always", "auto", "never"]),
        )
        .arg(arg!(-y --year <year> "Year").value_parser(value_parser!(u16).range(1900..=2100)))
        .arg(
            arg!(-m --month <month> "Month, in number")
                .value_parser(value_parser!(u8).range(1..=12)),
        )
        .get_matches();

    let language = if std::env::var("LANG").is_ok_and(|lang| lang.starts_with("zh")) {
        ChineseSimplified
    } else {
        English
    };
    let start_on_monday = matches.get_flag("start-on-monday");
    let enable_chinese =
        matches.get_flag("chinese") || !(matches.get_flag("no-chinese") || language == English);

    let start_of_week = if start_on_monday { Mon } else { Sun };
    let color = match matches.get_one::<String>("color") {
        Some(s) => match s.to_ascii_lowercase().as_str() {
            "always" => true,
            "never" => false,
            _ => std::io::stdout().is_terminal(),
        },
        _ => std::io::stdout().is_terminal(),
    };

    let today = chrono::Local::now().date_naive();

    let year = matches
        .get_one::<u16>("year")
        .copied()
        .unwrap_or_else(|| today.year() as u16);
    let month = matches
        .get_one::<u8>("month")
        .copied()
        .unwrap_or_else(|| today.month() as u8);
    let highlight_today = !matches.get_flag("no-highlight-today")
        && today.year() == year as i32
        && today.month() == month as u32;

    let title = nongli::language::Title(year, Month::try_from(month as u8).unwrap())
        .translate_to_string(language);
    println!("{}", Centered(&title, CELL_WIDTH * 7));

    if color {
        print!("{}", Style::new().invert().render());
    }
    for weekday in nongli::iter::Weekdays(start_of_week).take(7) {
        if color {
            let style = if nongli::is_weekend(weekday) {
                Style::new().bg_color(Some(WEEKEND_COLOR))
            } else {
                Style::new()
            }
            .invert();
            print!("{}", style.render());
        }
        print!(
            "{}",
            Centered(
                &ShortTranslateAdapter(&weekday, language).to_string(),
                CELL_WIDTH
            )
        );
        if color {
            print!("{}", Reset.render());
        }
    }
    println!();

    let weekday_of_1st = NaiveDate::from_ymd_opt(year as i32, month as u32, 1)
        .unwrap()
        .weekday();
    let mut spaces = if start_on_monday {
        weekday_of_1st.num_days_from_monday()
    } else {
        weekday_of_1st.num_days_from_sunday()
    } as usize;
    let mut week_size = 7 - spaces as u8;
    let mut start_day = 1u8;
    let days = nongli::days_of_month(year, month);
    while start_day <= days {
        let end_day = (start_day + week_size).min(days + 1);
        for _ in 0..spaces * CELL_WIDTH {
            print!(" ");
        }
        for day in start_day..end_day {
            let date = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap();
            if color {
                let is_weekend = [Weekday::Sun, Weekday::Sat].contains(&date.weekday());
                let style = if highlight_today && day == today.day() as u8 {
                    if is_weekend {
                        Style::new().bg_color(Some(WEEKEND_COLOR))
                    } else {
                        Style::new()
                    }.invert()
                } else if is_weekend {
                    Style::new().fg_color(Some(WEEKEND_COLOR))
                } else {
                    Style::new()
                };
                print!(
                    "{}{day:^2$}{}",
                    style.render(),
                    style.render_reset(),
                    CELL_WIDTH
                );
            } else {
                #[allow(clippy::collapsible_if)]
                if highlight_today && day == today.day() as u8 {
                    print!("[{day:^0$}]", CELL_WIDTH - 2);
                } else {
                    print!("{day:^0$}", CELL_WIDTH);
                }
            }
        }
        println!();
        if enable_chinese {
            for _ in 0..spaces * CELL_WIDTH {
                print!(" ");
            }
            for day in start_day..end_day {
                let date = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap();
                let ch_day = ChineseDate::from_gregorian(&date)
                    .map(|ch_date| {
                        let ch_day = ch_date.day();
                        if ch_day == 1 {
                            let ch_month =
                                ChineseMonth::new(ch_date.month(), ch_date.leap()).unwrap();
                            if language == English {
                                format!("(M{})", TranslateAdapter(&ch_month, language))
                            } else {
                                ch_month.translate_to_string(language)
                            }
                        } else {
                            let ch_day = ChineseDay::new(ch_day).unwrap();
                            if language == English {
                                format!("({})", TranslateAdapter(&ch_day, language))
                            } else {
                                ch_day.translate_to_string(language)
                            }
                        }
                    })
                    .unwrap_or_default();
                if color {
                    let is_weekend = [Weekday::Sun, Weekday::Sat].contains(&date.weekday());
                    let style = if highlight_today && day == today.day() as u8 {
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
                    print!(
                        "{}{}{}",
                        style.render(),
                        Centered(&ch_day, CELL_WIDTH),
                        style.render_reset(),
                    );
                } else {
                    print!("{}", Centered(&ch_day, CELL_WIDTH));
                }
            }
            println!();
        }
        start_day = end_day;
        week_size = 7;
        spaces = 0;
    }
}
