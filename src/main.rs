use std::io::IsTerminal;

use anstyle::{Color, AnsiColor, Style};
use chrono::{Datelike, Weekday};

pub const WEEKEND_COLOR: Color = Color::Ansi(AnsiColor::Red);

fn main() {
    let is_terminal = std::io::stdout().is_terminal();
    let today = chrono::Local::now().date_naive();
    println!("{:20}{:7}",
        chrono::Month::try_from(today.month() as u8).unwrap().name(),
        today.year(),
    );
    if is_terminal {
        println!(
            "{0}Sun{1} Mon Tue Wed Thu Fri {0}Sat{1}",
            Style::new().fg_color(Some(WEEKEND_COLOR)).render(),
            anstyle::Reset.render()
        );
    } else {
        println!("Sun Mon Tue Wed Thu Fri Sat");
    }
    let spaces = today.with_day(1).unwrap().weekday().num_days_from_sunday();
    for _ in 0..spaces {
        print!("    ");
    }
    let days = match today.month() {
        1 => 31,
        2 => if today.leap_year() { 29 } else { 28 },
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => unreachable!(),
    };
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
