use std::{ffi::OsStr, path::PathBuf};

use crate::flylang::errors::{ErrorType, RaisableErr};

pub struct InvalidEntryPoint(pub PathBuf);
impl RaisableErr for InvalidEntryPoint {
    fn _kind(&self) -> ErrorType {
        ErrorType::Stop
    }
    fn _message(&self) -> String {
        format!(
            "Entrypoint ({}) is not valid.\nMaybe the file does not exist ?",
            self.0.display()
        )
    }
}

pub struct WeirdExtension(pub PathBuf);
impl RaisableErr for WeirdExtension {
    fn _kind(&self) -> ErrorType {
        ErrorType::Warn
    }
    fn _message(&self) -> String {
        format!(
            "The module ({}) uses a non-flylang extension (.{}). Prefer using a (.fly) extension.",
            self.0.display(),
            self.0.extension().unwrap_or(OsStr::new("")).to_str().unwrap()
        )
    }
}
