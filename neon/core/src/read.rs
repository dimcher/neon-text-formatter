use std::thread;

use crate::csv::{csv_records, parse_csv, trim_cr};
use crate::formats::*;
use crate::io::read_file;

const THSIZE: usize = 200000;

pub fn parse_data(lines: &[String], delim: &str) -> Vec<Vec<String>> {
    lines
        .iter()
        .map(|line| line.split(delim).map(String::from).collect())
        .collect()
}

fn split_records(data: &str, delim: &str) -> Vec<String> {
    if delim == COMMA {
        return csv_records(data);
    }

    data.lines().map(|line| trim_cr(line).to_string()).collect()
}

fn run_thread(lines: &[String], delim: &str) -> Vec<Vec<String>> {
    if delim == COMMA {
        return parse_csv(lines);
    }

    parse_data(lines, delim)
}

pub fn parse_text(data: &str, delim: &str) -> Vec<Vec<String>> {
    let lines = split_records(data, delim);

    thread::scope(|scope| {
        let handles: Vec<_> = lines
            .chunks(THSIZE)
            .map(|chunk| scope.spawn(move || run_thread(chunk, delim)))
            .collect();

        handles
            .into_iter()
            .flat_map(|handle| handle.join().unwrap())
            .collect()
    })
}

pub fn file_array(file: &str) -> Result<Vec<Vec<String>>, String> {
    let format = Format::from_path(file)?;
    let data = read_file(file)?;

    Ok(parse_text(&data, format.delim()))
}
