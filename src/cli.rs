use clap::builder::NonEmptyStringValueParser;
use clap::{Parser, ValueEnum};

use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Copy, ValueEnum)]
pub enum ColorMode {
    Always,
    Auto,
    Never,
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

#[derive(Parser)]
#[clap(author, about, version)]
pub struct Cli {
    /// Follow symbolic links
    ///
    /// When specified, symbolic links will be followed when navigating the directory tree.
    #[clap(short = 'L', long = "follow")]
    pub follow_links: bool,

    /// Controls when to use color
    #[clap(long, value_enum, ignore_case = true, value_name = "WHEN", default_value_t = ColorMode::Auto)]
    pub color: ColorMode,

    /// The directory to start searching under
    #[clap(short = 'D', long, value_name = "DIR", default_value = ".")]
    pub directory: PathBuf,

    /// Descend at most LEVELS directories below DIR
    #[clap(short = 'd', long, value_name = "LEVELS")]
    pub max_depth: Option<usize>,

    /// Match REGEX against the full directory path
    ///
    /// By default, REGEX matches against only the directory name.
    /// Using this flag, REGEX matches against the full canonical path.
    #[clap(long)]
    pub full_path: bool,

    /// Filters command to repo(s) matching provided regular expression
    #[clap(short, long, default_value = ".*")]
    pub regex: String,

    /// Maximum number of commands to run in parallel
    #[clap(short = 'j', long, value_name = "NUM")]
    pub threads: Option<usize>,

    /// The program to run in each repo
    #[clap(short = 'X', long, value_name = "PROGRAM", default_value = "git")]
    pub executable: String,

    /// A single git command to run in each repo
    #[clap(required = true, value_parser = NonEmptyStringValueParser::new())]
    pub command: Vec<String>,
}
