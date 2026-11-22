use std::thread;

use crate::csv::{csv_records, parse_csv, trim_cr};
use crate::formats::*;
use crate::io::read_file;

const MIN_CHUNK: usize = 20000;

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

fn worker_count() -> usize {
    thread::available_parallelism().map_or(1, |count| count.get())
}

pub fn chunk_size(total: usize, workers: usize) -> usize {
    let even = total.div_ceil(workers.max(1));

    even.max(MIN_CHUNK)
}

fn parse_chunks(lines: &[String], delim: &str, size: usize) -> Vec<Vec<String>> {
    thread::scope(|scope| {
        let handles: Vec<_> = lines
            .chunks(size)
            .map(|chunk| scope.spawn(move || run_thread(chunk, delim)))
            .collect();

        handles
            .into_iter()
            .flat_map(|handle| handle.join().unwrap())
            .collect()
    })
}

fn parse_records(lines: &[String], delim: &str, size: usize) -> Vec<Vec<String>> {
    if lines.len() <= size {
        return run_thread(lines, delim);
    }

    parse_chunks(lines, delim, size)
}

pub fn parse_text_sized(data: &str, delim: &str, size: usize) -> Vec<Vec<String>> {
    parse_records(&split_records(data, delim), delim, size)
}

pub fn parse_text(data: &str, delim: &str) -> Vec<Vec<String>> {
    let lines = split_records(data, delim);
    let size = chunk_size(lines.len(), worker_count());

    parse_records(&lines, delim, size)
}

pub fn file_array(file: &str) -> Result<Vec<Vec<String>>, String> {
    let format = Format::from_path(file)?;
    let data = read_file(file)?;

    Ok(parse_text(&data, format.delim()))
}
