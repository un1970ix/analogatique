# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/un1970ix/analogatique/compare/0.3.0...master
[0.3.0]: https://github.com/un1970ix/analogatique/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/un1970ix/analogatique/compare/0.1.0...0.2.0
