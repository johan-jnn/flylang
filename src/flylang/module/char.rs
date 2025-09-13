use std::rc::Rc;

use crate::flylang::module::LangModule;

/// One character of the module
/// This is the same than a `LangModuleSlice`, but it has only one character.
/// It's like a `LangModuleSlice` with a range : index..index+1
#[derive(Debug, Clone)]
pub struct LangModuleChar {
    module: Rc<LangModule>,
    index: usize,
}
impl LangModuleChar {
    pub fn new(module: &Rc<LangModule>) -> Self {
        Self {
            module: Rc::clone(module),
            index: 0,
        }
    }
    pub fn set(&mut self, index: usize) -> &mut Self {
        assert!(index < self.module.code.len());
        self.index = index;

        self
    }
    /// Get the char's index in the module's code
    pub fn index(&self) -> usize {
        self.index
    }
    /// Get the slice's module
    pub fn module(&self) -> &Rc<LangModule> {
        &self.module
    }
    pub fn code(&self) -> char {
        self.module
            .code
            .chars()
            .nth(self.index)
            .expect("Invalid index provided.")
    }

    /// Get the line index (starting at 0) of the character on its module
    pub fn line(&self) -> usize {
        self.module.code[..self.index]
            .lines()
            .count()
            .saturating_sub(1)
    }
    /// Get the location of the character on the character's line
    pub fn line_index(&self) -> usize {
        let mut shift = 0usize;
        let mut last_is_carriage = false;

        while shift < self.index {
            // The index cannot be greater than the module length (shift <= range.start <= module.len).
            let character = self.module.code.chars().nth(self.index - shift).unwrap();
            if character == '\n' {
                // We do not count the ending line character (and the carriage return if any).
                shift = shift.saturating_sub(1 + last_is_carriage as usize);
                break;
            }
            last_is_carriage = character == '\r';

            shift += 1;
        }
        shift
    }
}
