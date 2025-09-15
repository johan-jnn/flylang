use crate::flylang::{
    errors::lang_err,
    lexer::tokens::{self, Operator, Token, Tokens},
    module::slice::LangModuleSlice,
    parser::{
        ast::{BoxedNode, Node, expressions::Expressions, instructions::Instructions},
        errors::{UnexpectedNode, UnexpectedToken},
        mods::ParserBehaviors,
        parsable::Parsable,
    },
};

type Operand = BoxedNode<Expressions>;
type Operands = (Operand, Operand);
#[derive(Debug, Clone)]
pub struct Operation<Operator> {
    pub operator: Token<Operator>,
    pub operands: Operands,
}
type NumericOperations = Operation<tokens::Operator>;
type BinaryOperations = Operation<tokens::BinaryOperator>;
type ComparativeOperations = Operation<tokens::Comparison>;

// ------
#[derive(Debug, Clone)]
pub enum Operations {
    Numeric(NumericOperations),
    Binary(BinaryOperations),
    Comparative(ComparativeOperations),
}

impl Parsable for Operations {
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
                    Tokens::Operator(_) | Tokens::Comparison(_) | Tokens::BinaryOperator(_)
                )
        );

        let token = parser.analyser.get()[0].clone();
        let Some(left_node) = previous else {
            return lang_err!(UnexpectedToken(token));
        };

        if !parser.analyser.able_to(0, 1) {
            return lang_err!(UnexpectedToken(token));
        };
        parser.analyser.next(0, 1);
        parser.behaviors.insert(ParserBehaviors::Lazy);
        let mut right_operand = Expressions::parse(parser, None)?;

        let Instructions::ValueOf(left_expression) = left_node.kind() else {
            return lang_err!(UnexpectedNode(left_node));
        };

        // ? Operation ordering
        while let Some(slice) = parser.analyser.lookup(0, 1) {
            // If the next token is stronger that the current one, we parse the next one first
            let mut next_token = slice[0].kind();

            // Handle the case "!<", "!&", ...
            let skip_one = matches!(next_token, Tokens::Not);
            if skip_one {
                if let Some(slice) = parser.analyser.lookup(1, 1) {
                    next_token = slice[0].kind();
                } else {
                    return lang_err!(UnexpectedToken(slice[0].clone()));
                }
            }

            let use_next = match next_token {
                Tokens::Operator(operator) => {
                    !matches!(operator, Operator::Add | Operator::Substract)
                        && matches!(
                            token.kind(),
                            Tokens::Operator(Operator::Add | Operator::Substract)
                        )
                }
                Tokens::Comparison(_) => {
                    !matches!(token.kind(), Tokens::Operator(_) | Tokens::Comparison(_))
                }
                Tokens::BinaryOperator(op_next) => {
                    if let Tokens::BinaryOperator(op_now) = token.kind() {
                        (op_now.clone() as usize) < (op_next.clone() as usize)
                    } else {
                        false
                    }
                }
                _ => false,
            };

            if use_next {
                parser.analyser.next(skip_one as usize, 1);
                parser.behaviors.insert(ParserBehaviors::Lazy);

                right_operand = Expressions::parse(
                    parser,
                    Some(right_operand.clone_as(|k, l| (Instructions::ValueOf(k), l))),
                )?
            } else {
                break;
            }
        }

        let location = LangModuleSlice::from(&vec![
            left_node.location().clone(),
            right_operand.location().clone(),
        ]);

        let operands = (
            Node::new(left_expression.clone(), left_node.location()).into(),
            right_operand.into(),
        );

        Ok(Node::new(
            match token.kind() {
                Tokens::Operator(kind) => Self::Numeric(Operation {
                    operator: Token::new(kind.clone(), token.location()),
                    operands,
                }),
                Tokens::Comparison(kind) => Self::Comparative(Operation {
                    operator: Token::new(kind.clone(), token.location()),
                    operands,
                }),
                Tokens::BinaryOperator(kind) => Self::Binary(Operation {
                    operator: Token::new(kind.clone(), token.location()),
                    operands,
                }),
                _ => panic!(
                    "Weird operator {:?}. Above conditions invalid.",
                    token.kind()
                ),
            },
            &location,
        ))
    }
}
