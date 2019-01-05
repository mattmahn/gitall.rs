# gitall

gitall finds all repositories below a directory and runs the given [Git][] command in each repository in parallel.


## Usage

The simplest form is to `cd` to a directory containing all the repos you want to operate on, then write your Git command changing `git` to `gitall`.


### Examples

Fetch the latest changes for all your repositories under `~/code`:
```console
$ cd ~/code
$ gitall fetch origin
```
Alternatively, you could also run `gitall -D ~/code fetch origin` from any directory.


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

Clone this repository, build gitall, then copy the executable to a directory in your `$PATH`:
```console
$ git clone https://github.com/mattmahn/gitall.rs && cd gitall.rs
$ cargo build --release
# install target/release/gitall /usr/bin/
```

Shell completions for Bash, Zsh, fish, Elvish, and PowerShell are also generated during build; you can find them at `target/release/build/gitall-<hash>/out/`.
Refer to your shell's documentation for installation.


[Git]: https://git-scm.com/
