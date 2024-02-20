# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2024-02-20

### Added
- Add timestamps conversion to `std::time::Duration` in `subtp::srt::SrtTimestamp` and `subtp::vtt::VttTimestamp`.
- Add support for unofficial line position option in SubRip Subtitle format.

### Changed
- Ignore a line that has only whitespaces in `multiline` rule.
- Remove unused codes for testing.

### Fixed
- Fix errors of document tests.

## [0.1.0] - 2024-02-11

### Added

- First release.

[unreleased]: https://github.com/mochi-neko/subtp/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/mochi-neko/subtp/releases/tag/v0.1.0
