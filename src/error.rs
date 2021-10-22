use std::fmt::Display;
use std::process::Termination;

pub enum Error<'a> {
    InvalidArgs(&'a str),
    CouldNotSpawnProcess,
    ProcessFailed(String),
}

impl From<Error<'_>> for i32 {
    fn from(err: Error) -> Self {
        match err {
            Error::InvalidArgs(_) => 1,
            Error::CouldNotSpawnProcess => 2,
            Error::ProcessFailed(_) => 4,
        }
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidArgs(msg) => write!(f, "Invalid arguments: {}", msg),
            Error::CouldNotSpawnProcess => write!(f, "Could not spawn child process"),
            Error::ProcessFailed(msg) => write!(f, "Process failed: {}", msg),
        }
    }
}

pub struct Exit<'a>(Result<(), Error<'a>>);

impl<'a> From<Result<(), Error<'a>>> for Exit<'a> {
    fn from(res: Result<(), Error<'a>>) -> Self {
        Exit(res)
    }
}

impl<'a> Termination for Exit<'a> {
    fn report(self) -> i32 {
        match self.0 {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("{}", err);
                err.into()
            }
        }
    }
}
