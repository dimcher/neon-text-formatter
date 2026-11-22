use crate::formats::*;

use regex::Regex;
use std::{fs::File, io::{Read, Write}, thread};

const EOL: &str = "\r\n";
const QUOTE: char = '"';
const CSV_DELIM: char = ',';
const THSIZE: usize = 200000;

pub fn file_type<'a>(file: &'a str) -> Result<&'a str, String> {
    let re = Regex::new(r"[^\.]+$").unwrap();

    re.find(file)
        .map(|mat| &file[mat.start()..mat.end()])
        .ok_or_else(|| format!("Cannot find a file extension in '{}'", file))
}

pub fn read_file(name: &str) -> Result<String, String> {
    let mut text = String::new();

    File::open(name)
        .and_then(|mut f| f.read_to_string(&mut text))
        .map_err(|err| format!("Cannot read {}: {}", name, err))?;

    Ok(text)
}

pub fn write_file(file: &str, data: &str) -> Result<usize, String> {
    let bytes = data.as_bytes();

    File::create(file)
        .and_then(|mut f| f.write_all(bytes))
        .map_err(|err| format!("Cannot write {}: {}", file, err))?;

    Ok(bytes.len())
}

#[derive(PartialEq)]
enum CsvEvent {
    Open,
    Close,
    Escaped,
    Delim,
    Char,
}

struct CsvState {
    in_quotes: bool,
    at_start: bool,
}

impl CsvState {
    fn new() -> Self {
        CsvState { in_quotes: false, at_start: true }
    }

    fn next(&mut self, c: char, upcoming: Option<char>) -> CsvEvent {
        if self.in_quotes {
            return self.next_quoted(c, upcoming);
        }

        self.next_plain(c)
    }

    fn next_quoted(&mut self, c: char, upcoming: Option<char>) -> CsvEvent {
        if c != QUOTE {
            return CsvEvent::Char;
        }

        if upcoming == Some(QUOTE) {
            return CsvEvent::Escaped;
        }

        self.in_quotes = false;
        CsvEvent::Close
    }

    fn next_plain(&mut self, c: char) -> CsvEvent {
        if c == QUOTE && self.at_start {
            self.in_quotes = true;
            self.at_start = false;
            return CsvEvent::Open;
        }

        if c == CSV_DELIM {
            self.at_start = true;
            return CsvEvent::Delim;
        }

        self.at_start = false;
        CsvEvent::Char
    }
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut state = CsvState::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match state.next(c, chars.peek().copied()) {
            CsvEvent::Open | CsvEvent::Close => {}
            CsvEvent::Escaped => {
                chars.next();
                field.push(QUOTE);
            }
            CsvEvent::Delim => fields.push(std::mem::take(&mut field)),
            CsvEvent::Char => field.push(c),
        }
    }

    fields.push(field);
    fields
}

struct Record {
    start: usize,
    end: usize,
    next: usize,
}

struct Scan {
    record: Record,
    closed: bool,
}

fn scan_record(data: &str, start: usize) -> Scan {
    let mut state = CsvState::new();
    let mut chars = data[start..].char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        let upcoming = chars.peek().map(|(_, ch)| *ch);

        if state.next(c, upcoming) == CsvEvent::Escaped {
            chars.next();
        } else if c == '\n' && !state.in_quotes {
            let record = Record { start, end: start + i, next: start + i + 1 };
            return Scan { record, closed: true };
        }
    }

    let record = Record { start, end: data.len(), next: data.len() };
    Scan { record, closed: !state.in_quotes }
}

fn first_line_bounds(data: &str, start: usize) -> Record {
    match data[start..].find('\n') {
        Some(i) => Record { start, end: start + i, next: start + i + 1 },
        None => Record { start, end: data.len(), next: data.len() },
    }
}

fn next_record(data: &str, start: usize) -> Record {
    let scan = scan_record(data, start);

    if scan.closed {
        return scan.record;
    }

    first_line_bounds(data, start)
}

fn trim_cr(text: &str) -> &str {
    text.strip_suffix('\r').unwrap_or(text)
}

fn record_text(data: &str, record: &Record) -> String {
    let text = &data[record.start..record.end];

    if record.next > record.end {
        return trim_cr(text).to_string();
    }

    text.to_string()
}

fn csv_records(data: &str) -> Vec<String> {
    let mut records: Vec<String> = Vec::new();
    let mut start = 0;

    while start < data.len() {
        let record = next_record(data, start);

        records.push(record_text(data, &record));
        start = record.next;
    }

    records
}

fn split_records(data: &str, delim: &str) -> Vec<String> {
    if delim == COMMA {
        return csv_records(data);
    }

    data.lines().map(|g| g.into()).collect()
}

pub fn parse_csv(lines: &Vec<String>) -> Box<Vec<Vec<String>>> {
    Box::new(lines.iter().map(|line| parse_csv_line(line)).collect())
}

pub fn parse_data(lines: &Vec<String>, delim: &str) -> Box<Vec<Vec<String>>> {
    let mut data: Vec<Vec<String>> = Vec::new();

    for line in lines {
        let vec = line.split(delim).map(String::from).collect();
        data.push(vec);
    }

    Box::new(data)
}

fn run_thread(data: &Vec<String>, delim: &str) -> Box<Vec<Vec<String>>> {
    if delim == COMMA {
        return parse_csv(data);
    }

    parse_data(data, delim)
}

pub fn parse_text(data: &str, delim: &str) -> Box<Vec<Vec<String>>> {
    let lines: Vec<String> = split_records(data, delim);
    let chunks: Vec<Vec<String>> = lines.chunks(THSIZE).map(|m| m.into()).collect();
    let mut vec = vec![];

    let mut handles = vec![];

    for c in chunks {
        let dl = delim.to_string();
        let handle = thread::spawn(move || run_thread(&c, &dl));

        handles.push(handle);
    }

    for h in handles {
        let res = *h.join().unwrap();
        let mut r = res;

        vec.append(&mut r);
    }

    Box::new(vec)
}

pub fn file_array(file: &str) -> Result<Box<Vec<Vec<String>>>, String> {
    let fmt = file_type(file)?;
    let data = read_file(file)?;
    let map = FileFormats::new();
    let delim = map.get_delim(&fmt);

    Ok(parse_text(&data, &delim))
}

pub fn conv_data<'a>(data: &Vec<Vec<String>>, key: &str) -> Box<String> {
    let map = FileFormats::new();
    let dlm = map.get_delim(key);
    let len = data.len();
    let mut arr: Vec<String> = Vec::new();

    if key == "csv" {
        for i in 0..len {
            let mut line: Vec<String> = Vec::new();

            for j in 0..data[i].len() {
                line.push(format!("{}{}{}", QUOTE, data[i][j].replace(QUOTE, "\"\""), QUOTE));
            }

            arr.push(line.join(&dlm.to_string()));
        }
    }
    else {
        for i in 0..len {
            arr.push(data[i].join(&dlm.to_string()));
        }
    }

    Box::new(arr.join(EOL))
}
