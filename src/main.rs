use atty;
use clap::ArgMatches;
use rayon;
use regex::Regex;
use termcolor::{StandardStream};
use walkdir::{DirEntry, WalkDir};

use std::ffi::OsStr;
use std::process::{Command, Output};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::iter::{IntoIterator, Iterator};

mod cli;
mod output;

/// List of Git commands that support the `--color` option.  Only commands shown
/// here will have `--color` set in the spawned process.
const COMMANDS: &[&'static str] = &["branch", "diff", "diff-index",
    "format-patch", "grep", "log", "reflog", "rev-list", "shortlog", "show"];

#[derive(Debug)]
pub struct GitResult {
    directory_name: PathBuf,
    output: Output,
}

fn main() {
    let matches: ArgMatches<'static> = cli::build_cli().get_matches();

    let git_command_args = matches.values_of("cmd").unwrap();
    let git_command: Arc<Vec<String>> = Arc::new(git_command_args.map(String::from).collect());
    let git_color_arg: Arc<String> = Arc::new(format!("--color={}", cli::get_color_mode(&matches)));

    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
    let (tx, rx) = channel::<GitResult>();

    // configure directory tree walker
    let parent_dir = matches.value_of_os("dir").unwrap_or(OsStr::new("."));
    let mut wd = WalkDir::new(parent_dir)
        .min_depth(1)
        .follow_links(matches.is_present("follow_links"));
    if let Some(max_depth) = matches.value_of("max_depth") {
        wd = wd.max_depth(usize::from_str_radix(max_depth, 10).expect("max-depth option could not be parsed as a base-10 number"));
    }

    let regex_str = matches.value_of("regex").unwrap();  // default is provided to clap; can safely unwrap
    let regex = Regex::new(regex_str).expect("regex option is not a valid regular expression (see https://docs.rs/regex/1/regex/#syntax for syntax)");
    let regex_full_path = matches.is_present("full_path");

    // walk directory tree
    let mut it = wd.into_iter();
    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(err)) => {
                eprintln!("skipping directory due to error: {}", err);
                it.skip_current_dir();
                continue;
            },
            Some(Ok(entry)) => entry,
        };
        if is_git_dir(&entry) { // we've found a Git repo root
            // stop searching below this directory since we've already
            // found a Git repo root
            it.skip_current_dir();
            if is_matching_dir(&entry, &regex, regex_full_path) { // Only run commands for filtered repo(s)
                let tx = tx.clone();
                let args = git_command.clone();
                let color_arg = git_color_arg.clone();

                pool.spawn(move || {
                    let mut command = Command::new("git");
                    command.current_dir(entry.path())
                        .args(args.as_ref());
                    if should_set_color_arg(args.as_ref()) {
                        command.arg(color_arg.as_ref());
                    }
                    let output = command.output()
                        .expect("failed to execute git command");

                    tx.send(GitResult {
                        directory_name: entry.into_path(),
                        output
                    }).unwrap();
                });
            }
        }
        // here, `entry` _may not_ be a Git repo
    }

    // manually drop tx so the receiver ends
    drop(tx);

    let color_mode = cli::get_color_mode(&matches);
    let stdout = StandardStream::stdout(output::color_choice(&color_mode, &atty::Stream::Stdout));
    let stderr = StandardStream::stderr(output::color_choice(&color_mode, &atty::Stream::Stderr));
    {
        let mut stdout_l = stdout.lock();
        let mut stderr_l = stderr.lock();

        for gr in rx {
            output::print_git_result(&mut stdout_l, &mut stderr_l, gr).unwrap();
        }
    }
}

fn is_git_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
        && entry.path().join(".git").is_dir()
}

fn is_matching_dir(entry: &DirEntry, regex: &Regex, full_path: bool) -> bool {
    let canonical_path = entry.path().canonicalize().expect("failed to canonicalize the directory path");
    let asdf: &str = if full_path {
        canonical_path.to_str().unwrap()
    } else {
        entry.file_name().to_str().unwrap()
    };
    regex.is_match(asdf)
}

fn should_set_color_arg(cmd: &Vec<String>) -> bool {
    cmd.iter().any(|term| COMMANDS.contains(&term.as_str()))
}
