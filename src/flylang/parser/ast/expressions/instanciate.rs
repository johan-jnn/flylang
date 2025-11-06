use crate::flylang::{
    errors::lang_err,
    lexer::tokens::{Keywords, Tokens},
    module::slice::LangModuleSlice,
    parser::{
        ast::{BoxedNode, BoxedNodes, Node, expressions::Expressions, instructions::Instructions},
        errors::{Expected, UnexpectedNode, UnexpectedToken},
        mods::ParserBehaviors,
        parsable::Parsable,
    },
};

#[derive(Debug, Clone)]
pub struct ClassInstanciation {
    pub class: BoxedNode<Expressions>,
    pub arguments: BoxedNodes<Expressions>,
}

impl Parsable for ClassInstanciation {
    type ResultKind = Self;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<crate::flylang::parser::ast::Node>,
    ) -> crate::flylang::errors::LangResult<crate::flylang::parser::ast::Node<Self::ResultKind>>
    {
        assert!(
            parser.analyser.min_len(1)
                && matches!(
                    parser.analyser.get()[0].kind(),
                    Tokens::Keyword(Keywords::New)
                )
        );

        let token = parser.analyser.get()[0].clone();

        if previous.is_some() {
            return lang_err!(UnexpectedToken(token));
        }
        if !parser.analyser.able_to(0, 1) {
            return lang_err!(Expected {
                after: token.location().clone(),
                expected: Some(String::from("<class-call>")),
                but_found: None
            });
        }
        parser.analyser.next(0, 1);

        parser.behaviors.insert(ParserBehaviors::Lazy);
        let parsed = Instructions::parse(parser, None)?;
        parser.behaviors.remove(&ParserBehaviors::Lazy);

        let Instructions::ValueOf(Expressions::ReturnOf(call_data)) = parsed.kind() else {
            return lang_err!(UnexpectedNode(parsed));
        };

        let location =
            LangModuleSlice::from(&vec![token.location().clone(), parsed.location().clone()]);

        Ok(Node::new(
            Self {
                class: call_data.callable.clone(),
                arguments: call_data.arguments.clone(),
            },
            &location,
        ))
    }
}
