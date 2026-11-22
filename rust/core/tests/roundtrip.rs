use text_formatter_core::{formats::*, read::*, write::*};

const ALPHABET: [&str; 9] = ["a", "b", "\"", ",", "\n", "\r", " ", "é", "|"];

struct Lcg(u64);

impl Lcg {
    fn next(&mut self, bound: usize) -> usize {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);

        ((self.0 >> 33) as usize) % bound
    }
}

fn random_cell(rng: &mut Lcg) -> String {
    let len = rng.next(6);

    (0..len).map(|_| ALPHABET[rng.next(ALPHABET.len())]).collect()
}

fn random_row(rng: &mut Lcg) -> Vec<String> {
    let width = 1 + rng.next(4);

    (0..width).map(|_| random_cell(rng)).collect()
}

fn random_rows(rng: &mut Lcg) -> Vec<Vec<String>> {
    let height = 1 + rng.next(5);

    (0..height).map(|_| random_row(rng)).collect()
}

fn with_visible_last_row(mut rows: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let last_is_blank = rows.last().map_or(false, |row| row.len() == 1 && row[0].is_empty());

    if last_is_blank {
        rows.push(vec!["x".to_string()]);
    }

    rows
}

fn random_text(rng: &mut Lcg) -> String {
    let len = rng.next(40);

    (0..len).map(|_| ALPHABET[rng.next(ALPHABET.len())]).collect()
}

#[test]
fn csv_written_by_conv_data_always_reads_back_the_same() {
    let mut rng = Lcg(7);

    for _ in 0..3000 {
        let rows = random_rows(&mut rng);
        let text = conv_data(&rows, Format::Csv);

        assert_eq!(parse_text(&text, COMMA), rows, "text: {:?}", text);
    }
}

#[test]
fn parsing_any_text_as_csv_never_panics_and_never_loses_every_row() {
    let mut rng = Lcg(11);

    for _ in 0..5000 {
        let text = random_text(&mut rng);
        let rows = parse_text(&text, COMMA);

        assert!(text.is_empty() || !rows.is_empty(), "text: {:?}", text);
    }
}

#[test]
fn text_without_quotes_gives_one_row_per_line_like_tsv_does() {
    let mut rng = Lcg(3);

    for _ in 0..2000 {
        let text = random_text(&mut rng).replace('"', "");
        let csv_rows = parse_text(&text, COMMA).len();
        let tsv_rows = parse_text(&text, TAB).len();

        assert_eq!(csv_rows, tsv_rows, "text: {:?}", text);
    }
}

#[test]
fn tsv_and_psv_output_never_has_more_cells_or_rows_than_the_input() {
    let mut rng = Lcg(5);

    for _ in 0..2000 {
        let rows = with_visible_last_row(random_rows(&mut rng));

        for format in [Format::Tsv, Format::Psv] {
            let text = conv_data(&rows, format);
            let back = parse_text(&text, format.delim());

            assert_eq!(back.len(), rows.len(), "text: {:?}", text);
            assert!(back.iter().zip(&rows).all(|(b, r)| b.len() == r.len()), "text: {:?}", text);
        }
    }
}
