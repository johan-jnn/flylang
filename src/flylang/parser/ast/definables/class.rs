use crate::flylang::{
    errors::lang_err,
    lexer::tokens::{Keywords, Literals, Toggleable, Tokens},
    module::slice::LangModuleSlice,
    parser::{
        ast::{
            Node,
            definables::{
                Definables,
                functions::DefineFunction,
                variables::{DefineVariable, VariableEmplacements},
            },
            expressions::{
                Expressions,
                literals::{ParsedLiterals, Word},
            },
            instructions::Instructions,
        },
        errors::{Expected, UnableToParse, UnexpectedNode, UnexpectedToken},
        parsable::Parsable,
    },
};

#[derive(Debug, Clone)]
pub enum ClassItemVisibility {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone)]
pub struct ClassItem<Kind> {
    pub visibility: ClassItemVisibility,
    pub is_static: bool,
    pub item: Kind,
    pub modifie_by: Vec<Node<Word>>,
}

#[derive(Debug, Clone)]
pub struct DefineClass {
    pub name: Node<Word>,
    pub constructor: Option<Node<DefineFunction>>,
    pub parents: Vec<Node<Word>>,

    pub attributes: Vec<Node<ClassItem<DefineVariable>>>,
    pub methods: Vec<Node<ClassItem<DefineFunction>>>,
}

impl Parsable for DefineClass {
    type ResultKind = Self;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<Node>,
    ) -> crate::flylang::errors::LangResult<Node<Self::ResultKind>> {
        assert!(
            parser.analyser.min_len(1)
                && matches!(
                    parser.analyser.get()[0].kind(),
                    Tokens::Keyword(Keywords::Cs)
                )
        );

        let keyword = parser.analyser.get()[0].clone();
        if previous.is_some() {
            return lang_err!(UnexpectedToken(keyword));
        }

        if !parser.analyser.able_to(0, 1) {
            return lang_err!(Expected {
                after: keyword.location().clone(),
                expected: Some(String::from("class name")),
                but_found: None
            });
        }
        parser.analyser.next(0, 1);

        let token = &parser.analyser.get()[0];
        let name = if let Tokens::Literal(Literals::Word) = token.kind() {
            Node::new(Word, token.location())
        } else {
            return lang_err!(UnexpectedToken(token.clone()));
        };

        parser.analyser.next(0, 0);
        if !(parser.analyser.min_len(1)
            && matches!(
                parser.analyser.get()[0].kind(),
                Tokens::Block(Toggleable::Openning)
            ))
        {
            return lang_err!(Expected {
                after: if parser.analyser.range().is_empty() {
                    LangModuleSlice::new_with(parser.module(), parser.module().tail_range())
                } else {
                    parser.analyser_slice()
                },
                expected: Some("'('".to_string()),
                but_found: None
            });
        };

        parser.analyser.next(0, 0);
        let branches = parser.branches(
            |_, t| matches!(t.kind(), Tokens::Block(Toggleable::Closing)),
            |_, t| matches!(t.kind(), Tokens::ArgSeparator),
            None,
        )?;

        let location =
            LangModuleSlice::from(&vec![keyword.location().clone(), parser.analyser_slice()]);

        let mut constructor = None;
        let mut body_processed = false;
        let mut methods = vec![];
        let mut attributes = vec![];
        let mut parents = vec![];

        if branches.len() > 1 || !branches[0].is_empty() {
            for branch in branches {
                if branch.len() == 1 {
                    match branch[0].kind() {
                        Instructions::ValueOf(Expressions::Defined(Definables::Function(
                            defined,
                        ))) => {
                            // constructor
                            if constructor.is_none() {
                                constructor =
                                    Some(Node::new(defined.clone(), branch[0].location()));
                                continue;
                            }
                        }
                        Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Word)) => {
                            // parent
                            if constructor.is_some() || body_processed {
                                return lang_err!(UnexpectedNode(branch[0].clone()));
                            }

                            parents.push(Node::new(Word, branch[0].location()));
                            continue;
                        }
                        _ => (),
                    }
                }
                // body
                if body_processed {
                    return lang_err!(UnableToParse(
                        if branch.is_empty() {
                            location
                        } else {
                            branch[0].location().clone()
                        },
                        String::from("Cannot declare multiple body")
                    ));
                }

                for instruction in branch {
                    let mut modifiers = vec![];
                    let defined_element: &Definables = match &instruction.kind() {
                        Instructions::ValueOf(Expressions::Defined(def)) => def,
                        Instructions::ValueOf(Expressions::Modifed(modif)) => {
                            modifiers = modif.modified_by.clone();
                            modif.definable.kind()
                        }
                        _ => {
                            return lang_err!(UnexpectedNode(instruction));
                        }
                    };

                    match defined_element {
                        Definables::Function(method) => {
                            if method.name.is_none() {
                                return lang_err!(UnableToParse(
                                    instruction.location().clone(),
                                    String::from("Methods must have a name")
                                ));
                            }

                            methods.push(Node::new(
                                ClassItem {
                                    visibility: ClassItemVisibility::Public,
                                    is_static: false,
                                    item: method.clone(),
                                    modifie_by: modifiers,
                                },
                                instruction.location(),
                            ));
                        }
                        Definables::Variable(attribute) => {
                            if !matches!(attribute.emplacement.kind(), VariableEmplacements::Scope)
                            {
                                return lang_err!(UnableToParse(
                                    attribute.emplacement.location().clone(),
                                    String::from("Invalid attribute name")
                                ));
                            };

                            attributes.push(Node::new(
                                ClassItem {
                                    visibility: ClassItemVisibility::Public,
                                    is_static: false,
                                    item: attribute.clone(),
                                    modifie_by: modifiers,
                                },
                                instruction.location(),
                            ));
                        }
                        _ => {
                            return lang_err!(UnexpectedNode(instruction));
                        }
                    }
                }

                body_processed = true;
            }
        }

        Ok(Node::new(
            Self {
                name,
                constructor,
                parents,
                methods,
                attributes,
            },
            &location,
        ))
    }
}
