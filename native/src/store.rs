#[path = "./formats.rs"]
mod formats;
use formats::*;

use regex::Regex;
use std::{fs::File, io::{Read, Write}, {thread}};

const EOL: &str = "\r\n";
const QUOTE: char = '"';
const THSIZE: usize = 100000;

fn min_max(word: &str) -> Box<[char; 2]> {
    let w: Vec<char> = word.chars().collect();

    Box::new([w[0], w[w.len()-1]])
}

pub fn file_type<'a>(file: &'a str) -> &'a str {
    let re = Regex::new(r"[^\.]+$").unwrap();
    let mat = re.find(file).unwrap();
    
    &file[mat.start()..mat.end()]
}

pub fn read_file<'a>(name: &str) -> Box<String> {
    let mut file = File::open(name).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    Box::new(contents)
}

pub fn write_file(file: &str, data: &str) -> usize {
    let mut file = File::create(file).unwrap();
    let data = data.as_bytes();
    file.write_all(data).unwrap();

    data.len()
}

fn auto_trim (text: &str, beg: bool, end: bool) -> Box<String> {
    let len: usize = text.len();
    let mut data = text.to_string();

    if len > 1 {
        if beg {
            data.remove(0);
        }
        if end {
            data.pop();
        }
    }

    Box::new(data)
}

pub fn parse_csv(lines: &Vec<String>) -> Box<Vec<Vec<String>>> {
    let mut data: Vec<Vec<String>> = Vec::new();
    let re = Regex::new(r#""{2}"#).unwrap();

    for line in lines {
        let ln = re.replace(line, String::from(QUOTE));

        let parts = ln.split(COMMA).enumerate();
        let last = parts.clone().count() - 1;

        let mut words: Vec<String> = Vec::new();
        let mut append: String = String::from("");

        for (wi, wd) in parts {
            let [mut min, mut max]: [char; 2] = [' ', ' '];

            if wd.len() > 0 {
                [min, max] = *min_max(wd);
            }

            let beg: bool = min == QUOTE;
            let end: bool = max == QUOTE;

            if append.len() > 0 {
                append.push_str(COMMA);
                append.push_str(wd);
                
                if end {
                    words.push(auto_trim(&append, true, true).to_string());
                    append = String::new();
                }
                else if wi == last {
                    words.push(auto_trim(&append, true, end).to_string());
                }

                continue;
            }

            if beg && !end && wi != last {
                append = String::from(wd);
                continue;
            }

            words.push(auto_trim(&wd.to_string(), beg, end).to_string());
        }
        
        data.push(words);
    }

    Box::new(data)
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

pub fn file_array(file: &str) -> Box<Vec<Vec<String>>> {
    let fmt = file_type(file);
    let data = read_file(file);
    let map = FileFormats::new();
    let delim = map.file_delim(&fmt);

    parse_text(&data, &delim)
}

pub fn conv_data<'a>(data: &Vec<Vec<String>>, delim: &str) -> Box<String> {
    let len = data.len();
    let mut arr: Vec<String> = Vec::new();

    for i in 0..len {
        arr.push(data[i].join(&delim.to_string()));
    }
    
    Box::new(arr.join(EOL))
}
