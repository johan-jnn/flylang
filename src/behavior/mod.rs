use clap::builder::OsStr;
use std::{
    collections::{BTreeSet, HashMap},
    fs::read_to_string,
    path::Path,
};
use toml::{Table, Value};

/// This package is used to retreive the language's behavior from .toml files
#[derive(Debug, Clone, Default)]
pub struct LangBehavior {
    map: HashMap<String, Value>,
    processed: BTreeSet<OsStr>,
}

impl LangBehavior {
    pub fn new_parsed(base_file: &Path) -> Self {
        let mut behavior = Self::default();
        behavior.parse(base_file);
        behavior
    }

    fn handle_extend(&mut self, extend: &Value) -> &mut Self {
        match extend {
            Value::String(f) => {
                self.parse(Path::new(f));
            }
            Value::Array(files) => {
                for f in files {
                    self.handle_extend(f);
                }
            }
            _ => panic!("Invalid extend path(s)."),
        }
        self
    }

    pub fn parse(&mut self, base_file: &Path) -> &mut Self {
        if self.processed.contains(base_file.as_os_str()) {
            return self;
        }

        let Ok(content) = read_to_string(base_file) else {
            panic!("File at {:?} is does not exist.", base_file.as_os_str());
        };

        let data = content.parse::<Table>().unwrap_or_else(|_| {
            panic!(
                "File at {:?} is not readable in toml format.",
                base_file.as_os_str()
            )
        });

        if let Some(extend) = data.get("extends") {
            self.handle_extend(extend);
        }

        self.map.extend(data);
        self
    }
}
