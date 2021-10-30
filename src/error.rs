use std::fmt::Display;
use std::io;
use std::process::Termination;

pub enum Error<'a> {
    InvalidArgs(&'a str),
    CouldNotSpawnProcess,
    ProcessFailed(String),
    ProcessErrExit(i32),
    Io(io::Error),
}

impl From<Error<'_>> for i32 {
    fn from(err: Error) -> Self {
        match err {
            Error::InvalidArgs(_) => 1,
            Error::CouldNotSpawnProcess => 2,
            Error::ProcessFailed(_) => 4,
            Error::ProcessErrExit(code) => code,
            Error::Io(_) => 5, // TODO: Recheck
        }
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidArgs(msg) => write!(f, "Invalid arguments: {}", msg),
            Error::CouldNotSpawnProcess => write!(f, "Could not spawn child process"),
            Error::ProcessFailed(msg) => write!(f, "Process failed: {}", msg),
            Error::ProcessErrExit(code) => {
                write!(f, "Target command returned non-zero exit code: {}", code)
            }
            Error::Io(err) => write!(f, "{}", err),
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
            Ok(_) => {
                println!("\x1B[?1049l");
                0
            }
            Err(err) => {
                println!("\x1B[?1049l");
                eprintln!("{}", err);
                err.into()
            }
        }
    }
}
