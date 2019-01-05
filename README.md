# gitall

gitall finds all repositories below a directory and runs the given [Git][] command in each repository in parallel.

[![Linux/macOS Build Status](https://travis-ci.org/mattmahn/gitall.rs.svg?branch=master)](https://travis-ci.org/mattmahn/gitall.rs)


## Usage

The simplest form is to `cd` to a directory containing all the repos you want to operate on, then write your Git command changing `git` to `gitall`.

**Pro Tip:** if you add a file called `git-foo` to your `$PATH` (either via copying or symlink), you can call that program _through_ `git` using `git foo`; no additional aliases or setup needed.
So, after running `ln -s /usr/bin/gitall /usr/bin/git-all`, you can use gitall via `git all` exactly the same as if you used `gitall`.


### Examples

Fetch the latest changes for all your repositories under `~/code`:
```console
$ cd ~/code
$ gitall fetch origin
```
Alternatively, you can run `gitall -D ~/code fetch origin` from any directory.


### Options

```
Executes git(1) commands in repos below a parent directory

USAGE:
    gitall [FLAGS] [OPTIONS] <COMMAND>...

FLAGS:
    -L, --follow     Follow symbolic links
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -D, --directory <DIR>       The directory to start searching under [default: .]
    -d, --max-depth <LEVELS>    Descend at most LEVELS of directories below DIR

ARGS:
    <COMMAND>...    A single git command to run in each repo
```


## Installation

Dependencies:
  - Git
  - Rust & Cargo (for build only)

You can download pre-built binaries for some platforms on the [releases][] page.
After extracting the release artifacts, move the `gitall` executable to some directory in your `$PATH`.
Refer to your shell's documentation for installing  the completion scripts located in `complete/`.


### From source

Clone this repository, build gitall, then copy the executable to a directory in your `$PATH`:
```console
$ git clone https://github.com/mattmahn/gitall.rs && cd gitall.rs
$ cargo build --release
# install target/release/gitall /usr/bin/
```

Shell completions for Bash, Zsh, fish, Elvish, and PowerShell are also generated during build; you can find them at `target/release/build/gitall-<hash>/out/`.
Refer to your shell's documentation for installation.


[Git]: https://git-scm.com/
[releases]: https://github.com/mattmahn/gitall.rs/releases
