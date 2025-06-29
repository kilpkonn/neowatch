use std::fmt::Display;
use std::io;
use std::process::{ExitCode, Termination};

pub enum Error<'a> {
    InvalidArgs(&'a str),
    CouldNotSpawnProcess,
    CouldNotSetSignalHandler,
    ProcessFailed(String),
    ProcessErrExit(i32),
    Io(io::Error),
}

impl From<Error<'_>> for ExitCode {
    fn from(err: Error) -> Self {
        match err {
            Error::InvalidArgs(_) => ExitCode::from(1),
            Error::CouldNotSpawnProcess => ExitCode::from(2),
            Error::CouldNotSetSignalHandler => ExitCode::from(1),
            Error::ProcessFailed(_) => ExitCode::from(4),
            // TODO: Properly handle not so common codes
            Error::ProcessErrExit(code) => ExitCode::from(code as u8),
            Error::Io(_) => ExitCode::from(5), // TODO: Recheck
        }
    }
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidArgs(msg) => write!(f, "Invalid arguments: {}", msg),
            Error::CouldNotSpawnProcess => write!(f, "Could not spawn child process"),
            Error::CouldNotSetSignalHandler => write!(f, "Could not set signal handler"),
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

impl Termination for Exit<'_> {
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(_) => {
                println!("\x1B[?1049l");
                ExitCode::SUCCESS
            }
            Err(err) => {
                println!("\x1B[?1049l");
                eprintln!("{}", err);
                err.into()
            }
        }
    }
}
