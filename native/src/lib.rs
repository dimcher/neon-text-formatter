use neon::prelude::*;

use std::fs::File;
use std::io::{Read, Write, Result};

fn vec_string(vec: &Vec<char>) -> String {
    vec.iter().cloned().collect::<String>()
}

fn read_file(filename: String) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn write_file(filename: String, context: &str) -> Result<usize> {
    let mut file = File::create(filename)?;
    let text = context.as_bytes();
    file.write_all(text)?;

    Ok(context.len())
}

fn parse_csv(text: &str) -> Vec<Vec<String>> {
    const DCHAR: char = 'a';
    const COMMA: char = ',';
    const QUOTE: char = '"';

    let mut data: Vec<Vec<String>> = Vec::new();
    let mut line: Vec<String> = Vec::new();
    let mut word: Vec<char> = Vec::new();

    let lines = text.lines().enumerate();

    for (_li, ln) in lines {
        let mut open: bool = false;
        let mut prev: char = DCHAR;

        let len: usize = ln.len();
        let last: usize = len - 1;
        let chrs = ln.chars().enumerate();

        for (ci, cr) in chrs {
            if ci == 0 && ci == last { // one caracter string
                word.push(cr);
                line.push(vec_string(&word));
            }
            else if ci == 0 {        // beg of line
                if cr == COMMA {
                    line.push(vec_string(&word));
                }
                else if cr == QUOTE {
                    open = true;
                }
                else {
                    word.push(cr);
                }
            }
            else if ci == last {      // end of line
                if open {           // QUOTEs opened
                    if cr == COMMA {
                        word.insert(0, QUOTE);
                        line.push(vec_string(&word));
                        word.clear();
                    }
                    else if cr != QUOTE {
                        word.push(cr);
                    }

                    line.push(vec_string(&word));
                }               
                else {              // no QUOTEs opened
                    if cr == COMMA {
                        line.push(vec_string(&word));
                        word.clear();
                        line.push(vec_string(&word));
                    }
                    else {
                        word.push(cr);
                        line.push(vec_string(&word));
                    }
                }
            }
            else {                  // any other position
                if open {
                    if cr == QUOTE && prev == QUOTE {
                        word.push(cr);
                    } 
                    else if cr == QUOTE {
                        open = false;
                    }
                    else if cr != QUOTE {
                        word.push(cr);
                    }
                } else {
                    if cr == QUOTE && prev == QUOTE {
                        word.push(cr);
                    } 
                    else if cr == COMMA && prev == QUOTE {
                        line.push(vec_string(&word));
                        word.clear();
                    }
                    else if cr == COMMA {
                        line.push(vec_string(&word));
                        word.clear();
                    }
                    else if cr == QUOTE && prev == COMMA {
                        open = true;
                    }
                    else if cr != QUOTE {
                        word.push(cr);
                    }
                }
            }

            prev = cr;
        }

        data.push(line.clone());
        word.clear();
        line.clear();

    }
//    println!("= {:?}", data);
    data
}

fn format_text(data: &Vec<Vec<String>>, delim: String) -> String {
    const EOL: &str = "\r\n";

    let len = data.len();
    let mut arr: Vec<String> = Vec::new();

    for i in 0..len {
        arr.push(data[i].join(&delim));
    }
    
    arr.join(EOL)
}

fn readtext(mut cx: FunctionContext) -> JsResult<JsString> {
    let name: Handle<JsString> = cx.argument(0)?;
    let file: String = name.value() as String;

    let res = read_file(file);
    
    match res.ok() {
        Some(text) => {
            println!("{:?}", text.len());
            return Ok(cx.string(text.to_string()));    
        },
        _ => {
            println!("{:?}", "none is ready");
            return Ok(cx.string("".to_string()));    
        }
    }
}

fn writetext(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let name: Handle<JsString> = cx.argument(0)?;
    let cont: Handle<JsString> = cx.argument(1)?;

    let file: String = name.value() as String;
    let data: String = cont.value() as String;

    let res = write_file(file, &data);
    
    match res.ok() {
        Some(size) => {
            return Ok(cx.number(size as f64));    
        },
        _ => {
            return Ok(cx.number(0));    
        }
    }
}

fn convert(mut cx: FunctionContext) -> JsResult<JsString> {
    let targ: Handle<JsString> = cx.argument(0)?;
    let darg: Handle<JsString> = cx.argument(1)?;

    let text: String = targ.value() as String; 
    let delim: String = darg.value() as String; 

    let data: Vec<Vec<String>> = parse_csv(&text);
    let tsv: String = format_text(&data, delim);

    Ok(cx.string(tsv))
}

register_module!(mut cx, {
    println!("Register local methods");

    cx.export_function("convert", convert)?;
    cx.export_function("readtext", readtext)?;
    cx.export_function("writetext", writetext)?;
    Ok(())
});
