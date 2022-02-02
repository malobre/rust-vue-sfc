# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- `AttributeName`, `AttributeValue` and `BlockName` now implements `PartialOrd`, `Ord` and `Default`.
- `Section` and `Block` now implements `PartialOrd`, `Ord` and `Hash`.

### Fixed

- Fix nested tags parsing.

## [0.1.0] - 2022-02-02

### Added

- Initial release.

[unreleased]: https://github.com/malobre/vue-sfc/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/malobre/vue-sfc/releases/tag/v0.1.0