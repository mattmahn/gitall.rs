use clap::{
    crate_authors, crate_description, crate_version, value_t_or_exit, App, Arg, ArgMatches,
};

use std::ffi::OsStr;
use std::fmt;
use std::str::FromStr;

pub enum ColorMode {
    Always,
    Auto,
    Never,
}

pub struct ColorModeError {
    kind: ColorModeErrorKind,
}
enum ColorModeErrorKind {
    NoMatch,
}

impl ColorMode {
    pub fn variants() -> Vec<&'static str> {
        vec!["always", "true", "auto", "never", "false"]
    }
}

impl fmt::Display for ColorMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            ColorMode::Always => "always",
            ColorMode::Auto => "auto",
            ColorMode::Never => "never",
        };
        write!(f, "{}", text)
    }
}

impl FromStr for ColorMode {
    type Err = ColorModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("always") {
            Ok(ColorMode::Always)
        } else if s.eq_ignore_ascii_case("auto") || s.eq_ignore_ascii_case("true") {
            Ok(ColorMode::Auto)
        } else if s.eq_ignore_ascii_case("never") || s.eq_ignore_ascii_case("false") {
            Ok(ColorMode::Never)
        } else {
            Err(ColorModeError {
                kind: ColorModeErrorKind::NoMatch,
            })
        }
    }
}

pub fn build_cli() -> App<'static, 'static> {
    App::new("gitall")
        .version(crate_version!())
        .author(crate_authors!(", "))
        .about(crate_description!())
        .arg(Arg::with_name("follow_links")
             .short("L")
             .long("follow")
             .help("Follow symbolic links")
             .long_help("When specified, symbolic links will be followed when navigating the directory tree."))
        .arg(Arg::with_name("color_mode")
             .long("color")
             .takes_value(true)
             .possible_values(&ColorMode::variants())
             .case_insensitive(true)
             .value_name("WHEN")
             .default_value("auto")
             .help("Controls when to use color"))
        .arg(Arg::with_name("dir")
             .short("D")
             .long("directory")
             .help("The directory to start searching under")
             .takes_value(true)
             .value_name("DIR")
             .default_value("."))
        .arg(Arg::with_name("max_depth")
             .short("d")
             .long("max-depth")
             .help("Descend at most LEVELS of directories below DIR")
             .takes_value(true)
             .value_name("LEVELS"))
        .arg(Arg::with_name("full_path")
            .long("full-path")
            .help("Match REGEX against the full directory path")
            .long_help("By default, REGEX matches against only the directory name. Using this flag, REGEX matches against the full canonical path."))
        .arg(Arg::with_name("regex")
            .short("r")
            .long("regex")
            .help("Filters command to repo(s) matching provided regular expression")
            .takes_value(true)
            .value_name("REGEX")
            .default_value(".*"))
        .arg(Arg::with_name("workers")
            .short("j")
            .long("threads")
            .help("The maximum number of commands to run in parallel")
            .takes_value(true)
            .value_name("NUM"))
        .arg(Arg::with_name("executable")
            .short("X")
            .long("executable")
            .help("The program to run in each repo")
            .takes_value(true)
            .value_name("PROGRAM")
            .default_value_os(OsStr::new("git")))
        .arg(Arg::with_name("cmd")
             .help("A single git command to run in each repo")
             .index(1)
             .required(true)
             .empty_values(false)
             .multiple(true)
             .value_name("COMMAND"))
}

pub fn get_color_mode(matches: &ArgMatches) -> ColorMode {
    value_t_or_exit!(matches, "color_mode", ColorMode)
}
