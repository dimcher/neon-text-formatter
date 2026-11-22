use crate::formats::Format;
use crate::io::write_file;
use crate::read::{file_array, parse_text};
use crate::write::conv_data;

fn write_rows(rows: &[Vec<String>], target: &str) -> Result<usize, String> {
    let format = Format::from_path(target)?;

    write_file(target, &conv_data(rows, format))
}

pub fn convert_file(source: &str, target: &str) -> Result<usize, String> {
    let rows = file_array(source)?;

    write_rows(&rows, target)
}

pub fn convert_text(text: &str, delim: &str, target: &str) -> Result<usize, String> {
    write_rows(&parse_text(text, delim), target)
}
