use atty;
use clap::Parser;
use rayon;
use regex::Regex;
use termcolor::StandardStream;

use std::default::Default;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::mpsc::channel;
use std::sync::Arc;

mod cli;
use cli::Cli;
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
    let cli = Cli::parse();

    let git_command = Arc::new(cli.command);
    let git_color_arg: Arc<String> = Arc::new(format!("--color={}", cli.color));

    // configure directory tree walker
    let mut g = Giterator::default()
        .root(&cli.directory)
        .follow_links(cli.follow_links);
    if let Some(max_depth) = cli.max_depth {
        g = g.max_depth(max_depth);
    }
    let regex = Regex::new(&cli.regex).expect("regex option is not a valid regular expression (see https://docs.rs/regex/1/regex/#syntax for syntax)");
    g = g.regex(regex);
    g = g.match_full_path(cli.full_path);

    let threads = cli.threads.unwrap_or_else(num_cpus::get);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();
    let (tx, rx) = channel::<GitResult>();

    // walk directory tree
    for entry in g {
        // Only run commands for filtered repo(s)
        let tx = tx.clone();
        let args = git_command.clone();
        let color_arg = git_color_arg.clone();
        let program = cli.executable.clone();

        pool.spawn(move || {
            let mut command = Command::new(&program);
            command.current_dir(entry.path()).args(args.as_ref());
            if program == "git" && should_set_git_color_arg(args.as_ref()) {
                command.arg(color_arg.as_ref());
            }
            let output = command.output().expect("failed to execute command");

            tx.send(GitResult {
                directory_name: entry.into_path(),
                output,
            })
            .unwrap();
        });
    }

    // manually drop tx so the receiver ends
    drop(tx);

    let stdout = StandardStream::stdout(output::color_choice(&cli.color, &atty::Stream::Stdout));
    let stderr = StandardStream::stderr(output::color_choice(&cli.color, &atty::Stream::Stderr));
    {
        let mut stdout_l = stdout.lock();
        let mut stderr_l = stderr.lock();

        for gr in rx {
            output::print_git_result(&mut stdout_l, &mut stderr_l, gr).unwrap();
        }
    }
}

fn should_set_git_color_arg(cmd: &Vec<String>) -> bool {
    cmd.iter().any(|term| COMMANDS.contains(&term.as_str()))
}
