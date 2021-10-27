use std::{
    process::{Command, Stdio},
    thread,
    time::Instant,
};

use crate::{args::Args, error::Error};

const COLOR_CHANGE: &str = "\x1b[36m";
const COLOR_ADD: &str = "\x1b[32m";
const COLOR_END: &str = "\x1b[0m";
const SPLIT_LINES: &str = "\n";
const SPLIT_WORDS: &str = " ";
const SIMILARITY_THRESHOLD: f32 = 0.5;

pub fn run(args: Args) -> Result<(), Error<'static>> {
    let mut last_data = String::new();
    loop {
        println!("\x1B[2J\x1B[1;1H");
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
            highlight_diffs(&data, &last_data)
                .into_iter()
                .for_each(|s| print!("{}", s));
            println!();
        } else {
            println!("{}", data)
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

        last_data = data;
        thread::sleep(sleep_duration);
    }
}

fn highlight_diffs<'a>(input: &'a str, last: &'a str) -> Vec<&'a str> {
    let last_lines: Vec<&str> = last.split(SPLIT_LINES).collect();
    let mut res: Vec<&str> = Vec::new();

    for line in input.split(SPLIT_LINES) {
        if let Some((last_line, _)) = last_lines
            .iter()
            .map(|l| (l, similarity(line, l)))
            .max_by(|(_, s1), (_, s2)| s1.partial_cmp(s2).unwrap_or(std::cmp::Ordering::Equal))
        {
            for (idx, word) in line.split(SPLIT_WORDS).enumerate() {
                if idx != 0 {
                    res.push(SPLIT_WORDS)
                };

                let last_word_similarity = last_line
                    .split(SPLIT_WORDS)
                    .enumerate()
                    .map(|(i, w)| similarity(w, word) / (1.0 + idx.abs_diff(i) as f32).sqrt())
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap_or(0.0);

                if last_word_similarity >= 1.0 {
                    res.push(word)
                } else if last_word_similarity > SIMILARITY_THRESHOLD {
                    res.push(COLOR_CHANGE);
                    res.push(word);
                    res.push(COLOR_END);
                } else {
                    res.push(COLOR_ADD);
                    res.push(word);
                    res.push(COLOR_END);
                }
            }
            res.push(SPLIT_LINES);
        };
    }

    res
}

fn similarity(a: &str, b: &str) -> f32 {
    let length = if a.len() > b.len() { a.len() } else { b.len() };
    let same_count = a.chars().zip(b.chars()).filter(|(x, y)| x == y).count();
    same_count as f32 / length as f32
}
