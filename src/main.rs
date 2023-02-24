extern crate clap;

use crate::args::Args;
use crate::error::Exit;

mod args;
mod error;
mod neowatch;

fn main() -> error::Exit<'static> {
    let args = match Args::new() {
        Ok(args) => args,
        Err(err) => return Exit::from(Err(err)),
    };

    let handler = || {
        println!("\x1B[?1049l");
        std::process::exit(0)
    };
    if ctrlc::set_handler(handler).is_err() {
        return Exit::from(Err(error::Error::CouldNotSetSignalHandler));
    }

    println!("\x1B[?1049h");
    let res = neowatch::run(args);

    Exit::from(res)
}
