use clap::Shell;

use std::{env, fs, path};

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        Some(outdir) => outdir,
        None => {
            panic!("OUT_DIR environment variable not defined");
        }
    };
    fs::create_dir_all(&outdir).unwrap();

    let stamp_path = path::Path::new(&outdir).join("gitall-stamp");
    if let Err(err) = fs::File::create(&stamp_path) {
        panic!("failed to write {}: {}", stamp_path.display(), err);
    }

    // use clap to build completion files
    let mut app = cli::build_cli();
    let variants = &[
        Shell::Bash,
        Shell::Fish,
        Shell::Zsh,
        Shell::PowerShell,
        Shell::Elvish,
    ];
    for &variant in variants {
        app.gen_completions("gitall", variant, outdir.clone());
    }
}
