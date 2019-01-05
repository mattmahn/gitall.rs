use clap::Shell;

use std::env;

include!("src/cli.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = build_cli();
    let variants = &[
        Shell::Bash,
        Shell::Fish,
        Shell::Zsh,
        Shell::PowerShell,
        Shell::Elvish
    ];
    for &variant in variants {
        app.gen_completions("gitall", variant, outdir.clone());
    }
}
