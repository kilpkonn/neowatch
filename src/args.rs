use clap::{Arg, ArgAction, Command};
use std::{ffi::OsString, str::FromStr, time::Duration};
use termcolor::{Color, ColorSpec};

use crate::error::Error;

pub struct Args {
    pub interval: Duration,
    pub show_diff: bool,
    pub precise_mode: bool,
    pub exit_on_err: bool,
    pub exit_on_change: bool,
    pub show_number_diff: bool,
    pub radix: u32,
    pub color_new: ColorSpec,
    pub color_change: ColorSpec,
    pub color_increase: ColorSpec,
    pub color_decrease: ColorSpec,
    pub cmd: String,
    pub cmd_args: Vec<OsString>,
}

impl Args {
    pub fn new() -> Result<Self, Error<'static>> {
        let matches = Command::new("neowatch")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg_required_else_help(true)
            .allow_external_subcommands(true)
            .arg(
                Arg::new("interval")
                    .short('n')
                    .long("interval")
                    .value_name("SECS")
                    .default_value("1.0")
                    .help("Set update insterval")
                    .num_args(1)
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("show_diff")
                    .short('d')
                    .long("differences")
                    .help("Highlight differences since last update")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("precise")
                    .short('p')
                    .long("precise")
                    .help("Attempt to run command at precise intervals")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("exit_on_err")
                    .short('e')
                    .long("errexit")
                    .help("Exit on non-zero return code")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("exit_on_change")
                    .short('g')
                    .long("chgexit")
                    .help("Exit on output change")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("color_new")
                    .long("new-color")
                    .help("Color for new text [word|ANSI|rr,gg,bb]")
                    .value_name("COLOR")
                    .num_args(1)
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("color_change")
                    .long("change-color")
                    .help("Color for changed text [word|ANSI|rr,gg,bb]")
                    .value_name("COLOR")
                    .num_args(1)
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("number_diff")
                    .short('z')
                    .long("number-changes")
                    .help("Highlight number changes based on increase/decrease")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("radix")
                    .long("radix")
                    .help("Radix for numbers")
                    .default_value("10")
                    .num_args(1)
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("color_increase")
                    .long("increase-color")
                    .help("Color for increaseing numeric values")
                    .value_name("COLOR")
                    .num_args(1)
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("color_decrease")
                    .long("decrease-color")
                    .help("Color for decreasing numeric values")
                    .value_name("COLOR")
                    .num_args(1)
                    .action(ArgAction::Set),
            )
            .get_matches();

        let interval = matches
            .get_one::<String>("interval")
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(1.0);
        let interval = Duration::from_secs_f32(interval);

        let show_diff = matches.get_flag("show_diff");
        let precise_mode = matches.get_flag("precise");
        let exit_on_err = matches.get_flag("exit_on_err");
        let exit_on_change = matches.get_flag("exit_on_change");
        let show_number_diff = matches.get_flag("number_diff");

        let radix = matches
            .get_one::<String>("radix")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(10);

        let mut color_new = ColorSpec::new();
        let mut color_change = ColorSpec::new();
        let mut color_increase = ColorSpec::new();
        let mut color_decrease = ColorSpec::new();

        let col_new = matches
            .get_one::<String>("color_new")
            .and_then(|s| FromStr::from_str(s).ok())
            .unwrap_or(Color::Green);

        let col_change = matches
            .get_one::<String>("color_change")
            .and_then(|s| FromStr::from_str(s).ok())
            .unwrap_or(Color::Magenta);

        let col_increase = matches
            .get_one::<String>("color_increase")
            .and_then(|s| FromStr::from_str(s).ok())
            .unwrap_or(Color::Cyan);

        let col_decrease = matches
            .get_one::<String>("color_decrease")
            .and_then(|s| FromStr::from_str(s).ok())
            .unwrap_or(Color::Red);

        color_new.set_fg(Some(col_new));
        color_change.set_fg(Some(col_change));
        color_increase.set_fg(Some(col_increase));
        color_decrease.set_fg(Some(col_decrease));

        let (cmd, cmd_args) = match matches.subcommand() {
            Some((external, ext_m)) => {
                let ext_args = if let Some(args) = ext_m.get_many::<OsString>("") {
                    args.map(OsString::to_owned).collect()
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
            show_number_diff,
            radix,
            color_new,
            color_change,
            color_increase,
            color_decrease,
            cmd,
            cmd_args,
        };
        Ok(args)
    }
}
