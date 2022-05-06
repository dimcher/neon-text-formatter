use neon::prelude::*;

use regex::Regex;

use std::{fs::File, io::{Read, Write}, {collections::HashMap}};

const COMMA: char = ',';
const QUOTE: char = '"';

struct Extensions {}

impl Extensions {
    fn new() -> HashMap<&'static str, char> {
        let mut map = HashMap::new();

        map.insert("csv", ',');
        map.insert("tsv", '\t');
        map.insert("psv", '|');

        map
    }
}

fn minmax(word: &str) -> [char; 2] {
    let w: Vec<char> = word.chars().collect();

    [w[0], w[w.len()-1]]
}

fn file_type(file: &String) -> String {
    let re = Regex::new(r"[^\.]+$").unwrap();
    let mat = re.find(&file).unwrap();
    
    file[mat.start()..mat.end()].to_string()
}

fn file_delim(mode: &str, map: &HashMap<&str, char>) -> char {
    *map.get(mode).unwrap_or(&COMMA)
}

fn read_file<'a>(file: &String) -> Box<String> {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    Box::new(contents)
}

fn write_file(file: String, text: &str) -> usize {
    let mut file = File::create(file).unwrap();
    let data = text.as_bytes();
    file.write_all(data).unwrap();

    text.len()
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

fn parse_csv(text: &str) -> Box<Vec<Vec<String>>> {
    let mut data: Vec<Vec<String>> = Vec::new();
    let re = Regex::new(r#""{2}"#).unwrap();

    let lines = text.lines().enumerate();

    for (_li, ln) in lines {
        let ln = re.replace(ln, String::from(QUOTE));

        let parts = ln.split(COMMA).enumerate();
        let last = parts.clone().count() - 1;

        let mut words: Vec<String> = Vec::new();
        let mut append: String = String::from("");

        for (wi, wd) in parts {
            let [min, max]: [char; 2] = minmax(wd);
            let beg: bool = min == QUOTE;
            let end: bool = max == QUOTE;

            if append.len() > 0 {
                append.push_str(&String::from(COMMA));
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

fn format_text(data: &Vec<Vec<String>>, delim: char) -> Box<String> {
    const EOL: &str = "\r\n";

    let len = data.len();
    let mut arr: Vec<String> = Vec::new();

    for i in 0..len {
        arr.push(data[i].join(&delim.to_string()));
    }
    
    Box::new(arr.join(EOL))
}

fn readtext(mut cx: FunctionContext) -> JsResult<JsString> {
    let name: Handle<JsString> = cx.argument(0)?;
    let file: String = name.value() as String;

    let text = read_file(&file);
    
    Ok(cx.string(*text))
}

fn writetext(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let name: Handle<JsString> = cx.argument(0)?;
    let cont: Handle<JsString> = cx.argument(1)?;

    let file: String = name.value() as String;
    let data: String = cont.value() as String;

    let size = write_file(file, &data);
    
    Ok(cx.number(size as f64))
}

fn convert(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let s: Handle<JsString> = cx.argument(0)?;
    let t: Handle<JsString> = cx.argument(1)?;

    let source: String = s.value() as String; 
    let target: String = t.value() as String;

    let mut data: Vec<Vec<String>> = Vec::new();
    let map: HashMap<&str, char> = Extensions::new();
    let stype = file_type(&source);
    let ttype = file_type(&target);
    let text = read_file(&source);

    if stype == "csv" {
        data = *parse_csv(&text);
    }

    let delim = file_delim(&ttype, &map);

    let text: Box<String> = format_text(&data, delim);
    let size: usize = write_file(target, &text);

    Ok(cx.number(size as f64))
}

register_module!(mut cx, {
    println!("Register local methods");

    cx.export_function("convert", convert)?;
    cx.export_function("readtext", readtext)?;
    cx.export_function("writetext", writetext)?;

    Ok(())
});
