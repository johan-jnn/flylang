use std::{
    collections::BTreeSet,
    fs::{exists, read_to_string},
    path::{Path, PathBuf},
};
use toml::{Table, Value};

use crate::{
    behavior::errors::{InvalidKeyValue, InvalidPath, PathNotFound},
    flylang::errors::{ErrorType, LangResult, RaisableErr, lang_err},
    utils::{env::get_env_hashmap, str::ReplaceByKey},
};
pub mod errors;

/// This package is used to retreive the language's behavior from .toml files
#[derive(Debug, Clone, Default)]
pub struct LangBehavior {
    map: Table,
    processed: BTreeSet<PathBuf>,
}

impl LangBehavior {
    const EXTEND_KEY: &str = "extends";

    pub fn new_parsed(base_file: &Path) -> Self {
        let mut behavior = Self::default();
        behavior.parse(base_file.into(), None);
        behavior
    }

    fn get_parsed_value(value: &Value, deep_parse: bool) -> Value {
        match value {
            Value::String(s) => Value::String(ReplaceByKey::replace(s, get_env_hashmap())),
            Value::Array(arr) => Value::Array(
                arr.iter()
                    .map(|v| Self::get_parsed_value(v, deep_parse))
                    .collect(),
            ),
            Value::Table(table) => {
                if deep_parse {
                    Value::Table(
                        table
                            .iter()
                            .map(|(k, v)| (k.clone(), Self::get_parsed_value(v, deep_parse)))
                            .collect(),
                    )
                } else {
                    Value::Table(table.clone())
                }
            }
            _ => value.clone(),
        }
    }

    fn handle_extend(&mut self, extend: &Value, file: &PathBuf) -> LangResult<&mut Self> {
        match Self::get_parsed_value(extend, false) {
            Value::String(f) => {
                self.parse(Path::new(&f).into(), Some(file.clone()));

                Ok(self)
            }
            Value::Array(files) => {
                for f in files {
                    self.handle_extend(&f, file)?;
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

    fn merge(table: &Table, into: &mut Table, same_does_replace: bool) {
        for (key, raw_value) in table {
            let value = Self::get_parsed_value(raw_value, false);

            if !into.contains_key(key) {
                into.insert(key.clone(), Self::get_parsed_value(raw_value, true));
                continue;
            }

            if let Value::Table(table) = &value
                && let Value::Table(into) = &mut into[key]
            {
                Self::merge(table, into, same_does_replace);
            } else if let Value::Array(brothers) = &value
                && let Value::Array(family) = &mut into[key]
            {
                family.extend(brothers.clone());
            } else if same_does_replace {
                into.insert(key.clone(), Self::get_parsed_value(raw_value, true));
            }
        }
    }

    pub fn parse(&mut self, base_file: PathBuf, from: Option<PathBuf>) -> &mut Self {
        if self.processed.contains(&base_file) {
            return self;
        }

        let path_exist = exists(&base_file);
        if path_exist.is_err() || path_exist.is_ok_and(|v| !v) {
            PathNotFound { path: base_file }.raise()
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

        Self::merge(&data, &mut self.map, true);
        self.processed.insert(base_file);

        self
    }

    // Getters

    /// Get the value of an option defined by its key.
    /// Note that the key will be splitted by the `.` character
    /// You can use `.get("*")` to retrive the whole behaviors table
    pub fn get(&self, accessor: &str) -> Option<Value> {
        if accessor.is_empty() {
            return None;
        }

        let mut result: Option<Value> = Some(Value::Table(self.map.clone()));
        if accessor.trim() == "*" {
            return result;
        }

        let keys = accessor.split('.');
        for key in keys {
            let Some(Value::Table(table)) = result else {
                return None;
            };

            if !table.contains_key(key) {
                return None;
            }

            result = Some(table[key].clone());
        }

        result
    }
}
