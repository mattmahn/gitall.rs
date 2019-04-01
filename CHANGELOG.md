# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]
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


[Unreleased]: https://github.com/mattmahn/gitall.rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/mattmahn/gitall.rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/mattmahn/gitall.rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/mattmahn/gitall.rs/compare/d9647f8e72b5a50101217f090c7a8bc3716c5c98...v0.1.0
