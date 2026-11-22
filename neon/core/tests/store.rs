use std::{env, fs, path::PathBuf, process};

use text_formatter_core::formats::*;
use text_formatter_core::store::*;

fn lines(rows: &[&str]) -> Vec<String> {
    rows.iter().map(|row| row.to_string()).collect()
}

fn rows(data: &[&[&str]]) -> Vec<Vec<String>> {
    data.iter().map(|row| lines(row)).collect()
}

fn temp_path(name: &str) -> PathBuf {
    env::temp_dir().join(format!("ntf-{}-{}", process::id(), name))
}

fn temp_file(name: &str, text: &str) -> String {
    let path = temp_path(name);
    fs::write(&path, text).unwrap();
    path.to_string_lossy().into_owned()
}

#[test]
fn file_type_returns_the_extension() {
    assert_eq!(file_type("dir/data.csv"), "csv");
    assert_eq!(file_type("archive.tar.tsv"), "tsv");
}

#[test]
fn get_delim_maps_known_types_and_defaults_to_comma() {
    let formats = FileFormats::new();

    assert_eq!(formats.get_delim("tsv"), TAB);
    assert_eq!(formats.get_delim("psv"), PIPE);
    assert_eq!(formats.get_delim("csv"), COMMA);
    assert_eq!(formats.get_delim("txt"), COMMA);
}

#[test]
fn parse_data_splits_each_line_on_the_delimiter() {
    let result = parse_data(&lines(&["a|b|c", "1|2|3"]), PIPE);

    assert_eq!(*result, rows(&[&["a", "b", "c"], &["1", "2", "3"]]));
}

#[test]
fn parse_csv_strips_quotes_and_keeps_unquoted_fields() {
    let result = parse_csv(&lines(&["\"a\",b,\"c\""]));

    assert_eq!(*result, rows(&[&["a", "b", "c"]]));
}

#[test]
fn parse_csv_keeps_commas_inside_quoted_fields() {
    let result = parse_csv(&lines(&["\"a,b\",\"c, d, e\",f"]));

    assert_eq!(*result, rows(&[&["a,b", "c, d, e", "f"]]));
}

#[test]
fn parse_csv_unescapes_a_doubled_quote() {
    let result = parse_csv(&lines(&["\"say \"\"hi\",x"]));

    assert_eq!(*result, rows(&[&["say \"hi", "x"]]));
}

#[test]
#[ignore = "known bug: Regex::replace only unescapes the first doubled quote on a line"]
fn parse_csv_unescapes_every_doubled_quote() {
    let result = parse_csv(&lines(&["\"a\"\"b\",\"c\"\"d\""]));

    assert_eq!(*result, rows(&[&["a\"b", "c\"d"]]));
}

#[test]
#[ignore = "known bug: an empty quoted field comes back as a single quote"]
fn parse_csv_reads_an_empty_quoted_field_as_empty() {
    let result = parse_csv(&lines(&["\"a\",\"\",\"b\""]));

    assert_eq!(*result, rows(&[&["a", "", "b"]]));
}

#[test]
fn parse_text_uses_csv_rules_for_commas() {
    let result = parse_text("\"a,b\",c\n1,2", COMMA);

    assert_eq!(*result, rows(&[&["a,b", "c"], &["1", "2"]]));
}

#[test]
fn parse_text_keeps_row_order_across_thread_chunks() {
    let total = 450_000;
    let text: Vec<String> = (0..total).map(|n| format!("{}\tx", n)).collect();

    let result = parse_text(&text.join("\n"), TAB);

    assert_eq!(result.len(), total);
    assert!(result.iter().enumerate().all(|(n, row)| row[0] == n.to_string()));
}

#[test]
fn conv_data_quotes_every_csv_field_and_joins_with_crlf() {
    let text = conv_data(&rows(&[&["a", "b"], &["1", "2"]]), "csv");

    assert_eq!(*text, "\"a\",\"b\"\r\n\"1\",\"2\"");
}

#[test]
fn conv_data_joins_tsv_and_psv_without_quotes() {
    let data = rows(&[&["a", "b"], &["1", "2"]]);

    assert_eq!(*conv_data(&data, "tsv"), "a\tb\r\n1\t2");
    assert_eq!(*conv_data(&data, "psv"), "a|b\r\n1|2");
}

#[test]
#[ignore = "known bug: quotes inside a csv value are not doubled on write"]
fn conv_data_escapes_quotes_inside_csv_values() {
    let text = conv_data(&rows(&[&["say \"hi\""]]), "csv");

    assert_eq!(*text, "\"say \"\"hi\"\"\"");
}

#[test]
fn write_file_then_read_file_round_trips_and_counts_bytes() {
    let path = temp_path("round-trip.txt");
    let file = path.to_string_lossy();

    assert_eq!(write_file(&file, "héllo"), Ok(6));
    assert_eq!(read_file(&file), Ok("héllo".to_string()));

    fs::remove_file(path).unwrap();
}

#[test]
fn read_file_reports_a_missing_file() {
    let error = read_file("/no/such/dir/missing.csv").unwrap_err();

    assert!(error.starts_with("Cannot read /no/such/dir/missing.csv"), "{}", error);
}

#[test]
fn write_file_reports_a_missing_directory() {
    let error = write_file("/no/such/dir/out.csv", "x").unwrap_err();

    assert!(error.starts_with("Cannot write /no/such/dir/out.csv"), "{}", error);
}

#[test]
fn file_array_picks_the_delimiter_from_the_extension() {
    let file = temp_file("input.psv", "a|b\n1|2");

    let result = file_array(&file).unwrap();

    assert_eq!(*result, rows(&[&["a", "b"], &["1", "2"]]));
    fs::remove_file(file).unwrap();
}

#[test]
fn file_array_reports_a_missing_file() {
    assert!(file_array("/no/such/dir/missing.tsv").is_err());
}
