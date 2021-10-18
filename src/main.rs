#![feature(termination_trait_lib)]

use crate::args::Args;
use crate::error::Exit;

mod args;
mod error;

fn main() -> error::Exit<'static> {
    let args = match Args::from_env() {
        Ok(args) => args,
        Err(err) => return  Exit::from(Err(err)),
    };

    println!("Cmd: {}", args.cmd);

    for a in args.cmd_args {
        println!("Args: {}", a);
    }
    Exit::from(Ok(()))
}
