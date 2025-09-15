use crate::flylang::{
    lexer::tokens::{Keywords, Tokens},
    parser::{
        ast::{
            Node,
            expressions::Expressions,
            instructions::conditionnal::{If, IfResult},
        },
        mods::ParserBehaviors,
        parsable::Parsable,
    },
};

pub mod conditionnal;

#[derive(Debug, Clone)]
pub enum Instructions {
    ValueOf(Expressions),
    If(If),
}
impl Parsable for Instructions {
    type ResultKind = Self;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<super::Node>,
    ) -> crate::flylang::errors::LangResult<super::Node<Self::ResultKind>> {
        parser.analyser.min_len(1);
        assert_eq!(parser.analyser.range().len(), 1);

        let token = parser.analyser.get()[0].clone();

        let instruction = match token.kind() {
            Tokens::EndOfInstruction => previous
                .expect("Tried to parse an 'EndOfInstruction' token with no previous value."),
            Tokens::Keyword(Keywords::If) => {
                let result = If::parse(parser, previous)?;

                match result.kind() {
                    IfResult::If(condition) => {
                        Node::new(Self::If(condition.clone()), result.location())
                    }
                    IfResult::Ternary(ternary) => Node::new(
                        Self::ValueOf(Expressions::Ternary(ternary.clone())),
                        result.location(),
                    ),
                }
            }
            _ => {
                parser.behaviors.remove(&ParserBehaviors::Lazy);

                let expr = Expressions::parse(parser, previous)?;
                Node::new(Self::ValueOf(expr.kind().clone()), expr.location())
            }
        };

        Ok(instruction)
    }
}
