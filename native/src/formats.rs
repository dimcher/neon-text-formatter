use std::collections::HashMap;

pub const COMMA: &str = ",";

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

    pub fn get_map (&self) -> &HashMap<&'static str, &'static str> {
        &self.map
    }


    pub fn file_delim<'a>(&self, mode: &str) -> &str {
        let map = self.get_map();
        map.get(mode).unwrap_or(&COMMA)
    }
}
