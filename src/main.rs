#![feature(termination_trait_lib)]
#![feature(int_abs_diff)]

use crate::args::Args;
use crate::error::Exit;

mod args;
mod error;
mod neowatch;
mod signal;

const HELP_STR: &str = "Neowatch
Tavo Annus <tavo.annus@gmail.com>
Modern alternative to watch.

USAGE:
    neowatch [FLAGS] [OPTIONS] <COMMAND> [CMMAND ARGS...]

FLAGS:
    -h, --help           Print help message
    -d, --differences    Highlight changes between updates
    -p, --precise        Attempt to run command with precise intervals
    -e, --errexit        Exit if command has non-zero exit status
    -g, --chgexit        Exit when output of command changes

OPTIONS:
    -n, --interval <secs>    Seconds to wait between updates

ARGS:
    COMMAND    The command to execute";

fn main() -> error::Exit<'static> {
    let args = match Args::from_env() {
        Ok(args) => args,
        Err(err) => return Exit::from(Err(err)),
    };

    if args.show_help {
        print!("{}", HELP_STR);
        return Exit::from(Ok(()));
    }

    signal::setup_handlers();

    println!("\x1B[?1049h");
    let res = neowatch::run(args);

    Exit::from(res)
}
