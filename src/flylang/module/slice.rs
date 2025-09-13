use std::{fmt::Display, ops::Range, rc::Rc};

use crate::flylang::module::{LangModule, char::LangModuleChar};

/// A module's slice.
/// The range must follow those rules :
///     - start <= end
///     - end <= length of module
///
/// Additional informations :
///     - if start = end, the `code` method returns an empty &str
///     - if start = end = length of module, the "cursor" is at the end of the module's code
#[derive(Debug, Clone)]
pub struct LangModuleSlice {
    module: Rc<LangModule>,
    range: Range<usize>,
}
impl LangModuleSlice {
    pub fn new(module: &Rc<LangModule>) -> Self {
        Self {
            module: Rc::clone(module),
            range: 0..0,
        }
    }
    /// Override the current range.
    /// Panics if the range is not valid.
    pub fn set(&mut self, range: Range<usize>) -> &mut Self {
        assert!(
            range.start <= range.end,
            "Invalid range ({:?}). The start value must be <= to the end value.",
            range
        );
        assert!(
            range.end <= self.module.code.len(),
            "Invalid range ({:?}). Out of range (max: {}).",
            range,
            self.module.code.len()
        );

        self.range = range;
        self
    }
    /// Get the current range of the slice.
    /// To prevent invalid modifications of the range, the returned slice is cloned.
    /// To override the range, use the `set` method.
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }
    /// Get the slice's module
    pub fn module(&self) -> &Rc<LangModule> {
        &self.module
    }

    /// Get the content of the slice
    pub fn code(&self) -> &str {
        &self.module.code()[self.range()]
    }

    /// Get the `LangModuleChar` object of the first character, or None if the slice is empty
    pub fn start(&self) -> Option<LangModuleChar> {
        if !self.code().is_empty() {
            let mut modchar = LangModuleChar::new(&self.module);
            modchar.set(self.range.start);
            Some(modchar)
        } else {
            None
        }
    }
    /// Get the `LangModuleChar` object of the last character, or None if the slice is empty
    pub fn end(&self) -> Option<LangModuleChar> {
        if !self.code().is_empty() {
            let mut modchar = LangModuleChar::new(&self.module);
            modchar.set(self.range.end.saturating_sub(1));
            Some(modchar)
        } else {
            None
        }
    }
}

impl From<&Vec<LangModuleChar>> for LangModuleSlice {
    fn from(value: &Vec<LangModuleChar>) -> Self {
        assert!(
            !value.is_empty(),
            "Cannot create a module slice from an empty array of module chars"
        );

        let mut start = value[0].index();
        let mut end = value[0].index();
        let module = value[0].module();

        for modchar in value.iter().skip(1) {
            assert!(
                module == modchar.module(),
                "Different LangModule in the given array."
            );

            start = start.min(modchar.index());
            end = end.max(modchar.index());
        }

        let mut modslice = LangModuleSlice::new(module);
        modslice.set(start..end + 1);
        modslice
    }
}

impl From<&Vec<LangModuleSlice>> for LangModuleSlice {
    fn from(value: &Vec<LangModuleSlice>) -> Self {
        assert!(
            !value.is_empty(),
            "Cannot create a module slice from an empty array of module chars"
        );

        let mut start = value[0].range().start;
        let mut end = value[0].range().end;
        let module = value[0].module();

        for modchar in value.iter().skip(1) {
            assert!(
                module == modchar.module(),
                "Different LangModule in the given array."
            );

            start = start.min(modchar.range().start);
            end = end.max(modchar.range().end);
        }

        let mut modslice = LangModuleSlice::new(module);
        modslice.set(start..end);
        modslice
    }
}

impl From<&LangModuleChar> for LangModuleSlice {
    fn from(value: &LangModuleChar) -> Self {
        let mut modslice = LangModuleSlice::new(value.module());
        modslice.set(value.index()..(value.index() + 1));
        modslice
    }
}

impl Display for LangModuleSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "At: ")?;
        }

        if let Some(start) = self.start() {
            write!(
                f,
                "{}:{}:{}",
                self.module,
                start.line() + 1,
                start.line_index() + 1
            )
        } else {
            write!(
                f,
                "{}[{}; {}[ (empty)",
                self.module, self.range.start, self.range.end
            )
        }
    }
}
