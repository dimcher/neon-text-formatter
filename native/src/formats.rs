use std::collections::HashMap;

pub const EOL: &str = "\r\n";
pub const COMMA: &str = ",";
pub const QUOTE: char = '"';
pub const THSIZE: usize = 3;

pub struct FileFormats {
    map: HashMap<&'static str, &'static str>,
}

impl FileFormats {
    pub fn new() -> Self {
        Self {
            map: HashMap::from([
                ("csv", COMMA),
                ("tsv", "\t"),
                ("psv", "|"),
            ])

        }
    }

    pub fn get_map (&self) -> Box<HashMap<&'static str, &'static str>> {
        Box::new(self.map.clone())
    }
}
