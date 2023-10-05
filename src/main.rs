use std::io::IsTerminal;

use anstyle::{AnsiColor, Color, Style};
use chrono::{Datelike, Month, Weekday};
use nongli::language::{Language::*, ShortTranslate};

pub const WEEKEND_COLOR: Color = Color::Ansi(AnsiColor::Red);

fn main() {
    let language = if std::env::var("LANG").is_ok_and(|lang| lang.starts_with("zh")) {
        Chinese
    } else {
        English
    };
    let start_of_week = Weekday::Sun;
    let is_terminal = std::io::stdout().is_terminal();
    let today = chrono::Local::now().date_naive();
    println!(
        "{:^28}",
        nongli::language::Title(
            today.year(),
            Month::try_from(today.month() as u8).unwrap(),
            language
        ).to_string()
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
    let spaces = today.with_day(1).unwrap().weekday().num_days_from_sunday();
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
        if date.weekday() == Weekday::Sat {
            println!();
        }
    }
    println!();
}
