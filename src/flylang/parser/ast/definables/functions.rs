use crate::flylang::{
    errors::lang_err,
    lexer::tokens::{Keywords, Literals, ScopeTarget, Toggleable, Tokens},
    module::slice::LangModuleSlice,
    parser::{
        ast::{
            BoxedBranches, Branches, Node,
            expressions::{
                Expressions,
                literals::{ParsedLiterals, Word},
            },
            instructions::{
                Instructions,
                breakers::{Break, BreakKind},
            },
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
    pub scope_target: Option<Node<ScopeTarget>>,
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
        if !matches!(
            openner.kind(),
            Tokens::Block(Toggleable::Openning) | Tokens::ScopeTarget(_)
        ) {
            return lang_err!(UnexpectedToken(openner));
        };
        let (scope_target, mut branches) = parser.scope(None, None, None)?;

        let mut execution = branches.pop().unwrap();
        if execution.len() == 1 {
            let instruction = execution[0].clone();
            if let Instructions::ValueOf(expression) = instruction.kind() {
                execution = vec![instruction.clone_as(|_, l| {
                    (
                        Instructions::Break(Break {
                            kind: BreakKind::Return(None, Some(Node::new(expression.clone(), &l))),
                            keyword_location: l.clone(),
                        }),
                        l.clone(),
                    )
                })]
            }
        }

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
                Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Word))
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
                scope_target: scope_target
                    .map(|target| Node::new(target.kind().clone(), target.location())),
            },
            &location,
        ))
    }
}
