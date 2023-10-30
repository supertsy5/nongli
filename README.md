# Nongli: A Rust library and CLI tool for Chinese calendar
[![github](https://img.shields.io/badge/GitHub-supertsy5/nongli-blue?logo=github)](https://github.com/supertsy5/nongli)
[![crates.io](https://img.shields.io/crates/v/nongli?logo=rust)](https://crates.io/crates/nongli)
[![github](https://img.shields.io/badge/docs.rs-nongli-green?logo=rust)](https://docs.rs/nongli)
[![License](https://img.shields.io/crates/l/nongli?logo=rust)](https://github.com/supertsy5/nongli/blob/main/LICENSE)
![Preview](./preview.png)

Note: This project is in early development and not guaranteed to run on every platform.
Bug reports are welcome.
## Install (as an executable)
```sh
cargo install nongli
```
or
```sh
cargo install --git https://github.com/supertsy5/nongli
```
## As a library
Talk is cheap, let me show the code.
```rust
fn test() {
    use nongli::{ChineseDate, SolarTerm};
    let date = chrono::NaiveDate::from_ymd_opt(2023, 10, 30).unwrap();
    let chinese_date = ChineseDate::new(2023, 09, false, 16).unwrap();
    assert_eq!(ChineseDate::from_gregorian(date), chinese_date);
    assert_eq!(chinese_date.to_gregorian(), date);
    assert_eq!(NaiveDate::from_ymd_opt(2023, 10, 25).unwrap(), Some(SolarTerm::Shangjiang));
}
```

## Development
Tip: run
```sh
. aliases.sh
```
for common aliases.

Clone this repository and run:
```sh
cargo build
```