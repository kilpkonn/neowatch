#![feature(termination_trait_lib)]
#![feature(int_abs_diff)]

extern crate clap;

use crate::args::Args;
use crate::error::Exit;

mod args;
mod error;
mod neowatch;
mod signal;


fn main() -> error::Exit<'static> {
    let args = match Args::new() {
        Ok(args) => args,
        Err(err) => return Exit::from(Err(err)),
    };

    signal::setup_handlers();
    
    println!("\x1B[?1049h");
    let res = neowatch::run(args);

    Exit::from(res)
}
