use crate::flylang::{
    lexer::tokens::{ScopeTarget, Token},
    module::slice::LangModuleSlice,
    parser::{
        ast::{Node, expressions::Expressions},
        parsable::Parsable,
    },
};

#[derive(Debug, Clone)]
pub enum BreakKind {
    Stop(Option<Token<ScopeTarget>>),
    Pass(Option<Token<ScopeTarget>>),
    Return(Option<Token<ScopeTarget>>, Option<Node<Expressions>>),
}

#[derive(Debug, Clone)]
pub struct Break {
    pub kind: BreakKind,
    pub keyword_location: LangModuleSlice,
}

impl Parsable for Break {
    type ResultKind = Self;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<Node>,
    ) -> crate::flylang::errors::LangResult<Node<Self::ResultKind>> {
        todo!()
    }
}
