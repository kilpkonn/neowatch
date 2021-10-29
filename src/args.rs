use clap::{App, AppSettings, Arg};
use std::{str::FromStr, time::Duration};
use termcolor::{Color, ColorSpec};

use crate::error::Error;

pub struct Args {
    pub interval: Duration,
    pub show_diff: bool,
    pub precise_mode: bool,
    pub exit_on_err: bool,
    pub exit_on_change: bool,
    pub color_new: ColorSpec,
    pub color_change: ColorSpec,
    pub cmd: String,
    pub cmd_args: Vec<String>,
}

impl Args {
    pub fn new() -> Result<Self, Error<'static>> {
        let matches = App::new("neowatch")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .setting(AppSettings::AllowExternalSubcommands)
            .arg(
                Arg::new("interval")
                    .short('n')
                    .long("interval")
                    .value_name("SECS")
                    .default_value("1.0")
                    .about("Set update insterval")
                    .takes_value(true),
            )
            .arg(
                Arg::new("show_diff")
                    .short('d')
                    .long("differences")
                    .about("Highlight differences since last update")
                    .takes_value(false),
            )
            .arg(
                Arg::new("precise")
                    .short('p')
                    .long("precise")
                    .about("Attempt to run command at precise intervals")
                    .takes_value(false),
            )
            .arg(
                Arg::new("exit_on_err")
                    .short('e')
                    .long("errexit")
                    .about("Exit on non-zero return code")
                    .takes_value(false),
            )
            .arg(
                Arg::new("exit_on_change")
                    .short('g')
                    .long("chgexit")
                    .about("Exit on output change")
                    .takes_value(false),
            )
            .arg(
                Arg::new("color_new")
                    .long("new-color")
                    .about("Color for new text")
                    .takes_value(true),
            )
            .arg(
                Arg::new("color_change")
                    .long("change-color")
                    .about("Color for changed text")
                    .takes_value(true),
            )
            .get_matches();

        let interval = matches
            .value_of("interval")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(1.0);
        let interval = Duration::from_secs_f32(interval);

        let show_diff = matches.is_present("show_diff");
        let precise_mode = matches.is_present("precise_mode");
        let exit_on_err = matches.is_present("exit_on_err");
        let exit_on_change = matches.is_present("exit_on_change");

        let mut color_new = ColorSpec::new();
        let mut color_change = ColorSpec::new();

        let col_new = matches
            .value_of("color_new")
            .and_then(|s| FromStr::from_str(s).ok())
            .unwrap_or(Color::Green);

        let col_change = matches
            .value_of("color_change")
            .and_then(|s| FromStr::from_str(s).ok())
            .unwrap_or(Color::Cyan);

        color_new.set_fg(Some(col_new));
        color_change.set_fg(Some(col_change));

        let (cmd, cmd_args) = match matches.subcommand() {
            Some((external, ext_m)) => {
                let ext_args = if let Some(args) = ext_m.values_of("") {
                    args.map(String::from).collect()
                } else {
                    Vec::new()
                };
                (String::from(external), ext_args)
            }
            _ => {
                return Err(Error::InvalidArgs("No target command!"));
            }
        };

        let args = Args {
            interval,
            show_diff,
            precise_mode,
            exit_on_err,
            exit_on_change,
            color_new,
            color_change,
            cmd,
            cmd_args,
        };
        Ok(args)
    }
}
