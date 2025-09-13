use crate::flylang::{
    lexer::tokens::Tokens,
    parser::{
        ast::{Node, expressions::Expressions},
        parsable::Parsable,
    },
};

#[derive(Debug, Clone)]
pub enum Instructions {
    ValueOf(Expressions),
}
impl Parsable for Instructions {
    type ResultKind = Self;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<super::Node>,
        _lazy: bool,
    ) -> crate::flylang::errors::LangResult<super::Node<Self::ResultKind>> {
        parser.analyser.min_len(1);
        assert_eq!(parser.analyser.range().len(), 1);

        let token = parser.analyser.get()[0].clone();

        let instruction = match token.kind() {
            Tokens::EndOfInstruction => previous
                .expect("Tried to parse an 'EndOfInstruction' token with no previous value."),
            _ => {
                let expr = Expressions::parse(parser, previous, false)?;
                Node::new(Self::ValueOf(expr.kind().clone()), expr.location())
            }
        };

        Ok(instruction)
    }
}
