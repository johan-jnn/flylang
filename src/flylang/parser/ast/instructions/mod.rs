use crate::flylang::{
    lexer::tokens::{Keywords, Tokens},
    parser::{
        ast::{
            Node,
            expressions::Expressions,
            instructions::{
                breakers::Break,
                conditionnal::{If, IfResult},
                loops::Loop,
                r#use::Package,
            },
        },
        mods::ParserBehaviors,
        parsable::Parsable,
    },
};

pub mod breakers;
pub mod conditionnal;
pub mod loops;
pub mod r#use;

#[derive(Debug, Clone)]
pub enum Instructions {
    ValueOf(Expressions),
    If(If),
    Loop(Loop),
    Break(Break),
    Use(Package),
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
            Tokens::Keyword(Keywords::Use) => {
                let package = Package::parse(parser, previous)?;
                package.clone_as(|k, l| (Self::Use(k), l))
            }
            Tokens::Keyword(Keywords::Each | Keywords::Until | Keywords::While) => {
                let result = Loop::parse(parser, previous)?;
                result.clone_as(|k, l| (Self::Loop(k), l))
            }
            Tokens::Keyword(Keywords::Pass | Keywords::Return | Keywords::Stop) => {
                let result = Break::parse(parser, previous)?;
                result.clone_as(|k, l| (Self::Break(k), l))
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
