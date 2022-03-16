# Changelog
All notable changes to this library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this library adheres to Rust's notion of
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Support for multiple history tree versions:
  - `crypticcoin_history::Version` trait.
  - `crypticcoin_history::V1`, marking the original history tree version.
  - `crypticcoin_history::V2`, marking the history tree version from NU5.
- `crypticcoin_history::Entry::new_leaf`

### Changed
- `crypticcoin_history::{Entry, IndexedNode, Tree}` now have a `Version` parameter.

### Removed
- `impl From<NodeData> for Entry` (replaced by `Entry::new_leaf`).

## [0.2.0] - 2020-03-13
No changes, just a version bump.

## [0.0.1] - 2020-03-04
Initial release.
