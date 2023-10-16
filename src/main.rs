use std::io::IsTerminal;

use chrono::{Datelike, Month};
use clap::{arg, value_parser};
use nongli::{language::Language::*, calendar::{Options, MonthCalendar}};

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
        .arg(arg!(-y --year [year] "Year").value_parser(value_parser!(u16).range(1900..=2100)))
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
    let month = Month::try_from(
        matches.get_one::<u8>("month").copied().unwrap_or_else(|| today.month() as u8)
    )
    .unwrap();
    let highlight_today = !matches.get_flag("no-highlight-today")
        && today.year() == year as i32
        && today.month() == month.number_from_month();
    print!("{}", MonthCalendar {
        year,
        month,
        today,
        options: Options {
            language,
            enable_chinese,
            start_on_monday,
            highlight_today,
            color,
        }
});

}
