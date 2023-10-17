use std::io::IsTerminal;

use chrono::{Datelike, Month};
use clap::{arg, value_parser, Command};
use nongli::{
    calendar::{MonthCalendar, Options, TripleCalendar, YearCalendar},
    language::Language::*,
};

fn cmd() -> Command {
    clap::command!()
        .arg(arg!(-'3' --triple "Display the preceding, active and following month"))
        .arg(arg!(-C --chinese "Enable Chinese calendar"))
        .arg(arg!(-c --"no-chinese" "Disable Chinese calendar").conflicts_with("chinese"))
        .arg(arg!(-M --"start-on-monday" "Start on monday"))
        .arg(arg!(-n --"no-highlight-today" "Don't highlight today"))
        .arg(
            arg!(--color <color> "Whether to enable colors")
                .value_parser(["always", "auto", "never"]),
        )
        .arg(
            arg!(-y --year [year] "Year (1900-2100)")
                .value_parser(value_parser!(u16).range(1900..=2100))
                .default_missing_value(chrono::Local::now().date_naive().year().to_string()),
        )
        .arg(
            arg!(-m --month <month> "Month, in number (1-12)")
                .value_parser(value_parser!(u8).range(1..=12)),
        )
}

#[cfg(test)]
#[test]
fn test_cmd() {
    cmd().debug_assert();
}

fn main() {
    let matches = cmd().get_matches();

    let language = match std::env::var("LANG") {
        Ok(s) => match s.strip_prefix("zh_") {
            Some(s) => {
                if ["HK", "TW"].contains(&&s[..2]) {
                    ChineseTraditional
                } else {
                    ChineseSimplified
                }
            }
            None => English,
        },
        Err(_) => English,
    };
    let triple = matches.get_flag("triple");
    let start_on_monday = matches.get_flag("start-on-monday");
    let enable_chinese =
        matches.get_flag("chinese") || !(matches.get_flag("no-chinese") || language == English);
    let color = match matches.get_one::<String>("color") {
        Some(s) => match s.to_ascii_lowercase().as_str() {
            "always" => true,
            "never" => false,
            _ => std::io::stdout().is_terminal(),
        },
        _ => std::io::stdout().is_terminal(),
    };
    let highlight_today = !matches.get_flag("no-highlight-today");

    let today = chrono::Local::now().date_naive();

    let options = Options {
        language,
        enable_chinese,
        start_on_monday,
        highlight_today,
        color,
    };

    let year = matches.get_one::<u16>("year").copied();
    let month = matches
        .get_one::<u8>("month")
        .copied()
        .or_else(|| year.is_none().then_some(today.month() as u8))
        .and_then(|month| Month::try_from(month).ok());

    let year = year.unwrap_or_else(|| today.year() as u16);

    match month {
        Some(month) => {
            let calendar = MonthCalendar {
                year,
                month,
                today,
                options,
            };
            if triple {
                print!("{}", TripleCalendar(calendar.pred()));
            } else {
                print!("{}", calendar);
            }
        }
        None => print!(
            "{}",
            YearCalendar {
                year,
                today,
                options
            }
        ),
    }
}
