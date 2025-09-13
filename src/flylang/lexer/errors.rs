use std::rc::Rc;

use crate::flylang::{
    errors::{ErrorType, RaisableErr},
    module::slice::LangModuleSlice,
    utils::scoper::Scope,
};

pub struct UnknownCharacter(pub Rc<LangModuleSlice>);
impl RaisableErr for UnknownCharacter {
    fn _code(&self) -> i32 {
        2
    }
    fn _kind(&self) -> ErrorType {
        ErrorType::Stop
    }
    fn _message(&self) -> String {
        format!(
            "Cannot understand character{} \"{}\"\n{:#}",
            if self.0.range().len() > 1 { "s" } else { "" },
            self.0.code(),
            self.0
        )
    }
}

pub struct UnexpectedCharacter(pub Rc<LangModuleSlice>, pub Option<&'static str>);
impl RaisableErr for UnexpectedCharacter {
    fn _code(&self) -> i32 {
        3
    }
    fn _kind(&self) -> ErrorType {
        ErrorType::Stop
    }
    fn _message(&self) -> String {
        format!(
            "Unexpected character \"{}\"{}.\n{:#}",
            self.0.code(),
            if let Some(expected) = self.1 {
                format!(" (Expected matching \"{}\")", expected)
            } else {
                "".into()
            },
            self.0
        )
    }
}

pub struct InvalidScopeEnding(pub Rc<LangModuleSlice>, pub Scope<LangModuleSlice>);
impl RaisableErr for InvalidScopeEnding {
    fn _code(&self) -> i32 {
        3
    }
    fn _kind(&self) -> ErrorType {
        ErrorType::Stop
    }
    fn _message(&self) -> String {
        format!(
            "Unclosed {}. Expected to close {} at {} but found \"{}\".\n{:#}",
            self.1,
            self.1,
            self.1.data(),
            self.0.code(),
            self.0
        )
    }
}

pub struct UnclosedScope(pub Scope<LangModuleSlice>);
impl RaisableErr for UnclosedScope {
    fn _code(&self) -> i32 {
        3
    }
    fn _kind(&self) -> ErrorType {
        ErrorType::Stop
    }
    fn _message(&self) -> String {
        format!("Unclosed {} (openned at {}).", self.0, self.0.data())
    }
}
