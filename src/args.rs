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
    pub show_number_diff: bool,
    pub radix: u32,
    pub color_new: ColorSpec,
    pub color_change: ColorSpec,
    pub color_increase: ColorSpec,
    pub color_decrease: ColorSpec,
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
                    .help("Set update insterval")
                    .takes_value(true),
            )
            .arg(
                Arg::new("show_diff")
                    .short('d')
                    .long("differences")
                    .help("Highlight differences since last update")
                    .takes_value(false),
            )
            .arg(
                Arg::new("precise")
                    .short('p')
                    .long("precise")
                    .help("Attempt to run command at precise intervals")
                    .takes_value(false),
            )
            .arg(
                Arg::new("exit_on_err")
                    .short('e')
                    .long("errexit")
                    .help("Exit on non-zero return code")
                    .takes_value(false),
            )
            .arg(
                Arg::new("exit_on_change")
                    .short('g')
                    .long("chgexit")
                    .help("Exit on output change")
                    .takes_value(false),
            )
            .arg(
                Arg::new("color_new")
                    .long("new-color")
                    .help("Color for new text [word|ANSI|rr,gg,bb]")
                    .value_name("COLOR")
                    .takes_value(true),
            )
            .arg(
                Arg::new("color_change")
                    .long("change-color")
                    .help("Color for changed text [word|ANSI|rr,gg,bb]")
                    .value_name("COLOR")
                    .takes_value(true),
            )
            .arg(
                Arg::new("number_diff")
                    .short('z')
                    .long("number-changes")
                    .help("Highlight number changes based on increase/decrease")
                    .takes_value(false),
            )
            .arg(
                Arg::new("radix")
                    .long("radix")
                    .help("Radix for numbers")
                    .default_value("10")
                    .takes_value(true),
            )
            .arg(
                Arg::new("color_increase")
                    .long("increase-color")
                    .help("Color for increaseing numeric values")
                    .value_name("COLOR")
                    .takes_value(true),
            )
            .arg(
                Arg::new("color_decrease")
                    .long("decrease-color")
                    .help("Color for decreasing numeric values")
                    .value_name("COLOR")
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
        let show_number_diff = matches.is_present("number_diff");

        let radix = matches
            .value_of("radix")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(10);

        let mut color_new = ColorSpec::new();
        let mut color_change = ColorSpec::new();
        let mut color_increase = ColorSpec::new();
        let mut color_decrease = ColorSpec::new();

        let col_new = matches
            .value_of("color_new")
            .and_then(|s| FromStr::from_str(s).ok())
            .unwrap_or(Color::Green);

        let col_change = matches
            .value_of("color_change")
            .and_then(|s| FromStr::from_str(s).ok())
            .unwrap_or(Color::Magenta);

        let col_increase = matches
            .value_of("color_increase")
            .and_then(|s| FromStr::from_str(s).ok())
            .unwrap_or(Color::Cyan);

        let col_decrease = matches
            .value_of("color_decrease")
            .and_then(|s| FromStr::from_str(s).ok())
            .unwrap_or(Color::Red);

        color_new.set_fg(Some(col_new));
        color_change.set_fg(Some(col_change));
        color_increase.set_fg(Some(col_increase));
        color_decrease.set_fg(Some(col_decrease));

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
