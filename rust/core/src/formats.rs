use crate::io::file_type;

pub const TAB: &str = "\t";
pub const PIPE: &str = "|";
pub const COMMA: &str = ",";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format {
    Csv,
    Tsv,
    Psv,
}

impl Format {
    pub fn all() -> [Format; 3] {
        [Format::Csv, Format::Tsv, Format::Psv]
    }

    pub fn ext(self) -> &'static str {
        match self {
            Format::Csv => "csv",
            Format::Tsv => "tsv",
            Format::Psv => "psv",
        }
    }

    pub fn delim(self) -> &'static str {
        match self {
            Format::Csv => COMMA,
            Format::Tsv => TAB,
            Format::Psv => PIPE,
        }
    }

    pub fn from_ext(ext: &str) -> Result<Format, String> {
        Format::all()
            .iter()
            .copied()
            .find(|format| format.ext().eq_ignore_ascii_case(ext))
            .ok_or_else(|| format!("Unsupported file type '{}'. Use csv, tsv or psv", ext))
    }

    pub fn from_path(path: &str) -> Result<Format, String> {
        Format::from_ext(file_type(path)?)
    }
}
