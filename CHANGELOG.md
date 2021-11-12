# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

## [Unreleased]

## [2.0.0] - 2022-11-12
### Changed
- Backwards-incompatible changes have been introduced to the API. Now it consists of two functions:
  - `calculate` - returns a `u64`
  - `calculate_tuple` - returns a `(u32, u32)`, similar to `calculate` in 1.0.x
  Unlike 1.0.x, these functions are not generic. This enables upgrading them to `const fn` in a
  future release, without breaking the API (again).

## [1.0.2] - 2019-08-29
### Changed
- Updated criterion to 0.3. (No code changes were necessary in order to support this.)
- The changelog now conforms to [Keep a Changelog] format. It has been renamed from `CHANGES.md` to
  `CHANGELOG.md`.

## [1.0.1] - 2019-05-26
### Added
- Changes are now recorded in `CHANGES.md`.

### Changed
- Refactored for increased performance.

## [1.0.0] - 2019-05-26
### Added
- Initial release.

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[Unreleased]: https://github.com/FaultyRAM/bsa3-hash/compare/1.0.2...HEAD
[1.0.2]: https://github.com/FaultyRAM/bsa3-hash/compare/1.0.1...1.0.2
[1.0.1]: https://github.com/FaultyRAM/bsa3-hash/compare/1.0.0...1.0.1
[1.0.0]: https://github.com/FaultyRAM/bsa3-hash/releases/tag/1.0.0
