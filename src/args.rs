use std::time::Duration;
use clap::{Parser, crate_version, crate_authors, crate_description};
use termcolor::{Color, ColorSpec};


#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!(), about = crate_description!())]
pub struct Opts {
    /// Update insterval
    #[clap(short = 'n', long = "interval", default_value = "1.0")]
    pub interval: f32,
    /// Highlight differences since last update
    #[clap(short = 'd', long = "differences")]
    pub show_diff: bool,
    /// Try to run command at precise intervals
    #[clap(short = 'p', long = "precise")]
    pub precise_mode: bool,
    /// Exit on non-zero return code
    #[clap(short = 'e', long = "errexit")]
    pub exit_on_err: bool,
    /// Exit on output change
    #[clap(short = 'g', long = "chgexit")]
    pub exit_on_change: bool,
    /// Target command
    #[clap(value_name = "COMMAND")]
    pub cmd: String,
    /// Arguments for target command
    #[clap(value_name = "CMD_ARGS")]
    pub cmd_args: Vec<String>,
}


pub struct Args {
    pub interval: Duration,
    pub show_diff: bool,
    pub precise_mode: bool,
    pub exit_on_err: bool,
    pub exit_on_change: bool,
    pub color_change: ColorSpec,
    pub cmd: String,
    pub cmd_args: Vec<String>,
}

impl From<Opts> for Args {
    fn from(o: Opts) -> Self {
        let mut col = ColorSpec::new();
        col.set_fg(Some(Color::Cyan));
        Args {
            interval: Duration::from_secs_f32(o.interval),
            show_diff: o.show_diff,
            precise_mode: o.precise_mode,
            exit_on_err: o.exit_on_err,
            exit_on_change: o.exit_on_change,
            color_change: col,
            cmd: o.cmd,
            cmd_args: o.cmd_args,
        }
    }
}


