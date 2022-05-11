#[path = "./formats.rs"]
mod formats;
use formats::*;

use regex::Regex;
use std::collections::HashMap;
use std::{fs::File, io::{Read, Write}, {thread}};

const EOL: &str = "\r\n";
const COMMA: &str = ",";
const QUOTE: char = '"';
const THSIZE: usize = 3;

/* 
trait Threads {
    fn run(&self, order: &usize);
}

enum List {
    Node(i32, Box<List>),
//    Func,
    None,
}

struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}
 */
/* impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {
            element: element,
            next: None,
        }
    }

    fn append(&mut self, element: Box<Node<T>>) {
        self.next = Some(element);
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    len: u32,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        head: None,
        tail: None,
        len: 0,
    }

    pub fn push(&mut self, element: T) {
        let node: Box<Node<T>> = Box::new(Node::new(element));

        match self.tail {
            None => self.head = Some(node),
            Some(mut ref tail) => tail.append(node),
        }
        self.tail = Some(node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        //not implemented
    }

    pub fn get(&self, index: u32) -> Option<T> {
        //not implemented
    }
}
 */

fn min_max(word: &str) -> Box<[char; 2]> {
    let w: Vec<char> = word.chars().collect();

    Box::new([w[0], w[w.len()-1]])
}

pub fn file_formats<'a>() -> Box<HashMap<&'static str, &'static str>> {
    let map = FileFormats::new();
    map.get_map()
}

pub fn file_type(file: &str) -> Box<String> {
    let re = Regex::new(r"[^\.]+$").unwrap();
    let mat = re.find(file).unwrap();
    
    Box::new(file[mat.start()..mat.end()].to_string())
}

pub fn file_delim<'a>(mode: &str) -> Box<String> {
    let map = file_formats();
    Box::new(map.get(mode).unwrap_or(&COMMA).to_string())
}

pub fn read_file<'a>(file: &str) -> Box<String> {
    let mut file = File::open(file).unwrap();
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

fn auto_trim (mut text: String, beg: bool, end: bool) -> Box<String> {
    let len: usize = text.len();

    if len > 1 {
        if beg {
            text.remove(0);
        }
        if end {
            text.pop();
        }
    }

    Box::new(text)
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
                    append = *auto_trim(append, true, true);
                    words.push(append);
                    append = String::new();
                }
                else if wi == last {
                    words.push(*auto_trim(append.clone(), true, end));
                }

                continue;
            }

            if beg && !end && wi != last {
                append = String::from(wd);
                continue;
            }

            words.push(*auto_trim(String::from(wd), beg, end));
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
    let lsize: usize = lines.len();
    let csize = lsize / THSIZE;
    let chunks: Vec<Vec<String>> = lines.chunks(THSIZE).map(|m| m.into()).collect();
    let mut handles = vec![];
    
//    type aaa = List;
//    let a = aaa::Node(1, Box::new(List::Node(2, Box::new(List::None))));

    for i in 0..csize {
//        println!("Chunk={:?}={}", chunks[i], i);
        let dl = String::from(delim).clone();
        let ch = chunks[i].clone();

        let handle = thread::spawn(move || {
            run_thread(&ch, &dl)
        });

        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        println!("Result={:?}", result);
    }

        println!("Finished:{:?}", "dimcher");

    Box::new(Vec::new())
}

pub fn file_array(file: &str) -> Box<Vec<Vec<String>>> {
    let fmt = file_type(file);
    let data = read_file(file);
    let delim = file_delim(&fmt);

    parse_text(&data, &delim)
}

pub fn conv_data(data: &Vec<Vec<String>>, delim: &str) -> Box<String> {
    let len = data.len();
    let mut arr: Vec<String> = Vec::new();

    for i in 0..len {
        arr.push(data[i].join(&delim.to_string()));
    }
    
    Box::new(arr.join(EOL))
}
