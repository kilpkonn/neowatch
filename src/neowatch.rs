use std::{
    io::ErrorKind,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use crate::{args::Args, error::Error};

pub trait ChildProcess {
    fn wait_or_timeout(&mut self, timeout: Duration) -> Result<i32, ErrorKind>;
}

impl ChildProcess for Child {
    fn wait_or_timeout(&mut self, timeout: Duration) -> Result<i32, ErrorKind> {
        let start = Instant::now();

        // try_wait() does not drop stdin unlike wait and may cause deadlock
        drop(self.stdin.take());
        loop {
            match self.try_wait() {
                Ok(Some(status)) => return Ok(status.code().unwrap_or(0)),
                Err(err) => return Err(err.kind()),
                _ => {}
            };

            if start.elapsed() >= timeout {
                break;
            }

            thread::sleep(Duration::from_millis(100));
        }

        return Err(ErrorKind::TimedOut);
    }
}

pub fn run(args: Args) -> Result<(), Error<'static>> {
    loop {
        let mut process = Command::new(&args.cmd)
            .args(&args.cmd_args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|_| Error::CouldNotSpawnProcess)?;

        match process.wait_or_timeout(args.interval) {
            Ok(_) => {
                println!("\x1B[2J\x1B[1;1H");
            }
            Err(err) => match err {
                // ErrorKind::TimedOut => return Err(Error::ProcessFailed("Timed out")),
                err => return Err(Error::ProcessFailed(format!("{:?}", err))),
            },
        };
    }
}
