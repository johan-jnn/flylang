use std::{collections::HashMap, path::Path};

/// This package is used to retreive the language's behavior from .toml files
struct LangBehavior {
    map: HashMap<String, String>,
}

impl LangBehavior {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn new_parsed(base_file: &Path) -> Self {
        let mut behavior = Self::new();
        behavior.parse(base_file);
        behavior
    }

    pub fn parse(&mut self, base_file: &Path) -> &mut Self {
        self
    }
}
