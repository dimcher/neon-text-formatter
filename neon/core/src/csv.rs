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

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut state = CsvState::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match state.next(c, chars.peek().copied()) {
            CsvEvent::Open | CsvEvent::Close => {}
            CsvEvent::Escaped => {
                chars.next();
                field.push(QUOTE);
            }
            CsvEvent::Delim => fields.push(std::mem::take(&mut field)),
            CsvEvent::Char => field.push(c),
        }
    }

    fields.push(field);
    fields
}

pub fn parse_csv(lines: &[String]) -> Vec<Vec<String>> {
    lines.iter().map(|line| parse_csv_line(line)).collect()
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
    let mut state = CsvState::new();
    let mut chars = data[start..].char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        let upcoming = chars.peek().map(|(_, ch)| *ch);

        match state.next(c, upcoming) {
            CsvEvent::Escaped => {
                chars.next();
            }
            CsvEvent::Close if !ends_field(upcoming) => return None,
            _ if c == '\n' && !state.in_quotes => {
                return Some(Record { start, end: start + i, next: start + i + 1 });
            }
            _ => {}
        }
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

fn record_text(data: &str, record: &Record) -> String {
    let text = &data[record.start..record.end];
    let at_line_end = record.next > record.end || record.end == data.len();

    if at_line_end {
        return trim_cr(text).to_string();
    }

    text.to_string()
}

pub fn csv_records(data: &str) -> Vec<String> {
    let mut records: Vec<String> = Vec::new();
    let mut start = 0;

    while start < data.len() {
        let record = next_record(data, start);

        records.push(record_text(data, &record));
        start = record.next;
    }

    records
}
