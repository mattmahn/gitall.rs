use clap::{CommandFactory, ValueEnum};
use clap_complete::Shell;

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
    let mut cli = cli::Cli::command();
    for &variant in Shell::value_variants() {
        clap_complete::generate_to(variant, &mut cli, "gitall", outdir.clone()).unwrap();
    }
}
