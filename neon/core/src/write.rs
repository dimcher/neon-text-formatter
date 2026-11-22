use std::thread;

use crate::csv::QUOTE;
use crate::formats::*;
use crate::read::{chunk_size, worker_count};

const EOL: &str = "\r\n";
const SPACE: char = ' ';

fn push_quoted(out: &mut String, cell: &str) {
    out.push(QUOTE);

    for (index, part) in cell.split(QUOTE).enumerate() {
        if index > 0 {
            out.push_str("\"\"");
        }

        out.push_str(part);
    }

    out.push(QUOTE);
}

fn needs_flattening(cell: &str, delim: char) -> bool {
    cell.chars().any(|c| c == '\r' || c == '\n' || c == delim)
}

fn push_flat_chars(out: &mut String, cell: &str, delim: char) {
    let mut chars = cell.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\r' && chars.peek() == Some(&'\n') {
            chars.next();
        }

        out.push(if c == '\r' || c == '\n' || c == delim { SPACE } else { c });
    }
}

fn push_flat(out: &mut String, cell: &str, delim: char) {
    if needs_flattening(cell, delim) {
        push_flat_chars(out, cell, delim);
    } else {
        out.push_str(cell);
    }
}

fn push_cell(out: &mut String, cell: &str, format: Format) {
    match format {
        Format::Csv => push_quoted(out, cell),
        _ => push_flat(out, cell, delim_char(format)),
    }
}

fn delim_char(format: Format) -> char {
    format.delim().chars().next().unwrap_or(SPACE)
}

fn push_row(out: &mut String, row: &[String], format: Format) {
    for (index, cell) in row.iter().enumerate() {
        if index > 0 {
            out.push_str(format.delim());
        }

        push_cell(out, cell, format);
    }
}

fn estimated_len(rows: &[Vec<String>]) -> usize {
    rows.iter()
        .take(64)
        .map(|row| row.iter().map(|cell| cell.len() + 3).sum::<usize>() + EOL.len())
        .sum::<usize>()
        / rows.len().clamp(1, 64)
        * rows.len()
}

fn write_rows(rows: &[Vec<String>], format: Format) -> String {
    let mut out = String::with_capacity(estimated_len(rows));

    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            out.push_str(EOL);
        }

        push_row(&mut out, row, format);
    }

    out
}

fn write_chunks(data: &[Vec<String>], format: Format, size: usize) -> String {
    let parts: Vec<String> = thread::scope(|scope| {
        let handles: Vec<_> = data
            .chunks(size)
            .map(|chunk| scope.spawn(move || write_rows(chunk, format)))
            .collect();

        handles.into_iter().map(|handle| handle.join().unwrap()).collect()
    });

    parts.join(EOL)
}

pub fn conv_data_sized(data: &[Vec<String>], format: Format, size: usize) -> String {
    if data.len() <= size {
        return write_rows(data, format);
    }

    write_chunks(data, format, size)
}

pub fn conv_data(data: &[Vec<String>], format: Format) -> String {
    conv_data_sized(data, format, chunk_size(data.len(), worker_count()))
}
