use std::{
    collections::{BTreeSet, HashMap},
    fs::read_to_string,
    path::{Path, PathBuf},
};
use toml::{Table, Value};

use crate::{
    behavior::errors::{InvalidKeyValue, InvalidPath},
    flylang::errors::{ErrorType, LangResult, RaisableErr, lang_err},
};
pub mod errors;

/// This package is used to retreive the language's behavior from .toml files
#[derive(Debug, Clone, Default)]
pub struct LangBehavior {
    map: HashMap<String, Value>,
    processed: BTreeSet<PathBuf>,
}

impl LangBehavior {
    const EXTEND_KEY: &str = "extends";

    pub fn new_parsed(base_file: &Path) -> Self {
        let mut behavior = Self::default();
        behavior.parse(base_file.into(), None);
        behavior
    }

    fn handle_extend(&mut self, extend: &Value, file: &PathBuf) -> LangResult<&mut Self> {
        match extend {
            Value::String(f) => {
                self.parse(Path::new(f).into(), Some(file.clone()));

                Ok(self)
            }
            Value::Array(files) => {
                for f in files {
                    self.handle_extend(f, file)?;
                }

                Ok(self)
            }
            _ => lang_err!(InvalidKeyValue {
                from_file: file.clone(),
                key: Self::EXTEND_KEY.into(),
                value_found: Some(extend.clone()),
                expected: Some(vec![]),

                kind: ErrorType::Stop
            }),
        }
    }

    pub fn parse(&mut self, base_file: PathBuf, from: Option<PathBuf>) -> &mut Self {
        if self.processed.contains(&base_file) {
            return self;
        }

        let file_err = InvalidPath {
            from_file: from,
            invalid_file: base_file.clone(),
        };

        let Ok(content) = read_to_string(&base_file) else {
            file_err.raise()
        };

        let data = content
            .parse::<Table>()
            .unwrap_or_else(|_| file_err.raise());

        if let Some(extend) = data.get(Self::EXTEND_KEY)
            && let Err(e) = self.handle_extend(extend, &base_file)
        {
            e.controlled_raise();
        }

        self.map.extend(data);
        self.processed.insert(base_file);

        self
    }
}
