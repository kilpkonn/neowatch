#![feature(termination_trait_lib)]
#![feature(int_abs_diff)]

use args::Opts;
use clap::Parser;

use crate::args::Args;
use crate::error::Exit;

mod args;
mod error;
mod neowatch;
mod signal;


fn main() -> error::Exit<'static> {
    let opts = Opts::parse();
    let args = Args::from(opts);

    signal::setup_handlers();
    
    println!("\x1B[?1049h");
    let res = neowatch::run(args);

    Exit::from(res)
}
