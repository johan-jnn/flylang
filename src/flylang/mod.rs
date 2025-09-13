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

        Parser::from(&mut lexer)
    }
}
