# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Made `Calendar` and `Options` `Eq`
- Added function `Calendar::title`

### Changed
- Made `Calendar` and `YearCalendar` limited

## [0.3.1] - 2024-12-25
### Fixed
- Fixed a bug

## [0.3.0] - 2024-12-25
### Added
- Added the new `Calendar` system
### Changed
- Made CLI features optional
- renamed the old `calendar.rs` as `cli_calendar.rs`

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