use crate::formats::Format;
use crate::io::write_file;
use crate::read::{file_array, parse_text};
use crate::write::conv_data;

fn release(rows: Vec<Vec<String>>) {
    std::thread::spawn(move || drop(rows));
}

fn write_rows(rows: Vec<Vec<String>>, target: &str) -> Result<usize, String> {
    let format = Format::from_path(target)?;
    let text = conv_data(&rows, format);

    release(rows);

    write_file(target, &text)
}

pub fn convert_file(source: &str, target: &str) -> Result<usize, String> {
    let rows = file_array(source)?;

    write_rows(rows, target)
}

fn check_delim(delim: &str) -> Result<(), String> {
    if delim.chars().count() == 1 {
        return Ok(());
    }

    Err(format!("Delimiter must be one character, got '{}'", delim))
}

pub fn convert_text(text: &str, delim: &str, target: &str) -> Result<usize, String> {
    check_delim(delim)?;
    write_rows(parse_text(text, delim), target)
}
