use crate::formats::*;

use regex::Regex;
use std::{fs::File, io::{Read, Write}, iter::Peekable, str::Chars, thread};

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

fn read_quoted_char(c: char, chars: &mut Peekable<Chars>, field: &mut String) -> bool {
    if c != QUOTE {
        field.push(c);
        return true;
    }

    if chars.peek() == Some(&QUOTE) {
        chars.next();
        field.push(QUOTE);
        return true;
    }

    false
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut at_start = true;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if in_quotes {
            in_quotes = read_quoted_char(c, &mut chars, &mut field);
        } else if c == QUOTE && at_start {
            in_quotes = true;
            at_start = false;
        } else if c == CSV_DELIM {
            fields.push(std::mem::take(&mut field));
            at_start = true;
        } else {
            field.push(c);
            at_start = false;
        }
    }

    fields.push(field);
    fields
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
    let lines: Vec<String> = data.lines().map(|g| g.into()).collect();
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
