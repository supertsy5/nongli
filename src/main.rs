use std::io::IsTerminal;

use anstyle::{AnsiColor, Color, Style};
use chrono::{
    Datelike, Month,
    Weekday::{self, *},
};
use nongli::language::{Language::*, ShortTranslate};

pub const WEEKEND_COLOR: Color = Color::Ansi(AnsiColor::Red);

fn main() {
    let language = if std::env::var("LANG").is_ok_and(|lang| lang.starts_with("zh")) {
        Chinese
    } else {
        English
    };
    let start_on_monday = std::env::var("START_ON_MONDAY")
        .is_ok_and(|s| ["1", "true", "", "yes"].contains(&s.as_str()));
    let start_of_week = if start_on_monday { Mon } else { Sun };
    let end_of_week = start_of_week.pred();
    let is_terminal = std::io::stdout().is_terminal();
    let today = chrono::Local::now().date_naive();
    println!(
        "{:^28}",
        nongli::language::Title(
            today.year(),
            Month::try_from(today.month() as u8).unwrap(),
            language
        )
        .to_string()
    );
    for weekday in nongli::iter::Weekdays(start_of_week).take(7) {
        let mut style = Style::new();
        if is_terminal && nongli::is_weekend(weekday) {
            style = style.fg_color(Some(WEEKEND_COLOR));
            print!("{}", style.render());
        }
        match language {
            English => print!("{} ", weekday.short_translate(English)),
            Chinese => print!(" {} ", weekday.short_translate(Chinese)),
        }
        if is_terminal {
            print!("{}", style.render_reset());
        }
    }
    println!();
    let weekday_of_1st = today.with_day(1).unwrap().weekday();
    let spaces = if start_on_monday {
        weekday_of_1st.num_days_from_monday()
    } else {
        weekday_of_1st.num_days_from_sunday()
    };
    for _ in 0..spaces {
        print!("    ");
    }
    let days = nongli::days_of_month(today);
    for day in 1..=days {
        let date = today.with_day(day).unwrap();
        let mut style = Style::new();
        if is_terminal {
            let is_weekend = [Weekday::Sun, Weekday::Sat].contains(&date.weekday());
            if is_weekend {
                style = style.fg_color(Some(WEEKEND_COLOR));
            }
            if day == today.day() {
                style = style.invert();
            }
            print!("{} {day:2} {}", style.render(), style.render_reset());
        } else {
            if day == today.day() {
                print!("[{day:2}]");
            } else {
                print!(" {day:2} ");
            }
        }
        if date.weekday() == end_of_week {
            println!();
        }
    }
    println!();
}
