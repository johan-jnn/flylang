use std::{
    ffi::OsStr,
    fmt::{Debug, Display},
    fs::read_to_string,
    path::PathBuf,
    rc::Rc,
};

use crate::flylang::{
    errors::{LangResult, RaisableErr, lang_err},
    module::{
        char::LangModuleChar,
        errors::{InvalidEntryPoint, WeirdExtension},
        slice::LangModuleSlice,
    },
};

pub mod char;
pub mod errors;
pub mod slice;

#[derive(Clone, PartialEq)]
pub struct LangModule {
    path: PathBuf,
    code: String,
}

impl LangModule {
    pub fn new(path: PathBuf) -> LangResult<Self> {
        if !(path.is_file() && path.exists()) {
            return lang_err!(InvalidEntryPoint(path));
        }

        if path.extension() != Some(OsStr::new("fly")) {
            WeirdExtension(path.clone()).print();
        }

        Ok(Self {
            code: read_to_string(&path).expect("The file path is invalid."),
            path,
        })
    }
    pub fn into_rc(self) -> Rc<Self> {
        Rc::new(self)
    }
    /// Get the content of the module
    pub fn code(&self) -> &str {
        &self.code
    }
    /// Get the path of the module. To avoid override, the returned path is cloned.
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
    pub fn chars(&self) -> ModIter {
        ModIter::new(&Rc::new(self.clone()))
    }
}
impl Display for LangModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
impl Debug for LangModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

pub struct ModIter {
    module: Rc<LangModule>,
    cursor: usize,
    max: usize,
}
impl ModIter {
    pub fn new(module: &Rc<LangModule>) -> Self {
        Self {
            module: Rc::clone(module),
            cursor: 0,
            max: module.code.len(),
        }
    }
}
impl From<&LangModuleSlice> for ModIter {
    fn from(value: &LangModuleSlice) -> Self {
        Self {
            module: Rc::clone(value.module()),
            cursor: value.range().start,
            max: value.range().end,
        }
    }
}
impl Iterator for ModIter {
    type Item = LangModuleChar;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.max {
            let mut modchar = LangModuleChar::new(&self.module);
            modchar.set(self.cursor);
            self.cursor += 1;

            Some(modchar)
        } else {
            None
        }
    }
}
