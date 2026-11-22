pub(crate) const QUOTE: char = '"';
const CSV_DELIM: char = ',';

#[derive(PartialEq)]
enum CsvEvent {
    Open,
    Close,
    Escaped,
    Delim,
    Char,
}

struct CsvState {
    in_quotes: bool,
    at_start: bool,
}

impl CsvState {
    fn new() -> Self {
        CsvState { in_quotes: false, at_start: true }
    }

    fn next(&mut self, c: char, upcoming: Option<char>) -> CsvEvent {
        if self.in_quotes {
            return self.next_quoted(c, upcoming);
        }

        self.next_plain(c)
    }

    fn next_quoted(&mut self, c: char, upcoming: Option<char>) -> CsvEvent {
        if c != QUOTE {
            return CsvEvent::Char;
        }

        if upcoming == Some(QUOTE) {
            return CsvEvent::Escaped;
        }

        self.in_quotes = false;
        CsvEvent::Close
    }

    fn next_plain(&mut self, c: char) -> CsvEvent {
        if c == QUOTE && self.at_start {
            self.in_quotes = true;
            self.at_start = false;
            return CsvEvent::Open;
        }

        if c == CSV_DELIM {
            self.at_start = true;
            return CsvEvent::Delim;
        }

        self.at_start = false;
        CsvEvent::Char
    }
}

fn finish_field(field: &mut String, line: &str, run: usize, end: usize) -> String {
    if field.is_empty() {
        return line[run..end].to_string();
    }

    field.push_str(&line[run..end]);
    std::mem::take(field)
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let bytes = line.as_bytes();
    let mut fields: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut state = CsvState::new();
    let mut run = 0;
    let mut i = 0;

    while i < bytes.len() {
        let upcoming = bytes.get(i + 1).map(|b| *b as char);

        match state.next(bytes[i] as char, upcoming) {
            CsvEvent::Char => {}
            CsvEvent::Open | CsvEvent::Close => {
                field.push_str(&line[run..i]);
                run = i + 1;
            }
            CsvEvent::Escaped => {
                field.push_str(&line[run..i]);
                field.push(QUOTE);
                i += 1;
                run = i + 1;
            }
            CsvEvent::Delim => {
                fields.push(finish_field(&mut field, line, run, i));
                run = i + 1;
            }
        }

        i += 1;
    }

    fields.push(finish_field(&mut field, line, run, bytes.len()));
    fields
}

pub fn parse_csv<S: AsRef<str>>(lines: &[S]) -> Vec<Vec<String>> {
    lines.iter().map(|line| parse_csv_line(line.as_ref())).collect()
}

struct Record {
    start: usize,
    end: usize,
    next: usize,
}

fn ends_field(upcoming: Option<char>) -> bool {
    matches!(upcoming, None | Some(CSV_DELIM) | Some('\n') | Some('\r'))
}

fn scan_record(data: &str, start: usize) -> Option<Record> {
    let bytes = data.as_bytes();
    let mut state = CsvState::new();
    let mut i = start;

    while i < bytes.len() {
        let c = bytes[i] as char;
        let upcoming = bytes.get(i + 1).map(|b| *b as char);

        match state.next(c, upcoming) {
            CsvEvent::Escaped => i += 1,
            CsvEvent::Close if !ends_field(upcoming) => return None,
            _ if c == '\n' && !state.in_quotes => return Some(Record { start, end: i, next: i + 1 }),
            _ => {}
        }

        i += 1;
    }

    if state.in_quotes {
        return None;
    }

    Some(Record { start, end: data.len(), next: data.len() })
}

fn first_line_bounds(data: &str, start: usize) -> Record {
    match data[start..].find('\n') {
        Some(i) => Record { start, end: start + i, next: start + i + 1 },
        None => Record { start, end: data.len(), next: data.len() },
    }
}

fn next_record(data: &str, start: usize) -> Record {
    scan_record(data, start).unwrap_or_else(|| first_line_bounds(data, start))
}

pub(crate) fn trim_cr(text: &str) -> &str {
    text.strip_suffix('\r').unwrap_or(text)
}

fn record_text<'a>(data: &'a str, record: &Record) -> &'a str {
    let text = &data[record.start..record.end];
    let at_line_end = record.next > record.end || record.end == data.len();

    if at_line_end {
        return trim_cr(text);
    }

    text
}

pub fn csv_records(data: &str) -> Vec<&str> {
    let mut records: Vec<&str> = Vec::new();
    let mut start = 0;

    while start < data.len() {
        let record = next_record(data, start);

        records.push(record_text(data, &record));
        start = record.next;
    }

    records
}
