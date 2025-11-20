use std::{path::PathBuf, rc::Rc};

use crate::flylang::{lexer::Lexer, module::LangModule, parser::Parser};

pub mod errors;
pub mod lexer;
pub mod module;
pub mod parser;
pub mod utils;

pub struct FlyLang();

impl FlyLang {
    pub fn path(location: String) -> PathBuf {
        PathBuf::from(location)
    }
    pub fn module(path: PathBuf) -> LangModule {
        LangModule::new(path).unwrap_or_else(|e| e.raise())
    }
    pub fn lexer(path: PathBuf) -> Lexer {
        Lexer::new(&Rc::new(Self::module(path)))
    }
    pub fn parser(path: PathBuf) -> Parser {
        let mut lexer = Self::lexer(path);
        #[cfg(debug_assertions)]
        {
            dbg!(&lexer.lexify());
        }
        lexer.lexify();

        Parser::from(&mut lexer)
    }

    pub fn anonymous_module(script: &str, label: Option<&str>) -> LangModule {
        LangModule::new_from_raw(script.to_string(), label.unwrap_or("anonymous"))
    }
    pub fn anonymous_lexer(script: &str, label: Option<&str>) -> Lexer {
        Lexer::new(&Rc::new(Self::anonymous_module(script, label)))
    }
    pub fn anonymous_parser(script: &str, label: Option<&str>) -> Parser {
        let mut lexer = Self::anonymous_lexer(script, label);
        #[cfg(debug_assertions)]
        {
            dbg!(&lexer.lexify());
        }
        lexer.lexify();

        Parser::from(&mut lexer)
    }
}
