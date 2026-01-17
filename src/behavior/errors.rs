use std::path::{self, PathBuf};
use toml::Value;

use crate::{
    flylang::errors::{ErrorType, RaisableErr},
    utils::macros::abs_path::absolute_path,
};

#[derive(Debug, Clone)]
pub struct InvalidPath {
    pub from_file: Option<PathBuf>,
    pub invalid_file: PathBuf,
}

impl RaisableErr for InvalidPath {
    fn _kind(&self) -> ErrorType {
        ErrorType::Stop
    }
    fn _message(&self) -> String {
        if let Some(from) = &self.from_file {
            format!(
                "Behavior file at <{:?}> cannot be extended by <{:?}>. The format, file or path is not valid.",
                from,
                absolute_path!(self.invalid_file)
            )
        } else {
            format!(
                "Invalid behavior path <{:?}>. The format, file or path is not valid.",
                absolute_path!(self.invalid_file)
            )
        }
    }
}

pub struct InvalidKeyValue {
    pub from_file: PathBuf,
    pub key: String,
    pub value_found: Option<Value>,
    pub expected: Option<Vec<Value>>,

    pub kind: ErrorType,
}

impl RaisableErr for InvalidKeyValue {
    fn _kind(&self) -> ErrorType {
        self.kind.clone()
    }

    fn _message(&self) -> String {
        format!(
            "Value for key '{:?}' in <{:?}> is not valid.\nExpected: {:?}. Found: {:?}",
            self.key, self.from_file, self.expected, self.value_found
        )
    }
}
