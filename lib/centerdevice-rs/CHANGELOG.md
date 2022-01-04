# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1] - 2020-04-09

### Fix
* `get_token` call used to use `reqwest` client without adding a Root CA

## [0.5.0] - 2020-04-08

### Breaking Change
* `Client` changed to allow for externally defined, reqwest based HTTP clients

### Add
* `ClientBuilder` added that supports adding an self-maintained Root CA -- cf. `examples/root_ca.rs`.

## [0.4.1] - 2020-03-11

### Change
* Disable native-tls for reqwest; only use rust-tls

## [0.4.0] - 2020-02-10

### Update Dependencies
* ring > 0.16
* reqwest > 0.10
* Update minimum Rust version to 1.39

## [0.3.12] - 2019-22-22

### Add
* debug logging for metadata in upload

### Fix
* NO_CONTENT error for empty collection search result


## [0.3.10] - 2019-06-18

### Add
* debug info to requests and responses

### Fix
* users search passed parameters as form instead of query parameters


## [0.3.9] - 2019-06-12

### Fix
* Collection results in-accessible because of private visibility
* Send parameters as query instead of form parameters in collection search

## [0.3.8] - 2019-06-13

### Add

* Users listing
* Collections listing

### Change

* search documents returns empty `Vec` instead of None in case no documents have been found.


## [0.3.7] - 2019-06-13

### Add

* General error handling

### Fixed

* Optional fields in search results


## [0.3.6] - 2019-06-12

### Add

* Serialization to search and delete results


## [0.3.5] - 2019-06-12

### Changed

* struct visibilities changed


## [0.3.4] - 2019-06-12

### Changed

* Enhances search results by datetime, representations, and extented-metadata
* `Search` derives `Debug`


## [0.3.2] - 2019-06-11

### Changed

* Derive `Debug` for `client::download::Download`.
* `client::download::Download` uses references.


## [0.3.1] - 2019-06-06

### Changed

* Derive `Debug` for `client::upload::Upload`.


## [0.3.0] - 2019-06-06

### Added

* Exporting `reqwest::IntoUrl` because it's part of the public API in `auth::CodeProvider`.
* `auth::Token` can be cloned.

### Changed

* `ClientCredentials` uses references
* `UnauthorizedClient` and `AuthorizedClient` use references


## [0.2.0] - 2019-06-06

### Changed

* Simplified errors


## [0.1.0] - 2019-06-03

Initial release supports
* auth
* search
* upload
* delete
* download

[Unreleased]: https://github.com/lukaspustina/ceres/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/lukaspustina/ceres/compare/v0.4.1...0.5.0
[0.4.1]: https://github.com/lukaspustina/ceres/compare/v0.4.0...0.4.1
[0.4.0]: https://github.com/lukaspustina/ceres/compare/v0.3.12...0.4.0
[0.3.12]: https://github.com/lukaspustina/ceres/compare/v0.3.10...0.3.12
[0.3.10]: https://github.com/lukaspustina/ceres/compare/v0.3.9...0.3.10
[0.3.9]: https://github.com/lukaspustina/ceres/compare/v0.3.8...0.3.9
[0.3.8]: https://github.com/lukaspustina/ceres/compare/v0.3.7...0.3.8
[0.3.7]: https://github.com/lukaspustina/ceres/compare/v0.3.6...0.3.7
[0.3.6]: https://github.com/lukaspustina/ceres/compare/v0.3.5...0.3.6
[0.3.5]: https://github.com/lukaspustina/ceres/compare/v0.3.4...0.3.5
[0.3.4]: https://github.com/lukaspustina/ceres/compare/v0.3.3...0.3.4
[0.3.3]: https://github.com/lukaspustina/ceres/compare/v0.3.2...0.3.3
[0.3.2]: https://github.com/lukaspustina/ceres/compare/v0.3.1...0.3.2
[0.3.1]: https://github.com/lukaspustina/ceres/compare/v0.3.0...0.3.1
[0.3.0]: https://github.com/lukaspustina/ceres/compare/v0.2.0...0.3.0
[0.2.0]: https://github.com/lukaspustina/ceres/compare/v0.1.0...0.2.0
[0.1.0]: https://github.com/lukaspustina/ceres/compare/v0.0.1...0.1.0

