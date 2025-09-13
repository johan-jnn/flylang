use crate::flylang::{
    errors::{ErrorType, RaisableErr},
    lexer::tokens::Token,
    module::slice::LangModuleSlice,
    parser::ast::Node,
};

pub struct UnexpectedNode(pub Node);
impl RaisableErr for UnexpectedNode {
    fn _code(&self) -> i32 {
        1
    }
    fn _kind(&self) -> ErrorType {
        ErrorType::Stop
    }
    fn _message(&self) -> String {
        format!("Unexpected {:?}.\n{:#}", self.0.kind(), self.0.location())
    }
}

pub struct UnexpectedToken(pub Token);
impl RaisableErr for UnexpectedToken {
    fn _code(&self) -> i32 {
        2
    }
    fn _kind(&self) -> ErrorType {
        ErrorType::Stop
    }
    fn _message(&self) -> String {
        format!(
            "Token \"{:?}\" was unexpected.\n{:#}",
            self.0.kind(),
            self.0.location()
        )
    }
}

pub struct Expected {
    pub after: LangModuleSlice,
    pub expected: Option<String>,
    pub but_found: Option<String>,
}
impl RaisableErr for Expected {
    fn _code(&self) -> i32 {
        3
    }
    fn _kind(&self) -> ErrorType {
        ErrorType::Stop
    }
    fn _message(&self) -> String {
        format!(
            "Expected {} after \"{}\" but found {}.\n{:#}",
            self.expected.clone().unwrap_or("expression".into()),
            self.after.code(),
            self.but_found.clone().unwrap_or("nothing".into()),
            self.after
        )
    }
}

pub struct EmptyScope(pub LangModuleSlice);
impl RaisableErr for EmptyScope {
    fn _code(&self) -> i32 {
        1
    }
    fn _kind(&self) -> ErrorType {
        ErrorType::Warn
    }
    fn _message(&self) -> String {
        format!(
            "Empty scope detected. Please remove it (in could break at future).\n{:#}",
            self.0
        )
    }
}

pub struct UnableToParse(pub LangModuleSlice, pub String);
impl RaisableErr for UnableToParse {
    fn _code(&self) -> i32 {
        4
    }
    fn _kind(&self) -> ErrorType {
        ErrorType::Stop
    }
    fn _message(&self) -> String {
        format!(
            "Unable to parse the expression \"{}\". {}.\n{:#}",
            self.0.code(),
            self.1,
            self.0
        )
    }
}
