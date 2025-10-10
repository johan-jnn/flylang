use crate::flylang::{
    errors::{LangResult, lang_err},
    lexer::tokens::{ScopeTarget, Tokens},
    parser::{ast::Node, errors::UnableToParse, parsable::Parsable},
};

impl Parsable for ScopeTarget {
    type ResultKind = Self;

    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        _: Option<Node>,
    ) -> crate::flylang::errors::LangResult<Node<Self::ResultKind>> {
        assert!(
            parser.analyser.min_len(1)
                && matches!(parser.analyser.get()[0].kind(), Tokens::ScopeTarget(_))
        );

        let analysing_token = parser.analyser.get()[0].clone();
        if let Tokens::ScopeTarget(target) = analysing_token.kind() {
            Ok(Node::new(target.clone(), analysing_token.location()))
        } else {
            panic!("Wrong above conditions.");
        }
    }
}

impl Node<ScopeTarget> {
    /// Check if the scope is valid for scope definition.
    pub fn expect_definition(&self) -> LangResult<&Self> {
        if !matches!(self.kind(), ScopeTarget::Named(_)) {
            return lang_err!(UnableToParse(
                self.location().clone(),
                String::from("You can only define a scope by a name")
            ));
        }

        Ok(self)
    }
}
