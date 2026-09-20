# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.2] - 2026-09-20

### Added
- Workflow running formatting, lint, test, and build checks on every push and pull request.
- Regression tests covering the filename collision, metadata parse failure, thumbnail sizing, and Exif camera fixes from 0.3.1.
- Dependabot updates for GitHub Actions and Cargo dependencies.
- Release workflow publishes tags with a suffix, such as `0.4.0-rc1`, as prereleases.

### Changed
- Declared a minimum supported Rust version of 1.88.
- Replaced the archived `actions/create-release`, `actions/upload-release-asset`, and `actions-rs/toolchain` actions with maintained equivalents.
- Cross is now installed only on Linux runners, where it has an image, and is pinned to an exact revision.
- Audit workflow runs `cargo audit` directly instead of an action that still targets Node 20.

### Fixed
- Audit workflow could not publish its results because the job token lacked permission to create a check run.

## [0.3.1] - 2026-09-20

### Added
- Weekly `cargo audit` workflow.

### Changed
- Limited `image` to the formats actually read and written, reducing the dependency tree from 142 to 94 crates.

### Fixed
- `extract-metadata` no longer discards every entry in `metadata.txt` when a single line fails to parse; the offending line number is reported instead.
- Photos sharing a name across formats no longer overwrite each other's generated files.
- Gallery ordering is now stable for photos sharing a date, instead of following filesystem order.
- Thumbnails for very wide panoramas no longer collapse to a single pixel.
- Camera name is read when only one of the Exif `Make` or `Model` tags is present, and is no longer duplicated when `Model` already repeats `Make`.
- `metadata.txt` now ends with a newline.

### Security
- Updated dependencies to clear RUSTSEC-2026-0204, RUSTSEC-2026-0190, and RUSTSEC-2026-0097, and to drop the yanked `core2`, `js-sys`, and `wasm-bindgen` releases.

## [0.3.0] - 2026-02-22

### Added
- Static pagination support via `photos_per_page` config option.
- Pre-computed masonry column layout using shortest-column-first algorithm.
- Pagination navigation with previous/next links across pages.

### Changed
- Replaced CSS `column-count` layout with flexbox-based column system.
- Gallery now renders balanced columns using photo aspect ratios.
- Added `width` and `height` attributes to images to prevent layout shift.
- Mobile layout preserves chronological reading order via `display: contents`.

## [0.2.0] - 2025-06-17

### Changed
- Made major code reorganization into modular structure.
- Refactored CLI into separate command structure.

## 0.1.0 - 2025-06-17

- First release!

[Unreleased]: https://github.com/un1970ix/analogatique/compare/0.3.2...master
[0.3.2]: https://github.com/un1970ix/analogatique/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/un1970ix/analogatique/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/un1970ix/analogatique/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/un1970ix/analogatique/compare/0.1.0...0.2.0
