use atty;
use clap::ArgMatches;
use rayon;
use regex::Regex;
use termcolor::StandardStream;

use std::default::Default;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::mpsc::channel;
use std::sync::Arc;

mod cli;
mod lib;
use lib::Giterator;
mod output;

/// List of Git commands that support the `--color` option.  Only commands shown
/// here will have `--color` set in the spawned process.
const COMMANDS: &[&'static str] = &[
    "branch",
    "diff",
    "diff-index",
    "format-patch",
    "grep",
    "log",
    "reflog",
    "rev-list",
    "shortlog",
    "show",
];

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

    // configure directory tree walker
    let parent_dir = matches.value_of_os("dir").unwrap_or(OsStr::new("."));
    let mut g = Giterator::default()
        .root(parent_dir)
        .follow_links(matches.is_present("follow_links"));
    if let Some(max_depth) = matches.value_of("max_depth") {
        g = g.max_depth(
            usize::from_str_radix(max_depth, 10)
                .expect("max-depth option could not be parsed as a base-10 number"),
        );
    }
    let regex_str = matches.value_of("regex").unwrap(); // default is provided to clap; can safely unwrap
    let regex = Regex::new(regex_str).expect("regex option is not a valid regular expression (see https://docs.rs/regex/1/regex/#syntax for syntax)");
    g = g.regex(regex);
    g = g.match_full_path(matches.is_present("full_path"));

    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
    let (tx, rx) = channel::<GitResult>();

    // walk directory tree
    for entry in g {
        // Only run commands for filtered repo(s)
        let tx = tx.clone();
        let args = git_command.clone();
        let color_arg = git_color_arg.clone();

        pool.spawn(move || {
            let mut command = Command::new("git");
            command.current_dir(entry.path()).args(args.as_ref());
            if should_set_color_arg(args.as_ref()) {
                command.arg(color_arg.as_ref());
            }
            let output = command.output().expect("failed to execute git command");

            tx.send(GitResult {
                directory_name: entry.into_path(),
                output,
            })
            .unwrap();
        });
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

fn should_set_color_arg(cmd: &Vec<String>) -> bool {
    cmd.iter().any(|term| COMMANDS.contains(&term.as_str()))
}
