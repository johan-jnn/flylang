use crate::flylang::{
    errors::lang_err,
    lexer::tokens::{Keywords, Literals, Toggleable, Tokens, Word},
    module::slice::LangModuleSlice,
    parser::{
        ast::{
            BoxedBranches, Branches, Node, expressions::Expressions, instructions::Instructions,
        },
        errors::{Expected, UnableToParse, UnexpectedNode, UnexpectedToken},
        parsable::Parsable,
    },
};

#[derive(Debug, Clone)]
pub struct DefineFunction {
    pub name: Option<Node<Word>>,
    pub arguments: BoxedBranches<Word>,
    pub execution: Branches,
}

impl Parsable for DefineFunction {
    type ResultKind = Self;

    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<Node>,
    ) -> crate::flylang::errors::LangResult<Node<Self::ResultKind>> {
        assert!(
            parser.analyser.min_len(1)
                && matches!(
                    parser.analyser.get()[0].kind(),
                    Tokens::Keyword(Keywords::Fn)
                )
        );

        let token = parser.analyser.get()[0].clone();
        if previous.is_some() {
            return lang_err!(UnexpectedToken(token));
        }

        let mut name: Option<Node<Word>> = None;
        if let Some(slice) = parser.analyser.lookup(0, 1) {
            if matches!(slice[0].kind(), Tokens::Literal(Literals::Word)) {
                name = Some(Node::new(Word, slice[0].location()));
                parser.analyser.next(1, 0);
            }
        } else {
            return lang_err!(UnexpectedToken(token));
        };

        if !parser.analyser.able_to(0, 1) {
            return lang_err!(Expected {
                after: if let Some(node) = name {
                    node.location().clone()
                } else {
                    token.location().clone()
                },
                but_found: None,
                expected: Some(String::from("("))
            });
        };

        parser.analyser.next(0, 1);
        let openner = parser.analyser.get()[0].clone();
        if !matches!(openner.kind(), Tokens::Block(Toggleable::Openning)) {
            return lang_err!(UnexpectedToken(openner));
        };
        parser.analyser.next(0, 0);

        // Parse the arguments (last one is the execution)
        let mut branches = parser.branches(
            |_, t| matches!(t.kind(), Tokens::Block(Toggleable::Closing)),
            |_, t| matches!(t.kind(), Tokens::ArgSeparator),
            None,
        )?;

        let execution = branches.pop().unwrap();
        let mut arguments = vec![];
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
            if !matches!(
                node.kind(),
                Instructions::ValueOf(Expressions::Literal(Literals::Word))
            ) {
                return lang_err!(UnexpectedNode(node.clone()));
            }

            arguments.push(node.clone_as(|_, l| (Word, l)));
        }

        let location =
            LangModuleSlice::from(&vec![token.location().clone(), parser.analyser_slice()]);

        Ok(Node::new(
            Self {
                name,
                execution,
                arguments: Box::new(arguments),
            },
            &location,
        ))
    }
}
