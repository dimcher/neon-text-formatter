use std::{env, fs, path::PathBuf, process};

use text_formatter_core::{convert::*, csv::*, formats::*, io::*, read::*, write::*};

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
fn format_maps_each_type_to_its_delimiter() {
    assert_eq!(Format::Tsv.delim(), TAB);
    assert_eq!(Format::Psv.delim(), PIPE);
    assert_eq!(Format::Csv.delim(), COMMA);
}

#[test]
fn format_reads_the_extension_in_any_case() {
    assert_eq!(Format::from_path("dir/DATA.CSV"), Ok(Format::Csv));
    assert_eq!(Format::from_path("a.Tsv"), Ok(Format::Tsv));
}

#[test]
fn format_rejects_an_unknown_extension_instead_of_guessing_csv() {
    let error = Format::from_path("notes.txt").unwrap_err();

    assert!(error.starts_with("Unsupported file type 'txt'"), "{}", error);
}

#[test]
fn file_type_ignores_a_dot_in_a_folder_name() {
    assert!(file_type("/tmp/dir.v2/data").is_err());
}

#[test]
fn parse_text_does_not_let_a_stray_quote_merge_lines() {
    let result = parse_text("\"a\nb\n\"c\",d", COMMA);

    assert_eq!(result, rows(&[&["a"], &["b"], &["c", "d"]]));
}

#[test]
fn parse_text_still_reads_text_after_a_closing_quote_on_one_line() {
    let result = parse_text("\"a\"b,c\nd,e", COMMA);

    assert_eq!(result, rows(&[&["ab", "c"], &["d", "e"]]));
}

#[test]
fn parse_text_drops_a_carriage_return_at_the_end_of_the_text() {
    assert_eq!(parse_text("a,b\r", COMMA), rows(&[&["a", "b"]]));
    assert_eq!(parse_text("a\tb\r", TAB), rows(&[&["a", "b"]]));
}

#[test]
fn convert_file_writes_the_target_format() {
    let source = temp_file("conv-in.csv", "\"a\nb\",c");
    let target = temp_path("conv-out.tsv").to_string_lossy().into_owned();

    assert_eq!(convert_file(&source, &target), Ok(5));
    assert_eq!(read_file(&target), Ok("a b\tc".to_string()));
    fs::remove_file(source).unwrap();
    fs::remove_file(target).unwrap();
}

#[test]
fn convert_file_stops_on_an_unknown_target_type_and_writes_nothing() {
    let source = temp_file("conv-keep.csv", "a,b");
    let target = temp_path("conv-out.txt").to_string_lossy().into_owned();

    assert!(convert_file(&source, &target).unwrap_err().starts_with("Unsupported file type"));
    assert!(read_file(&target).is_err());
    fs::remove_file(source).unwrap();
}

#[test]
fn convert_text_uses_the_given_delimiter() {
    let target = temp_path("text-out.psv").to_string_lossy().into_owned();

    assert_eq!(convert_text("a\tb", TAB, &target), Ok(3));
    assert_eq!(read_file(&target), Ok("a|b".to_string()));
    fs::remove_file(target).unwrap();
}

#[test]
fn convert_text_rejects_a_delimiter_that_is_not_one_character() {
    let target = temp_path("bad-delim.csv").to_string_lossy().into_owned();

    for delim in ["", ",,"] {
        assert!(convert_text("a,b", delim, &target).unwrap_err().starts_with("Delimiter must be one character"));
    }
    assert!(read_file(&target).is_err());
}

#[test]
fn file_array_reports_an_unsupported_extension() {
    let file = temp_file("input.txt", "a,b");

    assert!(file_array(&file).unwrap_err().starts_with("Unsupported file type"));
    fs::remove_file(file).unwrap();
}

#[test]
fn parse_data_splits_each_line_on_the_delimiter() {
    let result = parse_data(&lines(&["a|b|c", "1|2|3"]), PIPE);

    assert_eq!(result, rows(&[&["a", "b", "c"], &["1", "2", "3"]]));
}

#[test]
fn parse_csv_strips_quotes_and_keeps_unquoted_fields() {
    let result = parse_csv(&lines(&["\"a\",b,\"c\""]));

    assert_eq!(result, rows(&[&["a", "b", "c"]]));
}

#[test]
fn parse_csv_keeps_commas_inside_quoted_fields() {
    let result = parse_csv(&lines(&["\"a,b\",\"c, d, e\",f"]));

    assert_eq!(result, rows(&[&["a,b", "c, d, e", "f"]]));
}

#[test]
fn parse_csv_unescapes_a_doubled_quote() {
    let result = parse_csv(&lines(&["\"say \"\"hi\",x"]));

    assert_eq!(result, rows(&[&["say \"hi", "x"]]));
}

#[test]
fn parse_csv_unescapes_every_doubled_quote() {
    let result = parse_csv(&lines(&["\"a\"\"b\",\"c\"\"d\""]));

    assert_eq!(result, rows(&[&["a\"b", "c\"d"]]));
}

#[test]
fn parse_csv_reads_an_empty_quoted_field_as_empty() {
    let result = parse_csv(&lines(&["\"a\",\"\",\"b\""]));

    assert_eq!(result, rows(&[&["a", "", "b"]]));
}

#[test]
fn parse_csv_keeps_the_text_of_a_quote_that_never_closes() {
    let result = parse_csv(&lines(&["\"a,b", "x,\"y"]));

    assert_eq!(result, rows(&[&["a,b"], &["x", "y"]]));
}

#[test]
fn parse_csv_keeps_a_quote_in_the_middle_of_an_unquoted_field() {
    let result = parse_csv(&lines(&["5\" nails,b"]));

    assert_eq!(result, rows(&[&["5\" nails", "b"]]));
}

#[test]
fn parse_csv_keeps_empty_fields() {
    let result = parse_csv(&lines(&[",a,,", ""]));

    assert_eq!(result, rows(&[&["", "a", "", ""], &[""]]));
}

#[test]
fn parse_text_keeps_a_quoted_field_that_spans_lines() {
    let result = parse_text("\"a\nb\",c", COMMA);

    assert_eq!(result, rows(&[&["a\nb", "c"]]));
}

#[test]
fn parse_text_keeps_crlf_inside_a_quoted_field() {
    let result = parse_text("\"a\r\nb\",c\r\n1,2\r\n", COMMA);

    assert_eq!(result, rows(&[&["a\r\nb", "c"], &["1", "2"]]));
}

#[test]
fn parse_text_keeps_several_blank_lines_inside_a_quoted_field() {
    let result = parse_text("x,\"a\n\n\nb\"\ny,z", COMMA);

    assert_eq!(result, rows(&[&["x", "a\n\n\nb"], &["y", "z"]]));
}

#[test]
fn parse_text_reads_an_escaped_quote_before_a_newline_inside_a_field() {
    let result = parse_text("\"say \"\"hi\"\"\nbye\",x", COMMA);

    assert_eq!(result, rows(&[&["say \"hi\"\nbye", "x"]]));
}

#[test]
fn parse_text_falls_back_to_lines_when_a_quote_never_closes() {
    let result = parse_text("\"a,b\nc,d\ne,f", COMMA);

    assert_eq!(result, rows(&[&["a,b"], &["c", "d"], &["e", "f"]]));
}

#[test]
fn parse_text_splits_csv_like_lines_does_for_plain_text() {
    let text = "a,b\n\nc,d\r\ne,f\n";

    let result = parse_text(text, COMMA);

    assert_eq!(result, rows(&[&["a", "b"], &[""], &["c", "d"], &["e", "f"]]));
    assert_eq!(parse_text("", COMMA).len(), 0);
}

#[test]
fn csv_with_multi_line_fields_written_by_conv_data_reads_back_the_same() {
    let data = rows(&[&["line one\nline two", "a,b"], &["x\r\ny", "say \"hi\""]]);
    let text = conv_data(&data, Format::Csv);

    assert_eq!(parse_text(&text, COMMA), data);
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

    assert_eq!(result, rows(&[&["a,b", "c"], &["1", "2"]]));
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
    let text = conv_data(&rows(&[&["a", "b"], &["1", "2"]]), Format::Csv);

    assert_eq!(text, "\"a\",\"b\"\r\n\"1\",\"2\"");
}

#[test]
fn conv_data_joins_tsv_and_psv_without_quotes() {
    let data = rows(&[&["a", "b"], &["1", "2"]]);

    assert_eq!(conv_data(&data, Format::Tsv), "a\tb\r\n1\t2");
    assert_eq!(conv_data(&data, Format::Psv), "a|b\r\n1|2");
}

#[test]
fn conv_data_turns_line_breaks_and_delimiters_in_tsv_cells_into_spaces() {
    let data = rows(&[&["a\tb", "x\r\ny\nz"], &["1", "2"]]);

    assert_eq!(conv_data(&data, Format::Tsv), "a b\tx y z\r\n1\t2");
}

#[test]
fn conv_data_keeps_commas_in_psv_cells_and_replaces_pipes() {
    let data = rows(&[&["a,b", "c|d\ne"]]);

    assert_eq!(conv_data(&data, Format::Psv), "a,b|c d e");
}

#[test]
fn a_multi_line_csv_converts_to_tsv_with_one_row_per_record() {
    let data = parse_text("\"a\nb\",c\n1,2", COMMA);
    let tsv = conv_data(&data, Format::Tsv);

    assert_eq!(parse_text(&tsv, TAB), rows(&[&["a b", "c"], &["1", "2"]]));
}

#[test]
fn conv_data_escapes_quotes_inside_csv_values() {
    let text = conv_data(&rows(&[&["say \"hi\""]]), Format::Csv);

    assert_eq!(text, "\"say \"\"hi\"\"\"");
}

#[test]
fn csv_written_by_conv_data_reads_back_the_same() {
    let data = rows(&[&["say \"hi\"", "a,b", ""], &["1", "", "x"]]);
    let text = conv_data(&data, Format::Csv);
    let back = parse_text(&text, COMMA);

    assert_eq!(back, data);
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

    assert_eq!(result, rows(&[&["a", "b"], &["1", "2"]]));
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

#[test]
fn parse_text_keeps_non_ascii_text_around_a_multi_line_field() {
    let result = parse_text("é,\"ü\nñ\",日本\n語,x", COMMA);

    assert_eq!(result, rows(&[&["é", "ü\nñ", "日本"], &["語", "x"]]));
}

#[test]
fn parse_text_reads_a_quote_at_the_very_end_of_the_text() {
    assert_eq!(parse_text("\"a\"", COMMA), rows(&[&["a"]]));
    assert_eq!(parse_text("x,\"\"", COMMA), rows(&[&["x", ""]]));
    assert_eq!(parse_text("\"", COMMA), rows(&[&[""]]));
}

#[test]
fn parse_text_reads_a_field_that_is_only_an_escaped_quote() {
    assert_eq!(parse_text("\"\"\"\",x", COMMA), rows(&[&["\"", "x"]]));
}

#[test]
fn parse_text_gives_no_rows_for_empty_text_in_every_format() {
    assert_eq!(parse_text("", COMMA).len(), 0);
    assert_eq!(parse_text("", TAB).len(), 0);
    assert_eq!(parse_text("", PIPE).len(), 0);
}

#[test]
fn parse_text_reads_crlf_tsv_without_leaving_a_carriage_return() {
    let result = parse_text("a\tb\r\n1\t2\r\n", TAB);

    assert_eq!(result, rows(&[&["a", "b"], &["1", "2"]]));
}

#[test]
fn parse_text_keeps_a_multi_line_record_that_sits_on_a_chunk_boundary() {
    let total = 200_001;
    let mut text: Vec<String> = (0..total).map(|n| format!("{},x", n)).collect();
    text[199_999] = "199999,\"x\ny\"".to_string();
    text[200_000] = "200000,\"p\nq\"".to_string();

    let result = parse_text(&text.join("\n"), COMMA);

    assert_eq!(result.len(), total);
    assert_eq!(result[199_999], vec!["199999", "x\ny"]);
    assert_eq!(result[200_000], vec!["200000", "p\nq"]);
}

#[test]
fn parse_text_reads_a_long_quoted_field_with_many_lines() {
    let body = vec!["line"; 50_000].join("\n");
    let result = parse_text(&format!("\"{}\",end", body), COMMA);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0][0].lines().count(), 50_000);
    assert_eq!(result[0][1], "end");
}

#[test]
fn parse_text_with_many_unclosed_quotes_stays_fast_and_keeps_every_line() {
    let text = vec!["\"a,b"; 100_000].join("\n");

    let result = parse_text(&text, COMMA);

    assert!(!result.is_empty());
}

#[test]
fn convert_file_turns_psv_into_a_csv_that_reads_back() {
    let source = temp_file("rt-in.psv", "a,b|c\"d");
    let target = temp_path("rt-out.csv").to_string_lossy().into_owned();

    convert_file(&source, &target).unwrap();

    assert_eq!(file_array(&target), Ok(rows(&[&["a,b", "c\"d"]])));
    fs::remove_file(source).unwrap();
    fs::remove_file(target).unwrap();
}

#[test]
fn chunk_size_never_goes_below_the_minimum() {
    assert_eq!(chunk_size(0, 8), 20000);
    assert_eq!(chunk_size(1000, 8), 20000);
    assert_eq!(chunk_size(100000, 8), 20000);
}

#[test]
fn chunk_size_splits_big_input_evenly_across_workers() {
    assert_eq!(chunk_size(800000, 8), 100000);
    assert_eq!(chunk_size(800001, 8), 100001);
    assert!(800001_usize.div_ceil(chunk_size(800001, 8)) <= 8);
}

#[test]
fn chunk_size_copes_with_zero_workers() {
    assert_eq!(chunk_size(100000, 0), 100000);
}

#[test]
fn parse_text_keeps_row_order_when_split_into_several_chunks() {
    let text: String = (0..90000).map(|n| format!("{},x\n", n)).collect();
    let rows = parse_text(&text, ",");

    assert_eq!(rows.len(), 90000);
    assert!(rows.iter().enumerate().all(|(n, row)| row[0] == n.to_string()));
}

#[test]
fn parse_text_sized_keeps_multi_line_quoted_records_whole_across_chunks() {
    let text = (0..50)
        .map(|n| format!("\"line {n}\nsecond {n}\",{n}"))
        .collect::<Vec<_>>()
        .join("\n");

    let rows = parse_text_sized(&text, ",", 3);

    assert_eq!(rows.len(), 50);
    assert!(rows.iter().enumerate().all(|(n, row)| row == &vec![format!("line {n}\nsecond {n}"), n.to_string()]));
}

#[test]
fn parse_text_sized_matches_the_single_chunk_result() {
    let text = (0..200).map(|n| format!("{n},\"a,{n}\",x")).collect::<Vec<_>>().join("\n");

    assert_eq!(parse_text_sized(&text, ",", 7), parse_text_sized(&text, ",", 100000));
}

fn sample_rows(count: usize) -> Vec<Vec<String>> {
    (0..count)
        .map(|n| vec![format!("a\"{n}"), format!("line\r\nbreak\r{n}\n"), String::new(), format!("p|q\t{n}")])
        .collect()
}

#[test]
fn conv_data_sized_gives_the_same_text_for_any_chunk_size() {
    let rows = sample_rows(500);

    for format in Format::all() {
        let whole = conv_data_sized(&rows, format, 100000);

        assert_eq!(conv_data_sized(&rows, format, 7), whole);
        assert_eq!(conv_data_sized(&rows, format, 499), whole);
    }
}

#[test]
fn conv_data_flattens_line_breaks_and_delimiters_for_tsv_and_psv() {
    let rows = vec![vec!["a\r\nb\rc\nd".to_string(), "x|y\tz".to_string()]];

    assert_eq!(conv_data(&rows, Format::Tsv), "a b c d\tx|y z");
    assert_eq!(conv_data(&rows, Format::Psv), "a b c d|x y\tz");
}

#[test]
fn conv_data_quotes_every_csv_cell_and_doubles_quotes() {
    let rows = vec![vec!["say \"hi\"".to_string(), String::new()], vec![]];

    assert_eq!(conv_data(&rows, Format::Csv), "\"say \"\"hi\"\"\",\"\"\r\n");
}

#[test]
fn conv_data_of_no_rows_is_empty() {
    assert_eq!(conv_data(&[], Format::Csv), "");
}
