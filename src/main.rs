use std::io::IsTerminal;

use anstyle::{AnsiColor, Color, Reset, Style};
use chrono::{
    Datelike, Month,
    Weekday::{self, *},
};
use nongli::language::{Language::*, ShortTranslate};

pub const CELL_WIDTH: usize = 8;
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
    let language = if std::env::var("LANG").is_ok_and(|lang| lang.starts_with("zh")) {
        Chinese
    } else {
        English
    };
    let start_on_monday = std::env::var("START_ON_MONDAY")
        .is_ok_and(|s| ["", "1", "t", "true", "y", "yes"].contains(&s.as_str()));
    let start_of_week = if start_on_monday { Mon } else { Sun };
    let end_of_week = start_of_week.pred();
    let highlight_today = true;
    let is_terminal = std::io::stdout().is_terminal();
    let today = chrono::Local::now().date_naive();

    let title = nongli::language::Title(
        today.year(),
        Month::try_from(today.month() as u8).unwrap(),
        language,
    )
    .to_string();
    println!("{}", Centered(&title, CELL_WIDTH * 7));

    if is_terminal {
        print!("{}", Style::new().invert().render());
    }
    for weekday in nongli::iter::Weekdays(start_of_week).take(7) {
        if is_terminal {
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
            Centered(weekday.short_translate(language), CELL_WIDTH)
        );
        if is_terminal {
            print!("{}", Reset.render());
        }
    }
    println!();

    let weekday_of_1st = today.with_day(1).unwrap().weekday();
    let mut spaces = if start_on_monday {
        weekday_of_1st.num_days_from_monday()
    } else {
        weekday_of_1st.num_days_from_sunday()
    };
    for _ in 0..spaces {
        print!("    ");
    }
    let days = nongli::days_of_month(today.year() as u16, today.month() as u8) as u32;
    for day in 1..=days {
        let date = today.with_day(day).unwrap();
        if is_terminal {
            let is_weekend = [Weekday::Sun, Weekday::Sat].contains(&date.weekday());
            let style = if highlight_today && day == today.day() {
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
                "{}{day:^2$}{}",
                style.render(),
                style.render_reset(),
                CELL_WIDTH
            );
        } else {
            #[allow(clippy::collapsible_if)]
            if highlight_today && day == today.day() {
                print!("[{day:^0$}]", CELL_WIDTH - 2);
            } else {
                print!("{day:^0$}", CELL_WIDTH);
            }
        }
        if date.weekday() == end_of_week {
            println!();
            if spaces > 0 {
                for _ in 0..spaces {
                    print!("    ");
                }
                spaces = 0;
            }
        }
    }
    println!();
}
