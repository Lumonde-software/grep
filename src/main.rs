use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use ansi_term::Style;
// use atty::Stream;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// To find lines which don't have the pattern
    #[arg(short, long, default_value_t = false)]
    v: bool,

    /// Case insensitive
    #[arg(short, long, default_value_t = false)]
    i: bool,
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    let styled_pattern = Style::new()
        .fg(ansi_term::Color::Red)
        .paint(&args.pattern)
        .to_string();
    for file_name in args.path {
        let f = File::open(file_name).unwrap();
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = line.unwrap();
            let line = find_pattern(&args.pattern, &styled_pattern, line, args.v, args.i);
            match line {
                Some(line) => println!("{}", line),
                None => continue,
            }
        }
    }
}

fn find_pattern(
    pattern: &str,
    styled_pattern: &str,
    line: String,
    v: bool,
    i: bool,
) -> Option<String> {
    if line.trim().is_empty() {
        return None;
    }
    if i {
        if line.to_lowercase().contains(&pattern.to_lowercase()) {
            if !v {
                let idx: Vec<usize> = line
                    .to_lowercase()
                    .match_indices(&pattern.to_lowercase())
                    .map(|(index, _)| index)
                    .collect();
                let length = pattern.len();
                let mut line = line;
                for i in idx.iter().rev() {
                    let pattern = &line[*i..*i + length];
                    let styled_pattern = Style::new()
                        .fg(ansi_term::Color::Red)
                        .paint(pattern)
                        .to_string();
                    line.replace_range(*i..*i + length, &styled_pattern);
                }
                return Some(line);
            }
        } else if v {
            return Some(line);
        }
    } else if line.contains(pattern) {
        if !v {
            let line = line.replace(pattern, styled_pattern);
            return Some(line);
        }
    } else if v {
        return Some(line);
    }
    None
}
