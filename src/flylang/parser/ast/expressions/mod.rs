use crate::flylang::{
    errors::lang_err,
    lexer::tokens::{Keywords, Operator, Toggleable, Tokens},
    module::slice::LangModuleSlice,
    parser::{
        ast::{
            BoxedNode, Node,
            definables::Definables,
            expressions::{
                call::Call,
                instanciate::ClassInstanciation,
                literals::ParsedLiterals,
                modified::ModifiedDefinable,
                objects::{Array, PrimaryObject, Structure},
                operations::Operations,
                property::ReadProperty,
                reverse::{Reverse, ReverseKind},
                ternary::Ternary,
            },
            instructions::Instructions,
        },
        errors::{Expected, UnexpectedNode, UnexpectedToken},
        mods::ParserBehaviors,
        parsable::Parsable,
    },
};

pub mod call;
pub mod instanciate;
pub mod literals;
pub mod modified;
pub mod objects;
pub mod operations;
pub mod property;
pub mod reverse;
pub mod ternary;

#[derive(Debug, Clone)]
pub enum Expressions {
    Literal(ParsedLiterals),
    Defined(Definables),
    Read(ReadProperty),
    ReturnOf(Call),
    Reverse(Reverse),
    Operation(Operations),
    Prioritized(BoxedNode<Expressions>),
    Ternary(Ternary),
    Structure(Structure),
    Array(Array),
    Modifed(ModifiedDefinable),
    Instanciate(ClassInstanciation),
}

impl Expressions {
    /// If the expression is a prioritized expression,
    /// skip the prioritized variant (and children) to get the actual expression
    pub fn unprioritized(&self) -> &Self {
        match self {
            Self::Prioritized(expr) => expr.kind().unprioritized(),
            _ => self,
        }
    }
}

impl Parsable for Expressions {
    type ResultKind = Self;

    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<super::Node>,
    ) -> crate::flylang::errors::LangResult<super::Node<Self::ResultKind>> {
        parser.analyser.min_len(1);
        assert_eq!(parser.analyser.range().len(), 1);

        let token = parser.analyser.get()[0].clone();

        let node = match token.kind() {
            Tokens::Literal(_) => {
                let literal = ParsedLiterals::parse(parser, previous)?;
                literal.clone_as(|k, l| (Self::Literal(k), l))
            }
            Tokens::Modifier => {
                let modified = ModifiedDefinable::parse(parser, previous)?;
                modified.clone_as(|k, l| (Self::Modifed(k), l))
            }
            Tokens::VarDef(_) | Tokens::Keyword(Keywords::Fn | Keywords::Cs) => {
                let defined = Definables::parse(parser, previous)?;
                defined.clone_as(|k, l| (Self::Defined(k), l))
            }
            Tokens::Keyword(Keywords::If) => {
                // The ternary expressions is parsed in the same code as a if block (parsed in instructions)
                parser.behaviors.insert(ParserBehaviors::Lazy);
                let instruction = Instructions::parse(parser, previous)?;
                let Instructions::ValueOf(Expressions::Ternary(ternary)) = instruction.kind()
                else {
                    return lang_err!(UnexpectedNode(instruction));
                };

                Node::new(Self::Ternary(ternary.clone()), instruction.location())
            }
            Tokens::Keyword(Keywords::New) => {
                let instanciation = ClassInstanciation::parse(parser, previous)?;
                instanciation.clone_as(|k, l| (Self::Instanciate(k), l))
            }
            Tokens::Object(Toggleable::Openning) => {
                let object = PrimaryObject::parse(parser, previous)?;
                object.clone_as(|kind, l| {
                    (
                        match kind {
                            PrimaryObject::Arr(arr) => Self::Array(arr),
                            PrimaryObject::Struct(structure) => Self::Structure(structure),
                        },
                        l,
                    )
                })
            }
            Tokens::Not => {
                if !parser.analyser.able_to(0, 1) {
                    return lang_err!(UnexpectedToken(token));
                };
                parser.analyser.next(0, 1);

                let reverse = if previous.is_some() {
                    if matches!(
                        parser.analyser.get()[0].kind(),
                        Tokens::Comparison(_) | Tokens::BinaryOperator(_)
                    ) {
                        // Cover the case "xx !< yy", "xx !& yy", ...
                        Operations::parse(parser, previous)?
                            .clone_as(|k, l| (Expressions::Operation(k), l))
                    } else {
                        return lang_err!(UnexpectedToken(token));
                    }
                } else {
                    parser.behaviors.insert(ParserBehaviors::Lazy);
                    Expressions::parse(parser, None)?
                };

                let location = LangModuleSlice::from(&vec![
                    token.location().clone(),
                    reverse.location().clone(),
                ]);

                Node::new(
                    Self::Reverse(Reverse {
                        kind: ReverseKind::Boolean,
                        expression: reverse.into(),
                    }),
                    &location,
                )
            }
            Tokens::Operator(_) | Tokens::BinaryOperator(_) | Tokens::Comparison(_) => {
                if previous.is_none()
                    && matches!(token.kind(), Tokens::Operator(Operator::Substract))
                {
                    // Handle the case "-xxxx" which invert the sign
                    if !parser.analyser.able_to(0, 1) {
                        return lang_err!(UnexpectedToken(token));
                    }
                    parser.analyser.next(0, 1);

                    parser.behaviors.insert(ParserBehaviors::Lazy);
                    let invert = Self::parse(parser, None)?;
                    let location = LangModuleSlice::from(&vec![
                        token.location().clone(),
                        invert.location().clone(),
                    ]);

                    // If the inversed element is a number, then merge the number and the sign into one single number
                    if matches!(invert.kind(), Expressions::Literal(ParsedLiterals::Number)) {
                        invert.clone_as(|k, _| (k, location.clone()))
                    } else {
                        Node::new(
                            Self::Reverse(Reverse {
                                kind: ReverseKind::Sign,
                                expression: invert.into(),
                            }),
                            &location,
                        )
                    }
                } else {
                    let operation = Operations::parse(parser, previous)?;
                    operation.clone_as(|k, l| {
                        (
                            // We re-order the operation only if we're not lazy
                            // This prevents to reorder at each parsing step.
                            Expressions::Operation(k),
                            l,
                        )
                    })
                }
            }

            Tokens::Block(Toggleable::Openning) => {
                if let Some(previous) = previous {
                    // Function call
                    parser.behaviors.insert(ParserBehaviors::Lazy);
                    Call::parse(parser, Some(previous))?
                        .clone_as(|k, l| (Expressions::ReturnOf(k), l))
                } else {
                    // Priority
                    // Thanks to the lexer, the priority is sure to have an ending part.
                    parser.analyser.next(0, 1);

                    parser.behaviors.remove(&ParserBehaviors::Lazy);
                    // Empty block in expression = Empty literal
                    if matches!(
                        parser.analyser.get()[0].kind(),
                        Tokens::Block(Toggleable::Closing)
                    ) {
                        Node::new(
                            Self::Literal(ParsedLiterals::Empty),
                            &LangModuleSlice::from(&vec![
                                token.location().clone(),
                                parser.analyser_slice(),
                            ]),
                        )
                    } else {
                        let expr = Expressions::parse(parser, None)?;
                        let Some(closing) = parser.analyser.lookup(0, 1) else {
                            return lang_err!(Expected {
                                after: expr.location().clone(),
                                expected: Some(String::from(")")),
                                but_found: None
                            });
                        };

                        if !matches!(closing[0].kind(), Tokens::Block(Toggleable::Closing)) {
                            return lang_err!(Expected {
                                after: expr.location().clone(),
                                expected: Some(String::from(")")),
                                but_found: Some(closing[0].location().code().to_string())
                            });
                        };

                        // Include the closing block
                        parser.analyser.increase(1);

                        // Now the expression is the whole block, so we include the openning/closing tags in it
                        let priority_location = LangModuleSlice::from(&vec![
                            token.location().clone(),
                            parser.analyser_slice(),
                        ]);

                        let priority =
                            Node::new(Self::Prioritized(expr.into()), &priority_location);

                        // In lazy-mode, we return early the priority
                        // because we included the ending block inside it
                        if parser.behaviors.contains(&ParserBehaviors::Lazy) {
                            return Ok(priority);
                        }

                        priority
                    }
                }
            }
            Tokens::Accessor => {
                ReadProperty::parse(parser, previous)?.clone_as(|k, l| (Expressions::Read(k), l))
            }
            _ => return lang_err!(UnexpectedToken(token)),
        };

        if let Some(slice) = parser.analyser.lookup(0, 1) {
            let kind = slice[0].kind();
            // ? Explainations:
            // In lazy mode, continue parsing only if:
            // -> Next token mustn't be processed in lazy-mode
            //
            // Also, if the next token is an end of instruction, we stop the expression parsing
            if !(matches!(
                kind,
                Tokens::Block(Toggleable::Closing)
                    | Tokens::Object(Toggleable::Closing)
                    | Tokens::EndOfInstruction
                    | Tokens::ArgSeparator
            ) || parser.behaviors.contains(&ParserBehaviors::Lazy)
                && matches!(kind, Tokens::Operator(_) | Tokens::BinaryOperator(_)))
            {
                parser.analyser.next(0, 1);
                return Self::parse(
                    parser,
                    Some(node.clone_as(|e, l| (Instructions::ValueOf(e), l))),
                );
            }
        }

        Ok(node)
    }
}
