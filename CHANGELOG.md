<!-- The changelog shall follow the recommendations described here: https://keepachangelog.com/en/1.0.0/ 
Types for Changes:
- Added
- Changed
- Deprecated
- Removed
- Fixed
- Security
-->

# Changelog

<!-- next-header -->
## [0.7.0]

* Timeout interrupts running process.

## [0.6.0]

### Added 

* Timeout feature for test functions returning a result.

## [0.5.0]

### Changed

* Add name attribute for test cases additionaly to the test_name attribute.
* Repalce string special characters with something meaningfull instead of a `_`-character.

## [0.4.1]

### Changed

* Moved to GitHub

## [0.4.0]

### Removed

* Timebomb dependency

### Changed

* Timeout will only panic after the test execution if the test took longer than the defined time

## [0.3.4]

### Added

* Changelog
* Link check in CI pipeline via mlc
