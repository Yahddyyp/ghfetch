# Changelog

All notable changes to this project are documented here.

## [Unreleased]

## Fixes

- Made "break" field be removed when it is the last line in the fields to maintain consistency of one line gap.

## Changed

- Switched from a match to using clap for cli management.

### Added

- Automated release builds for macOS and Linux.
- `--no-avatar` and `--no-color` flags.

## [1.0.1] - 2026-09-22

This release was made as i messed up the first release and the ghfetch version is still v1.0.0

## [1.0.0] - 2026-09-22

### Added

- Initial release of ghfetch.
- Configurable fields and colors.
- Configurable terminal profile image.
- Kitty Graphics Protocol image rendering.
- GitHub token support.
- `--help` and `--version` options.

### Changed

- Improved GitHub API handling.
- Improved configuration handling.
