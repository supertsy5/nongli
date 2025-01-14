# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.1] - 2025-01-11
### Added
- Added festival Yuanxiaojie
- Added Chinese festivals to CLI calendar
- Added short translations for festivals

## [0.4.0] - 2025-01-01
### Added
- Made `Calendar` and `Options` `Eq`
- Added function `Calendar::title`
- Added week numbers
- Added festivals

### Changed
- Made `Calendar` and `YearCalendar` limited
- Updated translations
- Changed functions in `chinese_date.rs` to use `ChineseYear` and `ChineseMonth` as parameters

## [0.3.1] - 2024-12-25
### Fixed
- Fixed a bug

## [0.3.0] - 2024-12-25
### Added
- Added the new `Calendar` system
### Changed
- Made CLI features optional
- Renamed the old `calendar.rs` as `cli_calendar.rs`
- Change year type to `i32`

## [0.2.1] - 2024-12-24
### Fixed
- Fixed problems on wasm32

## [0.2.0] - 2024-12-24
### Added
- Completed solar term data up to 2039
### Removed
- Removed `nongli::data::SOLAR_TERMS_2020S`

## [0.1.1] - 2023-10-30
### Changed
- Re-exported `nongli::chinese_date::ChineseDate`

## [0.1.0] - 2023-10-30
- Initial release