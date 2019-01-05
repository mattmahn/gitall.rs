use clap::ArgMatches;
use rayon;
use walkdir::{DirEntry, WalkDir};

use std::ffi::OsStr;
use std::process::{Command, Output};
use std::path::PathBuf;
use std::sync::Arc;
use std::io::{self, Write};
use std::sync::mpsc::channel;

mod cli;

#[derive(Debug)]
struct GitResult {
    directory_name: PathBuf,
    output: Output,
}

fn main() {
    let matches: ArgMatches<'static> = cli::build_cli().get_matches();

    let git_command_args = matches.values_of("cmd").unwrap();
    let git_command: Arc<Vec<String>> = Arc::new(git_command_args.map(String::from).collect());

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

    // walk directory tree
    let dir_iter = wd.into_iter().filter_entry(is_git_dir);
    for entry in dir_iter {
        let tx = tx.clone();
        let args = git_command.clone();

        pool.spawn(move || {
            let entry: DirEntry = entry.unwrap();
            let output = Command::new("git")
                .current_dir(entry.path())
                .args(args.as_ref())
                .output()
                .expect("failed to execute git command");

            tx.send(GitResult {
                directory_name: entry.into_path(),
                output
            }).unwrap();
        });
    }

    // manually drop tx so the receiver ends
    drop(tx);

    let stdout = io::stdout();
    let mut stdout_l = stdout.lock();
    let stderr = io::stderr();
    let mut stderr_l = stderr.lock();

    for gr in rx {
        print_git_result(&mut stdout_l, &mut stderr_l, gr);
    }
}

fn is_git_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
        && entry.path().join(".git").is_dir()
}

fn print_git_result(stdout: &mut Write, stderr: &mut Write, gr: GitResult) {
    writeln!(stdout, "{:#?}", gr.directory_name.into_os_string()).unwrap();
    write!(stdout, "{}", String::from_utf8(gr.output.stdout).expect("git output is not valid UTF-8")).unwrap();
    write!(stderr, "{}", String::from_utf8(gr.output.stderr).expect("git output is not valid UTF-8")).unwrap();
    writeln!(stdout, "").unwrap();
}
