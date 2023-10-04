use std::io::IsTerminal;

use anstyle::{Color, AnsiColor};
use chrono::Datelike;

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

fn main() {
    let is_terminal = std::io::stdout().is_terminal();
    let date = chrono::Local::now().date_naive();
    println!("{:^28}", format_args!("{} {}", MONTHS[date.month() as usize], date.year()));
    println!("Sun Mon Tue Wed Thu Fri Sat");
    let spaces = date.with_day(1).unwrap().weekday().num_days_from_sunday();
    for _ in 0..spaces {
        print!("    ");
    }
    let mut column = spaces;
    let days = match date.month() {
        1 => 31,
        2 => if date.leap_year() { 29 } else { 28 },
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
        if day == date.day() {
            if is_terminal {
                print!(
                    " {}{day:^2}{} ",
                    anstyle::Style::new()
                        .bg_color(Some(Color::Ansi(AnsiColor::BrightWhite)))
                        .fg_color(Some(Color::Ansi(AnsiColor::Black)))
                        .render(),
                    anstyle::Reset.render(),
                );
            } else {
                print!("[{day:^2}]");
            }
        } else {
            print!("{day:^4}");
        }
        if column == 6 {
            println!();
            column = 0;
        } else {
            column += 1;
        }
    }
    println!();
}
