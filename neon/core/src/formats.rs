use std::collections::HashMap;

pub const TAB: &str = "\t";
pub const PIPE: &str = "|";
pub const COMMA: &str = ",";

pub struct FileFormats {
    map: HashMap<&'static str, &'static str>,
}

impl FileFormats {
    pub fn new() -> Self {
        Self {
            map: HashMap::from([
                ("tsv", TAB),
                ("psv", PIPE),
                ("csv", COMMA),
            ])

        }
    }

    pub fn get_map (&self) -> &HashMap<&'static str, &'static str> {
        &self.map
    }

    #[allow(dead_code)]
    pub fn get_delim<'a>(&self, mode: &str) -> &str {
        let map = self.get_map();
        map.get(mode).unwrap_or(&COMMA)
    }
}
