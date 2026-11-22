use crate::csv::QUOTE;
use crate::formats::*;

const EOL: &str = "\r\n";

fn quote_cell(cell: &str) -> String {
    format!("{}{}{}", QUOTE, cell.replace(QUOTE, "\"\""), QUOTE)
}

fn flat_cell(cell: &str, delim: &str) -> String {
    cell.replace("\r\n", " ").replace(['\r', '\n'], " ").replace(delim, " ")
}

fn write_cell(cell: &str, format: Format) -> String {
    match format {
        Format::Csv => quote_cell(cell),
        _ => flat_cell(cell, format.delim()),
    }
}

fn join_row(row: &[String], format: Format) -> String {
    let cells: Vec<String> = row.iter().map(|cell| write_cell(cell, format)).collect();

    cells.join(format.delim())
}

pub fn conv_data(data: &[Vec<String>], format: Format) -> String {
    let rows: Vec<String> = data.iter().map(|row| join_row(row, format)).collect();

    rows.join(EOL)
}
