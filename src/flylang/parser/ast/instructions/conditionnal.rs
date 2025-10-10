use crate::flylang::{
    errors::lang_err,
    lexer::tokens::{Keywords, ScopeTarget, Toggleable, Token, Tokens},
    module::slice::LangModuleSlice,
    parser::{
        ast::{
            BoxedBranches, BoxedNode, Branches, Node,
            expressions::{Expressions, ternary::Ternary},
            instructions::Instructions,
        },
        errors::{Expected, UnableToParse, UnexpectedNode, UnexpectedToken},
        parsable::Parsable,
    },
};

#[derive(Debug, Clone)]
pub enum IfFallBack {
    Process(Option<Node<ScopeTarget>>, Branches),
    If(Node<If>),
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Node<Expressions>,
    pub process: BoxedBranches,
    pub fallback: Option<BoxedNode<IfFallBack>>,
    pub scope_target: Option<Node<ScopeTarget>>,
}

#[derive(Debug, Clone)]
pub enum IfResult {
    Ternary(Ternary),
    If(If),
}

impl Parsable for If {
    type ResultKind = IfResult;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<Node>,
    ) -> crate::flylang::errors::LangResult<Node<Self::ResultKind>> {
        assert!(
            parser.analyser.min_len(1)
                && matches!(
                    parser.analyser.get()[0].kind(),
                    Tokens::Keyword(Keywords::If)
                )
        );

        let token = parser.analyser.get()[0].clone();
        if previous.is_some() {
            return lang_err!(UnexpectedToken(token));
        }

        if let Some(slice) = parser.analyser.lookup(0, 1) {
            if !matches!(
                slice[0].kind(),
                Tokens::Block(Toggleable::Openning) | Tokens::ScopeTarget(_)
            ) {
                return lang_err!(Expected {
                    after: token.location().clone(),
                    expected: Some(String::from("(<condition>, <code>)")),
                    but_found: Some(slice[0].location().code().to_string())
                });
            }
        } else {
            return lang_err!(UnexpectedToken(token));
        };

        parser.analyser.next(0, 1);
        let openner = parser.analyser.get()[0].clone();
        let (scope, branches) = parser.scope(None, None, None)?;

        let arguments_location =
            LangModuleSlice::from(&vec![openner.location().clone(), parser.analyser_slice()]);

        if !(2..=3).contains(&branches.len()) {
            return lang_err!(UnableToParse(
                arguments_location,
                format!("Expected 2 or 3 arguments. Found {}.", branches.len())
            ));
        }

        let condition_instruction = branches[0].clone();
        if condition_instruction.len() != 1 {
            return lang_err!(UnableToParse(
                arguments_location,
                format!(
                    "Unable to validate the condition. Expected 1 expression, but found {}.",
                    condition_instruction.len()
                )
            ));
        }
        let Instructions::ValueOf(condition_expression) = condition_instruction[0].kind() else {
            return lang_err!(UnexpectedNode(condition_instruction[0].clone()));
        };
        let condition = condition_instruction[0].clone_as(|_, l| (condition_expression.clone(), l));

        // The only way a if condition can have 3 arguments is if it is a ternary expression
        if branches.len() == 3 {
            if let Some(scope) = scope {
                return lang_err!(UnexpectedToken(Token::new(
                    Tokens::ScopeTarget(scope.kind().clone()),
                    scope.location()
                )));
            }

            let mut yes = None;
            let mut no = None;

            for (index, instructions) in branches.iter().skip(1).enumerate() {
                if instructions.len() != 1 {
                    return lang_err!(UnableToParse(
                        arguments_location,
                        format!(
                            "Unable to parse ternary items : expected 1 expression per argument, but found {}.",
                            instructions.len()
                        )
                    ));
                }

                let instruction = instructions[0].clone();
                let Instructions::ValueOf(expression) = instruction.kind() else {
                    return lang_err!(UnexpectedNode(instruction));
                };
                let boxed_expression = instruction.clone_as(|_, l| (expression.clone(), l));

                match index {
                    0 => {
                        yes = Some(boxed_expression.into());
                    }
                    1 => {
                        no = Some(boxed_expression.into());
                    }
                    _ => panic!("Invalid above tests."),
                }
            }

            return Ok(Node::new(
                IfResult::Ternary(Ternary {
                    condition: condition.into(),
                    yes: yes.expect("Invalid above tests (yes)."),
                    no: no.expect("Invalid above tests (no)."),
                }),
                &LangModuleSlice::from(&vec![token.location().clone(), parser.analyser_slice()]),
            ));
        };

        let process = branches[1].clone();
        // If there is an "else"
        let fallback = if let Some(slice) = parser.analyser.lookup(0, 1) {
            // Else token (see bellow)
            if matches!(slice[0].kind(), Tokens::Keyword(Keywords::Else)) {
                parser.analyser.next(0, 1);

                Some(Box::new(IfFallBack::parse(parser, previous)?))
            } else {
                None
            }
        } else {
            None
        };

        let location = LangModuleSlice::from(&vec![
            token.location().clone(),
            if let Some(fb) = &fallback {
                fb.location().clone()
            } else {
                parser.analyser_slice()
            },
        ]);

        Ok(Node::new(
            IfResult::If(Self {
                condition,
                process: process.into(),
                fallback,
                scope_target: scope,
            }),
            &location,
        ))
    }
}

impl Parsable for IfFallBack {
    type ResultKind = Self;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<Node>,
    ) -> crate::flylang::errors::LangResult<Node<Self::ResultKind>> {
        assert!(
            parser.analyser.min_len(1)
                && matches!(
                    parser.analyser.get()[0].kind(),
                    Tokens::Keyword(Keywords::Else)
                )
        );

        let token = parser.analyser.get()[0].clone();
        if !parser.analyser.able_to(0, 1) {
            return lang_err!(UnexpectedToken(token));
        };
        parser.analyser.next(0, 1);

        match parser.analyser.get()[0].kind() {
            Tokens::Keyword(Keywords::If) => {
                let result = If::parse(parser, previous)?;
                let condition = match result.kind() {
                    IfResult::If(c) => c,
                    IfResult::Ternary(t) => {
                        return lang_err!(UnexpectedNode(result.clone_as(|_, l| (
                            Instructions::ValueOf(Expressions::Ternary(t.clone())),
                            l
                        ))));
                    }
                };

                let location = LangModuleSlice::from(&vec![
                    token.location().clone(),
                    result.location().clone(),
                ]);
                Ok(Node::new(
                    Self::If(result.clone_as(|_, l| (condition.clone(), l))),
                    &location,
                ))
            }
            Tokens::Block(Toggleable::Openning) => {
                // Skipping the openning tag
                parser.analyser.next(0, 0);
                let branches = parser.branches(
                    |_, token| matches!(token.kind(), Tokens::Block(Toggleable::Closing)),
                    |_, _| false,
                    None,
                )?;

                let location =
                    LangModuleSlice::from(&vec![token.location().clone(), parser.analyser_slice()]);

                Ok(Node::new(
                    Self::Process(None, branches[0].clone()),
                    &location,
                ))
            }
            _ => {
                lang_err!(Expected {
                    after: token.location().clone(),
                    expected: Some(String::from("( or <if_instruction>")),
                    but_found: Some(parser.analyser.get()[0].location().code().to_string())
                })
            }
        }
    }
}
