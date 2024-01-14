# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Added
- Derived common traits for enums.

### Changed
- Update `embedded-hal` to version 1.0.
- Raised MSRV to 1.62.0

## [0.2.0] - 2021-09-02

### Added
- Support for PCT2075 devices. Thanks to @dkhayes117

### Changed
- [breaking-change] `SlaveAddr` has been renamed `Address` and now features conversions
  from bool tuples and custom `u8` values. Thanks to @dkhayes117

## [0.1.2] - 2021-01-30

### Added
- Parameter enums are now copyable and comparable.

### Changed
- Improved documentation.
- Use edition 2018 internally.

## [0.1.1] - 2018-10-20

This crate is now functionally complete.

### Added

- Setting the fault queue.
- Setting the OS temperature.
- Setting the hysteresis temperature.
- Setting the OS operation mode.
- Setting the OS polarity.

## 0.1.0 - 2018-10-19

This is the initial release to crates.io. All changes will be documented in
this CHANGELOG.

<!-- next-url -->
[Unreleased]: https://github.com/eldruin/lm75-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/eldruin/lm75-rs/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/eldruin/lm75-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/eldruin/lm75-rs/compare/v0.1.0...v0.1.1
