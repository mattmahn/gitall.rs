use is_terminal::IsTerminal;
use termcolor::{Color, ColorChoice, ColorSpec, WriteColor};

use std::io::{self, Write};

use crate::cli::ColorMode;
use crate::GitResult;

pub fn color_choice(color_mode: &ColorMode, stream: &dyn IsTerminal) -> ColorChoice {
    match (color_mode, stream.is_terminal()) {
        (ColorMode::Always, _) => ColorChoice::Always,
        (ColorMode::Auto, true) => ColorChoice::Auto,
        (ColorMode::Auto, false) => ColorChoice::Never,
        (ColorMode::Never, _) => ColorChoice::Never,
    }
}

pub fn print_git_result<W>(stdout: &mut W, stderr: &mut W, gr: GitResult) -> io::Result<()>
where
    W: Write + WriteColor,
{
    let out = String::from_utf8(gr.output.stdout).expect("Git stdout is not valid UTF-8");
    let err = String::from_utf8(gr.output.stderr).expect("Git stderr is not valid UTF-8");

    print_repo_header(
        stdout,
        gr.directory_name.display(),
        gr.output.status.success(),
    )?;

    // reset color to default
    stdout.set_color(&ColorSpec::new())?;
    stdout.flush()?; // why is a flush needed?

    write!(stdout, "{}", out)?;
    write!(stderr, "{}", err)?;

    writeln!(stdout, "")?;

    Ok(())
}

fn print_repo_header<W, D>(w: &mut W, repo: D, success: bool) -> io::Result<()>
where
    W: Write + WriteColor,
    D: ::std::fmt::Display,
{
    let mut style = ColorSpec::new();
    style.set_bold(true);
    style.set_underline(true);
    style.set_fg(Some(if success { Color::Green } else { Color::Red }));

    w.set_color(&style)?;
    writeln!(w, "{}", repo)?;

    Ok(())
}
