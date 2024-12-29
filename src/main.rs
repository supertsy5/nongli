use std::{io::IsTerminal, process::exit};

use chrono::{Datelike, Month};
use clap::{arg, value_parser, Command};
use nongli::{
    calendar::{Calendar, Options},
    cli_calendar::{ListCalendar, MonthCalendar, TripleCalendar, YearCalendar},
    iter::Months,
    language::{Language::*, Translate},
    ChineseDate,
};

fn cmd() -> Command {
    clap::command!()
        .arg(arg!(-'3' --triple "Display the preceding, active and following month"))
        .arg(
            arg!(-c --chinese [chinese] "Whether to enable Chinese calendar")
                .value_parser(["always", "auto", "never"])
                .default_missing_value("always"),
        )
        .arg(arg!(-M --"start-on-monday" "Start on monday"))
        .arg(arg!(-n --"no-highlight-today" "Don't highlight today"))
        .arg(
            arg!(-l --landscape "Show full-year calendar in 3 rows and 4 columns")
                .conflicts_with("month"),
        )
        .arg(
            arg!(-p --portrait "Show full-year calendar in 4 rows and 3 columns")
                .conflicts_with_all(["month", "landscape"]),
        )
        .arg(
            arg!(-L --list "Show the calendar in a list")
                .conflicts_with_all(["landscape", "portrait"]),
        )
        .arg(
            arg!(--color <color> "Whether to enable colors")
                .value_parser(["always", "auto", "never"]),
        )
        .arg(
            arg!(-y --year [year] "Year")
                .value_parser(value_parser!(i32).range(-262143..=262142))
                .default_missing_value(chrono::Local::now().year().to_string()),
        )
        .arg(
            arg!(-m --month <month> "Month, in number (1-12)")
                .value_parser(value_parser!(u8).range(1..=12)),
        )
        .arg(arg!(-w --week "Show week numbers"))
        .arg(arg!(-t --today "Show today in Chinese calendar"))
}

#[cfg(test)]
#[test]
fn test_cmd() {
    cmd().debug_assert();
}

fn main() {
    let matches = cmd().get_matches();

    let language = match std::env::var("LANG") {
        Ok(s) => match match s.split_once('.') {
            Some((a, _)) => a,
            None => &s,
        }
        .split_once('_')
        {
            Some(("zh", region)) => {
                if ["HK", "MO", "TW"].contains(&region) {
                    ChineseTraditional
                } else {
                    ChineseSimplified
                }
            }
            None => {
                if s == "zh" {
                    ChineseSimplified
                } else {
                    English
                }
            }
            _ => English,
        },
        Err(_) => English,
    };
    let triple = matches.get_flag("triple");
    let start_on_monday = matches.get_flag("start-on-monday");
    let enable_chinese = match matches.get_one::<String>("chinese") {
        Some(s) => match s.as_str() {
            "always" => true,
            "never" => false,
            _ => language != English,
        },
        _ => language != English,
    };
    let color = match matches.get_one::<String>("color") {
        Some(s) => match s.as_str() {
            "always" => true,
            "never" => false,
            _ => std::io::stdout().is_terminal(),
        },
        None => std::io::stdout().is_terminal(),
    };
    let highlight_today = !matches.get_flag("no-highlight-today");
    let landscape = matches.get_flag("landscape");
    let portrait = matches.get_flag("portrait");
    let list = matches.get_flag("list");
    let week_number = matches.get_flag("week");
    let show_today = matches.get_flag("today");

    let today = std::env::var("TODAY")
        .ok()
        .and_then(|string| chrono::NaiveDate::parse_from_str(&string, "%Y-%m-%d").ok())
        .unwrap_or_else(|| chrono::Local::now().date_naive());

    let options = Options {
        language,
        enable_chinese,
        start_on_monday,
        color,
        week_number,
    };

    if show_today {
        if let Some(chinese_date) = ChineseDate::from_gregorian(&today) {
            println!("{}", chinese_date.translate_adapter(language));
        } else {
            println!("Today is out of the Chinese calendar range");
        }
        return;
    }

    let year = matches.get_one::<i32>("year").copied();
    let month = matches
        .get_one::<u8>("month")
        .copied()
        .or_else(|| year.is_none().then_some(today.month() as u8))
        .and_then(|month| Month::try_from(month).ok());

    let year = year.unwrap_or_else(|| today.year());

    match month {
        Some(month) if !(landscape || portrait) => {
            let Some(calendar) =
                Calendar::new(year, month, highlight_today.then_some(today), options)
            else {
                eprintln!("Date out of range");
                exit(-1);
            };
            if list {
                if triple {
                    if let Some(pred) = calendar.pred() {
                        println!("{}", ListCalendar(pred));
                    }
                    println!("{}", ListCalendar(calendar));
                    if let Some(succ) = calendar.succ() {
                        println!("{}", ListCalendar(succ));
                    }
                } else {
                    print!("{}", ListCalendar(calendar));
                }
            } else if triple {
                let Some(pred) = calendar.pred() else {
                    eprintln!("Date out of range");
                    exit(-1);
                };
                print!("{}", TripleCalendar(pred));
            } else {
                print!("{}", MonthCalendar(calendar));
            }
        }
        _ => {
            if list {
                for month in Months(Month::January).take(12) {
                    print!(
                        "{}",
                        ListCalendar(
                            Calendar::new(year, month, highlight_today.then_some(today), options)
                                .unwrap()
                        )
                    );
                }
            } else {
                let Some(year) =
                    YearCalendar::new(year, highlight_today.then_some(today), options, landscape)
                else {
                    eprintln!("Year out of range");
                    exit(-1);
                };
                print!("{year}");
            }
        }
    }
}
