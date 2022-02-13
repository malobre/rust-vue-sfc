# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] - 2022-02-13

### Fixed

- Blocks are now parsed as raw text with two exceptions:
  - `template` blocks with no `lang` attribute,
  - `template` blocks with a `lang` attribute set to `html`.

## [0.3.0] - 2022-02-06

### Added

- Added `Raw` struct for raw sections.
- Added `InvalidRaw` error.

### Changed

- `Section::Raw` now contains a `Raw`.

### Fixed

- Fixed parsing of trailing raw sections, which could be empty if they contained only `\r` & `\n`.

## [0.2.0] - 2022-02-03

### Added

- Added `AttributeName::from_cow_unchecked`.
- Added `AttributeValue::from_cow_unchecked`.
- Added `BlockName::from_cow_unchecked`.
- Added errors:
  - `ast::InvalidBlockName`,
  - `ast::InvalidAttributeName`,
  - `ast::InvalidAttributeValue`,
  - `error::Error`.

### Removed

- Removed `AttributeName::new`.
- Removed `AttributeValue::new`.
- Removed `BlockName::new`.
- Removed `IllegalCharError`.
- Removed re-export of `ParseError`.

### Changed

- `AttributeName`, `AttributeValue` and `BlockName` now implements `PartialOrd`, `Ord` and `Default`.
- `Section` and `Block` now implements `PartialOrd`, `Ord` and `Hash`.
- Renamed `AttributeName::try_new` to `from_cow`.
- Renamed `AttributeValue::try_new` to `from_cow`.
- Renamed `BlockName::try_new` to `from_cow`.

### Fixed

- Fixed nested tags parsing.
- Fixed comments being parsed as block, (tag name must now begin with an ASCII alpha).
- Fixed consecutive blocks not being parsed.

## [0.1.0] - 2022-02-02

### Added

- Initial release.

[unreleased]: https://github.com/malobre/rust-vue-sfc/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/malobre/rust-vue-sfc/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/malobre/rust-vue-sfc/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/malobre/rust-vue-sfc/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/malobre/rust-vue-sfc/releases/tag/v0.1.0
