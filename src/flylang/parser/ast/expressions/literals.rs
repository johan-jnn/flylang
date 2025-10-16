use enum_variant_type::EnumVariantType;

use crate::flylang::{
    errors::{RaisableErr, lang_err},
    lexer::tokens::{Literals, StringItem, Token, Tokens},
    parser::{
        Parser,
        ast::{BoxedNode, Node, expressions::Expressions, instructions::Instructions},
        errors::{UnableToParse, UnexpectedNode, UnexpectedToken},
        parsable::Parsable,
    },
};

#[derive(Debug, Clone)]
pub enum ParsedStringItem {
    Literal(std::string::String),
    Expression(BoxedNode<Expressions>),
}

#[derive(Debug, Clone, EnumVariantType)]
pub enum ParsedLiterals {
    #[evt(derive(Debug, Clone))]
    Word,
    #[evt(derive(Debug, Clone))]
    True,
    #[evt(derive(Debug, Clone))]
    False,
    #[evt(derive(Debug, Clone))]
    Number,
    #[evt(derive(Debug, Clone))]
    Empty,
    #[evt(derive(Debug, Clone))]
    String(Vec<Node<ParsedStringItem>>),
}

impl TryInto<Node<ParsedStringItem>> for Token<StringItem> {
    type Error = Box<dyn RaisableErr>;

    fn try_into(self) -> Result<Node<ParsedStringItem>, Self::Error> {
        Ok(Node::new(
            match self.kind() {
                StringItem::Literal(content) => ParsedStringItem::Literal(content.clone()),
                StringItem::Expression(expr) => {
                    let mut parser = Parser::new(self.location().module(), expr.as_ref().clone());
                    let parsed = parser.parse();

                    if parsed.len() != 1 {
                        return lang_err!(UnableToParse(
                            self.location().clone(),
                            "Expected a single expression".to_string()
                        ));
                    };
                    let instruction = parsed[0].clone();

                    let Instructions::ValueOf(expression) = instruction.kind() else {
                        return lang_err!(UnexpectedNode(instruction));
                    };

                    ParsedStringItem::Expression(Box::new(Node::new(
                        expression.clone(),
                        instruction.location(),
                    )))
                }
            },
            self.location(),
        ))
    }
}

impl Parsable for ParsedLiterals {
    type ResultKind = Self;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<Node>,
    ) -> crate::flylang::errors::LangResult<Node<Self::ResultKind>> {
        assert!(
            parser.analyser.min_len(1)
                && matches!(parser.analyser.get()[0].kind(), Tokens::Literal(_))
        );

        let token = parser.analyser.get()[0].clone();
        if previous.is_some() {
            return lang_err!(UnexpectedToken(token));
        }

        let Tokens::Literal(literal) = token.kind() else {
            panic!("Invalid above verifications")
        };

        let literal: Self::ResultKind = match literal {
            Literals::Empty => Self::Empty,
            Literals::False => Self::False,
            Literals::True => Self::True,
            Literals::Number => Self::Number,
            Literals::Word => Self::Word,

            Literals::String(parts) => {
                let mut parsed_parts = vec![];
                for part in parts {
                    parsed_parts.push(part.clone().try_into()?);
                }

                Self::String(parsed_parts)
            }
        };

        Ok(Node::new(literal, token.location()))
    }
}
