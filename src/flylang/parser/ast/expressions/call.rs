use crate::flylang::{
    errors::lang_err,
    lexer::tokens::{Toggleable, Tokens},
    module::slice::LangModuleSlice,
    parser::{
        ast::{BoxedNode, BoxedNodes, Node, expressions::Expressions, instructions::Instructions},
        errors::{UnableToParse, UnexpectedNode, UnexpectedToken},
        parsable::Parsable,
    },
};

#[derive(Debug, Clone)]
pub struct Call {
    pub callable: BoxedNode<Expressions>,
    pub arguments: BoxedNodes<Expressions>,
}

impl Parsable for Call {
    type ResultKind = Self;

    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<crate::flylang::parser::ast::Node>,
        _lazy: bool,
    ) -> crate::flylang::errors::LangResult<crate::flylang::parser::ast::Node<Self::ResultKind>>
    {
        assert!(
            parser.analyser.min_len(1)
                && matches!(
                    parser.analyser.get()[0].kind(),
                    Tokens::Block(Toggleable::Openning)
                )
        );
        let openner = parser.analyser.get()[0].clone();

        let Some(callable_node) = previous else {
            return lang_err!(UnexpectedToken(openner));
        };
        let Instructions::ValueOf(callable_expr) = callable_node.kind() else {
            return lang_err!(UnexpectedNode(callable_node));
        };

        // Parsing arguments
        parser.analyser.next(0, 0);
        let branches = parser.branches(
            |_, token| matches!(token.kind(), Tokens::Block(Toggleable::Closing)),
            |_, token| matches!(token.kind(), Tokens::ArgSeparator),
        )?;
        let mut arguments = vec![];

        // If there is content (not: "()")
        if !(branches.len() == 1 && branches[0].is_empty()) {
            for nodes in branches {
                if nodes.len() != 1 {
                    let arguments_location = LangModuleSlice::from(&vec![
                        openner.location().clone(),
                        parser.analyser_slice(),
                    ]);

                    return lang_err!(UnableToParse(
                        arguments_location,
                        String::from("One of the argument is not a valid expression")
                    ));
                }
                let node = &nodes[0];

                let Instructions::ValueOf(argument) = node.kind() else {
                    return lang_err!(UnexpectedNode(node.clone()));
                };

                arguments.push(Box::new(node.clone_as(|_, l| (argument.clone(), l))));
            }
        }

        let location = LangModuleSlice::from(&vec![
            callable_node.location().clone(),
            parser.analyser_slice(),
        ]);

        Ok(Node::new(
            Self {
                callable: callable_node
                    .clone_as(|_, l| (callable_expr.clone(), l))
                    .into(),
                arguments,
            },
            &location,
        ))
    }
}
