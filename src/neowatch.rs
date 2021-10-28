use std::{io::{self, Write}, process::{Command, Stdio}, thread, time::Instant};

use termcolor::{Buffer, BufferWriter, ColorChoice, WriteColor};

use crate::{args::Args, error::Error};

const SPLIT_LINES: &str = "\n";
const SPLIT_WORDS: &str = " ";
const SIMILARITY_THRESHOLD: f32 = 0.5;

pub fn run(args: Args) -> Result<(), Error<'static>> {
    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    let mut last_data = String::new();
    loop {
        write!(&mut buffer, "\x1B[2J\x1B[1;1H")
            .map_err(|e| Error::IoError(e))?;
        let start = Instant::now();
        let process = Command::new(&args.cmd)
            .args(&args.cmd_args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|_| Error::CouldNotSpawnProcess)?;

        let output = process
            .wait_with_output()
            .map_err(|e| Error::ProcessFailed(format!("{:#?}", e.kind())))?;

        let data = String::from_utf8(output.stdout)
            .map_err(|_| Error::ProcessFailed("Invalid string was returned".to_string()))?;

        if args.show_diff {
            highlight_diffs(&mut buffer, &data, &last_data, &args)
                .map_err(|e| Error::IoError(e))?;
        } else {
            write!(&mut buffer, "{}", data)
                .map_err(|e| Error::IoError(e))?;
        }

        if args.exit_on_change && !last_data.is_empty() && data != last_data {
            return Ok(());
        }

        if args.exit_on_err && !output.status.success() {
            return Err(Error::ProcessErrExit(output.status.code().unwrap_or(-1)));
        }

        let sleep_duration = if args.precise_mode {
            args.interval.saturating_sub(start.elapsed())
        } else {
            args.interval
        };

        bufwtr.print(&buffer)
            .map_err(|e| Error::IoError(e))?;

        last_data = data;
        thread::sleep(sleep_duration);
    }
}

fn highlight_diffs<'a>(buffer: &mut Buffer, input: &'a str, last: &'a str, args: &Args) -> io::Result<()> {
    let last_lines: Vec<&str> = last.split(SPLIT_LINES).collect();

    for (idx, line) in input.split(SPLIT_LINES).enumerate() {
        if let Some((last_line, _)) = last_lines
            .iter()
            .enumerate()
            .map(|(i, l)| (l, similarity(line, l) / (1.0 + idx.abs_diff(i) as f32)))
            .max_by(|(_, s1), (_, s2)| s1.partial_cmp(s2).unwrap_or(std::cmp::Ordering::Equal))
        {
            for (idx, word) in line.split(SPLIT_WORDS).enumerate() {
                if idx != 0 {
                    write!(buffer, "{}", SPLIT_WORDS)?;
                };

                let last_word_similarity = last_line
                    .split(SPLIT_WORDS)
                    .enumerate()
                    .map(|(i, w)| similarity(w, word) / (1.0 + idx.abs_diff(i) as f32).sqrt())
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or(0.0);

                if last_word_similarity >= 1.0 {
                    write!(buffer, "{}", word)?;
                } else if last_word_similarity > SIMILARITY_THRESHOLD {
                    buffer.set_color(&args.color_change)?;
                    write!(buffer, "{}", word)?;
                    buffer.reset()?;
                } else {
                    buffer.set_color(&args.color_change)?;
                    write!(buffer, "{}", word)?;
                    buffer.reset()?;
                }
            }
            write!(buffer, "{}", SPLIT_LINES)?;
        };
    }
    Ok(())
}

fn similarity(a: &str, b: &str) -> f32 {
    let (length, same_count) = a
        .chars()
        .zip(b.chars())
        .fold((1, 1), |(total, matched), (x, y)| {
            (total + 1, if x == y { matched + 1 } else { matched })
        });
    same_count as f32 / length as f32
}
