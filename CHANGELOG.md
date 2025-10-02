# Changelog

Release notes.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- Windows platform now exports WCstr

## [0.1.0]
### Added
- `dynlink::api::Handle` and `dynlink::api::Symbol` provides a platform-independent dynamic linking API
- POSIX dynamic linking implementation.
- WIN32 dynamic linking implementation.

### Notes
- API types (`Handle`, `Symbol`) are stable for the next minor release;
behavior and ABI for platform implementations may receive bugfixes only.
