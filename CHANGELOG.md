# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Multi-platform CI/CD pipeline (Linux, macOS, Windows)
- Automatic binary releases for all platforms
- Comprehensive test suite (70 tests)

### Fixed
- Pass-only method handling
- Method name fixing in automatic corrections

### Changed
- Enhanced GitHub Actions workflow

## [0.1.0] - 2025-01-29

### Added
- Initial release
- Core checking functionality for `setUp`/`tearDown` methods
- Automatic fixing with `--fix` flag
- Validation of method casing (setUp/tearDown)
- Validation that super() call is last statement
- Support for nested classes
- Pre-commit hook configuration
- Comprehensive test suite (70 tests):
  - 17 fixing operation tests
  - 2 integration tests
  - 20 edge case tests
  - 31 real-world pattern tests
- Multi-platform support (Linux, macOS, Windows)
- Automatic crates.io publishing on release
- GitHub release artifacts

[Unreleased]: https://github.com/yourusername/pytest-super-setup-hooks/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/pytest-super-setup-hooks/releases/tag/v0.1.0
