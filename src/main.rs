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

    if let Err(_) = ctrlc::set_handler(|| {
        println!("\x1B[?1049l");
        std::process::exit(0)
    }) {
        return Exit::from(Err(error::Error::CouldNotSetSignalHandler));
    }

    println!("\x1B[?1049h");
    let res = neowatch::run(args);

    Exit::from(res)
}
