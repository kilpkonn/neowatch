use std::time::Duration;

use crate::error::Error;

pub struct Args {
    pub interval: Duration,
    pub cmd: String,
    pub cmd_args: Vec<String>,
}

impl Args {
    pub fn from_env() -> Result<Self, Error<'static>> {
        let mut args_vec: Vec<String> = std::env::args().collect();
        args_vec.remove(0);

        let mut args: Args = Default::default();

        let mut skip = false;
        for i in 0..args_vec.len() {
            if skip {
                skip = false;
                continue;
            }
            if let Some(arg) = args_vec.get(i) {
                if !arg.starts_with("-") {
                    args_vec.drain(0..i);
                    break;
                }
                if arg == "-n" || arg == "--interval" {
                    if let Some(interval) = args_vec.get(i + 1).and_then(|s| s.parse::<f32>().ok())
                    {
                        args.interval = Duration::from_secs_f32(interval);
                        skip = true;
                    } else {
                        return Err(Error::InvalidArgs("Invalid interval!"));
                    };
                }
            }
        }
        
        if args_vec.len() < 1 {
            return Err(Error::InvalidArgs("No Target command!"));
        }

        args.cmd = args_vec.remove(0);
        args.cmd_args = args_vec;

        Ok(args)
    }
}

impl Default for Args {
    fn default() -> Self {
        Args {
            interval: Duration::from_secs(1),
            cmd: String::from("neowatch"),
            cmd_args: vec![String::from("--help")],
        }
    }
}
