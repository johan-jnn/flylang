use std::collections::VecDeque;

use crate::flylang::{
    errors::lang_err,
    lexer::tokens::{Keywords, ScopeTarget, Tokens},
    module::slice::LangModuleSlice,
    parser::{
        ast::{
            BoxedBranches, BoxedNode, Node,
            expressions::{
                Expressions,
                literals::{ParsedLiterals, Word},
                reverse::{Reverse, ReverseKind},
            },
            instructions::Instructions,
        },
        errors::{UnableToParse, UnexpectedNode, UnexpectedToken},
        parsable::Parsable,
    },
};

#[derive(Debug, Clone)]
pub enum LoopParameter {
    Through(Each),
    Conditionnaly(While),
}

#[derive(Debug, Clone)]
pub struct Loop {
    pub parameter: LoopParameter,
    pub process: BoxedBranches,
    pub scope_target: Option<Node<ScopeTarget>>,
}

#[derive(Debug, Clone)]
pub struct Each {
    pub iterable: BoxedNode<Expressions>,
    pub item: Option<Node<Word>>,
    pub index: Option<Node<Word>>,
}

#[derive(Debug, Clone)]
// "Until" loops are converted to a while loop with a wrapper "not()" condition
pub struct While {
    pub condition: BoxedNode<Expressions>,
    pub iteration_number: Option<Node<Word>>,
}

impl Parsable for Loop {
    type ResultKind = Self;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<Node>,
    ) -> crate::flylang::errors::LangResult<Node<Self::ResultKind>> {
        assert!(
            parser.analyser.min_len(1)
                && matches!(
                    parser.analyser.get()[0].kind(),
                    Tokens::Keyword(Keywords::While | Keywords::Until | Keywords::Each)
                )
        );

        let token = parser.analyser.get()[0].clone();
        if previous.is_some() {
            return lang_err!(UnexpectedToken(token));
        };

        // skip the openner
        parser.analyser.next(0, 0);

        let (target_scope, branches_vec) = parser.scope(None, None, None)?;
        let mut branches = VecDeque::from(branches_vec);

        let scope_slice = parser.analyser_slice();

        // The only difference between an "each" loop and "while"/"until" loops (in the ast) is that the "each" loops accept a "current-item" argument.
        // So because the syntax is similar, we parse them in the same way.
        let is_conditional = !matches!(token.kind(), Tokens::Keyword(Keywords::Each));

        // condirator = condition + iterator (one or the other)
        let condirator_instructions = branches.pop_front().unwrap();
        if condirator_instructions.len() != 1 {
            return lang_err!(UnableToParse(
                scope_slice,
                format!(
                    "Missing the {}",
                    if is_conditional {
                        "condition"
                    } else {
                        "iteratable"
                    }
                )
            ));
        }
        let Instructions::ValueOf(condirator_expression) = condirator_instructions[0].kind() else {
            return lang_err!(UnexpectedNode(condirator_instructions[0].clone()));
        };
        let condirator =
            condirator_instructions[0].clone_as(|_, l| (condirator_expression.clone(), l));

        // The following define where is the "item" (the `true` value) argument.
        // It defines also the number of variable can be passed before the loop's code
        let is_item_table = if is_conditional {
            // Items are not valid in the "while"/"until" loops. So there can only be 1 parameter, and it's the index parameter
            vec![false]
        } else {
            vec![true, false]
        };

        let mut item = None;
        let mut index = None;

        for is_item in is_item_table {
            if branches.len() <= 1 {
                break;
            }

            let element_instruction = branches.pop_front().unwrap();
            if element_instruction.len() != 1 {
                return lang_err!(UnableToParse(
                    scope_slice,
                    "Invalid arguments given.".to_string()
                ));
            }
            if !matches!(
                element_instruction[0].kind(),
                Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Word))
            ) {
                return lang_err!(UnexpectedNode(element_instruction[0].clone()));
            }

            if is_item {
                item = Some(element_instruction[0].clone_as(|_, l| (Word, l)))
            } else {
                index = Some(element_instruction[0].clone_as(|_, l| (Word, l)))
            }
        }

        let parameter: LoopParameter = match token.kind() {
            Tokens::Keyword(Keywords::Each) => LoopParameter::Through(Each {
                iterable: condirator.into(),
                item,
                index,
            }),
            _ => {
                // while or until
                LoopParameter::Conditionnaly(While {
                    condition: if matches!(token.kind(), Tokens::Keyword(Keywords::Until)) {
                        condirator_instructions[0].clone_as(|_, l| {
                            (
                                Expressions::Reverse(Reverse {
                                    kind: ReverseKind::Boolean,
                                    expression: condirator.clone().into(),
                                }),
                                l,
                            )
                        })
                    } else {
                        condirator
                    }
                    .into(),
                    iteration_number: index,
                })
            }
        };

        if branches.len() != 1 {
            return lang_err!(UnableToParse(
                scope_slice,
                String::from(
                    "Too much arguments or missing code to execute each loop repetition. (Add a trailing ',' to give an empty execution)"
                )
            ));
        }

        Ok(Node::new(
            Self {
                parameter,
                process: branches.pop_front().unwrap().into(),
                scope_target: target_scope
                    .map(|target| Node::new(target.kind().clone(), target.location())),
            },
            &LangModuleSlice::from(&vec![token.location().clone(), parser.analyser_slice()]),
        ))
    }
}
