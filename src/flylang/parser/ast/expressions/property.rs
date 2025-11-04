use crate::flylang::{
    errors::lang_err,
    lexer::tokens::{Literals, Toggleable, Tokens},
    module::slice::LangModuleSlice,
    parser::{
        ast::{BoxedNode, Node, expressions::Expressions, instructions::Instructions},
        errors::{Expected, UnexpectedNode, UnexpectedToken},
        mods::ParserBehaviors,
        parsable::Parsable,
    },
};

#[derive(Debug, Clone)]
pub enum Property {
    Key,
    Index,
    Expression(BoxedNode<Expressions>),
}

#[derive(Debug, Clone)]
pub struct ReadProperty {
    pub from: BoxedNode<Expressions>,
    pub read: Node<Property>,
}

impl Parsable for ReadProperty {
    type ResultKind = Self;

    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<Node>,
    ) -> crate::flylang::errors::LangResult<Node<Self::ResultKind>> {
        assert!(
            parser.analyser.range().len() == 1
                && matches!(parser.analyser.get()[0].kind(), Tokens::Accessor)
        );

        let Some(previous_instruction) = previous else {
            return lang_err!(UnexpectedToken(parser.analyser.get()[0].clone()));
        };
        let Instructions::ValueOf(from) = previous_instruction.kind() else {
            return lang_err!(UnexpectedNode(previous_instruction));
        };

        if !parser.analyser.able_to(0, 1) {
            return lang_err!(Expected {
                after: parser.analyser_slice(),
                expected: None,
                but_found: None
            });
        }
        parser.analyser.next(0, 1);
        let next = parser.analyser.get()[0].clone();

        let property = match next.kind() {
            Tokens::Literal(Literals::Number) => Property::Index,
            Tokens::Literal(Literals::Word | Literals::False | Literals::True) => Property::Key,
            Tokens::Block(Toggleable::Openning) => {
                parser.behaviors.insert(ParserBehaviors::Lazy);

                Property::Expression(Box::new(Expressions::parse(parser, None)?))
            }
            _ => return lang_err!(UnexpectedToken(next)),
        };
        let read = Node::new(
            property,
            &LangModuleSlice::from(&vec![next.location().clone(), parser.analyser_slice()]),
        );

        let location = &LangModuleSlice::from(&vec![
            previous_instruction.location().clone(),
            read.location().clone(),
        ]);

        Ok(Node::new(
            Self {
                from: Box::new(previous_instruction.clone_as(|_, l| (from.clone(), l))),
                read,
            },
            location,
        ))
    }
}
