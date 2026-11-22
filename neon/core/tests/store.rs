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
    assert_eq!(file_type("dir/data.csv"), Ok("csv"));
    assert_eq!(file_type("archive.tar.tsv"), Ok("tsv"));
}

#[test]
fn file_type_reports_a_name_without_an_extension_instead_of_panicking() {
    assert!(file_type("data.").unwrap_err().starts_with("Cannot find a file extension"));
    assert!(file_type("").is_err());
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
fn parse_csv_unescapes_every_doubled_quote() {
    let result = parse_csv(&lines(&["\"a\"\"b\",\"c\"\"d\""]));

    assert_eq!(*result, rows(&[&["a\"b", "c\"d"]]));
}

#[test]
fn parse_csv_reads_an_empty_quoted_field_as_empty() {
    let result = parse_csv(&lines(&["\"a\",\"\",\"b\""]));

    assert_eq!(*result, rows(&[&["a", "", "b"]]));
}

#[test]
fn parse_csv_keeps_the_text_of_a_quote_that_never_closes() {
    let result = parse_csv(&lines(&["\"a,b", "x,\"y"]));

    assert_eq!(*result, rows(&[&["a,b"], &["x", "y"]]));
}

#[test]
fn parse_csv_keeps_a_quote_in_the_middle_of_an_unquoted_field() {
    let result = parse_csv(&lines(&["5\" nails,b"]));

    assert_eq!(*result, rows(&[&["5\" nails", "b"]]));
}

#[test]
fn parse_csv_keeps_empty_fields() {
    let result = parse_csv(&lines(&[",a,,", ""]));

    assert_eq!(*result, rows(&[&["", "a", "", ""], &[""]]));
}

#[test]
fn parse_text_keeps_a_quoted_field_that_spans_lines() {
    let result = parse_text("\"a\nb\",c", COMMA);

    assert_eq!(*result, rows(&[&["a\nb", "c"]]));
}

#[test]
fn parse_text_keeps_crlf_inside_a_quoted_field() {
    let result = parse_text("\"a\r\nb\",c\r\n1,2\r\n", COMMA);

    assert_eq!(*result, rows(&[&["a\r\nb", "c"], &["1", "2"]]));
}

#[test]
fn parse_text_keeps_several_blank_lines_inside_a_quoted_field() {
    let result = parse_text("x,\"a\n\n\nb\"\ny,z", COMMA);

    assert_eq!(*result, rows(&[&["x", "a\n\n\nb"], &["y", "z"]]));
}

#[test]
fn parse_text_reads_an_escaped_quote_before_a_newline_inside_a_field() {
    let result = parse_text("\"say \"\"hi\"\"\nbye\",x", COMMA);

    assert_eq!(*result, rows(&[&["say \"hi\"\nbye", "x"]]));
}

#[test]
fn parse_text_falls_back_to_lines_when_a_quote_never_closes() {
    let result = parse_text("\"a,b\nc,d\ne,f", COMMA);

    assert_eq!(*result, rows(&[&["a,b"], &["c", "d"], &["e", "f"]]));
}

#[test]
fn parse_text_splits_csv_like_lines_does_for_plain_text() {
    let text = "a,b\n\nc,d\r\ne,f\n";

    let result = parse_text(text, COMMA);

    assert_eq!(*result, rows(&[&["a", "b"], &[""], &["c", "d"], &["e", "f"]]));
    assert_eq!(parse_text("", COMMA).len(), 0);
}

#[test]
fn csv_with_multi_line_fields_written_by_conv_data_reads_back_the_same() {
    let data = rows(&[&["line one\nline two", "a,b"], &["x\r\ny", "say \"hi\""]]);
    let text = conv_data(&data, "csv");

    assert_eq!(*parse_text(&text, COMMA), data);
}

#[test]
fn parse_text_keeps_csv_row_order_across_thread_chunks() {
    let total = 450_000;
    let text: Vec<String> = (0..total).map(|n| format!("{},\"x\ny\"", n)).collect();

    let result = parse_text(&text.join("\n"), COMMA);

    assert_eq!(result.len(), total);
    assert!(result.iter().enumerate().all(|(n, row)| row[0] == n.to_string() && row[1] == "x\ny"));
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
fn conv_data_escapes_quotes_inside_csv_values() {
    let text = conv_data(&rows(&[&["say \"hi\""]]), "csv");

    assert_eq!(*text, "\"say \"\"hi\"\"\"");
}

#[test]
fn csv_written_by_conv_data_reads_back_the_same() {
    let data = rows(&[&["say \"hi\"", "a,b", ""], &["1", "", "x"]]);
    let text = conv_data(&data, "csv");
    let back = parse_text(&text, COMMA);

    assert_eq!(*back, data);
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

#[test]
fn file_array_reports_a_name_without_an_extension() {
    let error = file_array("data.").unwrap_err();

    assert!(error.starts_with("Cannot find a file extension"), "{}", error);
}
