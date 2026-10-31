# Changelog

All notable changes to this project are documented here.

## [Unreleased]

### Added

- `--no-avatar`, `--no-color` and `--access-token` flags.
- Added `issues`, `language`, `following`, `prs`, `break_if` and `text` in fields.
- Added check for `gh auth credential` and `GITHUB_TOKEN` to authenticate github requests.

### Changed

- Switched from a match to using clap for cli management.
- Changed `total_stars` to just `stars`.
- Made toml error have only one blank line instead of two.
- Remove the duplicate token retrival in `main.rs`.
- Made there be no blank lines at the top when you pass `--no-avatar` flag.
- Made `Joined` just show date instead of showing date and time.

### Fixed

- Two blank lines being made instead of one on some profiles.

## [1.0.1] - 2026-09-22

This release was made as the first release was messed up. ghfetch version is still v1.0.0.

## [1.0.0] - 2026-09-22

- Initial release
