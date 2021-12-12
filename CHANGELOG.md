# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]


## [0.6.0] - 2021-12-12
### Added
- [#7]: add `-X` option to run a program other than `git`
- [#8]: add `-j` option to limit the maximum number of repos to process in parallel

### Changed
- replaced Travis CI with GitHub Actions

[#7]: https://github.com/mattmahn/gitall.rs/issues/7
[#8]: https://github.com/mattmahn/gitall.rs/issues/8


## [0.5.0] - 2021-02-22
### Added
- crate/library entrypoint so `gitalll` can now be used as a library in other projects


## [0.4.0] - 2020-10-18
### Added
- [#5]: add `--full-path` flag to match `--regex` option against each directory's full & canonicalized path

### Changed
- update dependencies to more recent versions

[#5]: https://github.com/mattmahn/gitall.rs/issues/5


## [0.3.0] - 2020-10-15
### Added
- [#5]: add `-r`/`--regex` option to only work on Git directories matching the given
  regular expression

[#5]: https://github.com/mattmahn/gitall.rs/issues/5


## [0.2.2] - 2019-04-05
No changes except to fix my deploy script.  This is same code as v0.2.1.


## [0.2.1] - 2019-04-05
### Fixed
- [#4]: `--color` option is properly passed to Git
- `blame` has been removed from the `--color` whitelist because it does not support the option

[#4]: https://github.com/mattmahn/gitall.rs/issues/4


## [0.2.0] - 2019-02-03
### Fixed
- [#3]: Directories are properly recursed into until the "shallowest" Git repository is found.

### Added
- Future releases will have pre-built binaries for `x86_64-pc-windows-msvc`
- [#2]: The printed output is colorful by default!
  The path/repo headers for successful Git commands will be printed in green, while failed commands are printed in red.
  The new `--color` option controls the colorful behavior.
  The `--color` option is also passed to a whitelist of Git commands, in order to control their output as well.

[#2]: https://github.com/mattmahn/gitall.rs/issues/2
[#3]: https://github.com/mattmahn/gitall.rs/issues/3


## [0.1.1] - 2019-01-05
### Added
- The project now has CI/CD implemented using Travis CI to test against stable, beta, and nightly Rust release channels
- Future releases will have pre-built binaries for the following x86_64 platforms:
  - Linux GNU C
  - Linux musl C
  - macOS


## [0.1.0] - 2019-01-05
### Added
- Run Git commands in all repositories below the current directory
  - Commands will be run in parallel; the maximum of which is the number of logical processors in the system
- `-d`/`--max-depth` option to specify the maximum depth to search for Git repositories
- `-L`/`--follow` flag to follow symbolic links when walking the directory search tree
- `-D`/`--directory` option to set the search directory to something other than the current working directory


[Unreleased]: https://github.com/mattmahn/gitall.rs/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/mattmahn/gitall.rs/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/mattmahn/gitall.rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/mattmahn/gitall.rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/mattmahn/gitall.rs/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/mattmahn/gitall.rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/mattmahn/gitall.rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/mattmahn/gitall.rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/mattmahn/gitall.rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/mattmahn/gitall.rs/compare/d9647f8e72b5a50101217f090c7a8bc3716c5c98...v0.1.0
