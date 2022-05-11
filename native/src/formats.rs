use std::collections::HashMap;

pub struct FileFormats {
    map: HashMap<&'static str, &'static str>
}

impl FileFormats {
    pub fn new() -> Self {
        Self {
            map: HashMap::from([
                ("csv", ","),
                ("tsv", "\t"),
                ("psv", "|"),
            ])
        }
    }

    pub fn get_map (&self) -> Box<HashMap<&'static str, &'static str>> {
        Box::new(self.map.clone())
    }
}
