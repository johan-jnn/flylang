use crate::flylang::{
    lexer::tokens::{Keywords, ScopeTarget, Toggleable, Token, Tokens},
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
        assert!(
            parser.analyser.min_len(1)
                && matches!(
                    parser.analyser.get()[0].kind(),
                    Tokens::Keyword(Keywords::Return | Keywords::Pass | Keywords::Stop)
                )
        );

        let token = parser.analyser.get()[0].clone();
        let Tokens::Keyword(keyword) = token.kind() else {
            panic!("Above verifications are wrong.")
        };

        let scope = if let Some(slice) = parser.analyser.lookup(0, 1) {
            let token = slice[0].clone();

            match token.kind() {
                Tokens::ScopeTarget(target) => {
                    parser.analyser.next(0, 1);
                    Some(Token::new(target.clone(), token.location()))
                }
                _ => None,
            }
        } else {
            None
        };

        let location =
            LangModuleSlice::from(&vec![token.location().clone(), parser.analyser_slice()]);

        Ok(Node::new(
            Self {
                keyword_location: token.location().clone(),
                kind: match keyword {
                    Keywords::Pass => BreakKind::Pass(scope),
                    Keywords::Stop => BreakKind::Stop(scope),
                    Keywords::Return => {
                        let returned_value = if let Some(slice) = parser.analyser.lookup(0, 1) {
                            let token = slice[0].clone();
                            match token.kind() {
                                Tokens::ArgSeparator
                                | Tokens::Block(Toggleable::Closing)
                                | Tokens::EndOfInstruction => None,
                                _ => {
                                    parser.analyser.next(0, 0);
                                    Some(Expressions::parse(parser, previous)?)
                                }
                            }
                        } else {
                            None
                        };

                        BreakKind::Return(scope, returned_value)
                    }
                    _ => panic!("Above verifications are wrong."),
                },
            },
            &location,
        ))
    }
}
