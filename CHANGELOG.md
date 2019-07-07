# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://code.plopgrizzly.com/semver/).

## [Unreleased]

## [0.3.0] - 2019-07-07
### Added
- Can now generate GLSL shaders for use with [window](https://crates.io/crates/window), [barg](https://crates.io/crates/barg) or [cala](https://crates.io/crates/cala) crates using `generate()`.

### Changed
- Old API is now being phased out in place of new `generate()` function.

## [0.2.1] - 2017-05-13
### Changed
- Upgraded versions of dependencies.

## [0.2.0] - 2017-06-05
### Added
- TOML parsing support for the crate.

## [0.1.0] - 2017-05-28
### Added
- `generate()` function for grabbing files from `res` folder and including with `include_bytes()`.
