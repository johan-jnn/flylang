use crate::flylang::{
    errors::lang_err,
    lexer::tokens::{Keywords, Tokens},
    parser::{
        ast::definables::{
            class::DefineClass, functions::DefineFunction, variables::DefineVariable,
        },
        errors::UnexpectedToken,
        parsable::Parsable,
    },
};

pub mod class;
pub mod functions;
pub mod variables;

#[derive(Debug, Clone)]
pub enum Definables {
    Function(DefineFunction),
    Variable(DefineVariable),
    Class(DefineClass),
}

impl Parsable for Definables {
    type ResultKind = Self;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<super::Node>,
    ) -> crate::flylang::errors::LangResult<super::Node<Self::ResultKind>> {
        parser.analyser.min_len(1);
        assert_eq!(parser.analyser.range().len(), 1);

        let token = &parser.analyser.get()[0];
        match token.kind() {
            Tokens::VarDef(_) => {
                let node = DefineVariable::parse(parser, previous)?;
                Ok(node.clone_as(|k, l| (Self::Variable(k), l)))
            }
            Tokens::Keyword(Keywords::Cs) => {
                let node = DefineClass::parse(parser, previous)?;
                Ok(node.clone_as(|k, l| (Self::Class(k), l)))
            }
            Tokens::Keyword(Keywords::Fn) => {
                let node = DefineFunction::parse(parser, previous)?;
                Ok(node.clone_as(|k, l| (Self::Function(k), l)))
            }
            _ => lang_err!(UnexpectedToken(token.clone())),
        }
    }
}
