use crate::flylang::{
    errors::{LangResult, RaisableErr, lang_err},
    lexer::tokens::{Literals, Toggleable, Tokens, VarDefinition},
    module::slice::LangModuleSlice,
    parser::{
        ast::{
            BoxedNodes, Node,
            definables::{
                Definables,
                variables::{DefineVariable, VariableEmplacements},
            },
            expressions::Expressions,
            instructions::Instructions,
        },
        errors::{UnableToParse, UnexpectedNode, UnexpectedToken},
        mods::ParserBehaviors,
        parsable::Parsable,
    },
};

#[derive(Debug, Clone)]
pub struct StructureEntry {
    pub key: Node<Expressions>,
    pub value: Node<Expressions>,
}
impl TryFrom<&DefineVariable> for StructureEntry {
    type Error = Box<dyn RaisableErr>;

    fn try_from(value: &DefineVariable) -> LangResult<Self> {
        if value.readonly {
            let mut token_slice = LangModuleSlice::new(value.emplacement.location().module());
            token_slice.set(
                value.emplacement.location().range().start..value.value.location().range().end,
            );

            return lang_err!(UnableToParse(
                token_slice,
                String::from("Invalid character. \"::\" cannot be used for structure items.")
            ));
        };

        let key = match value.emplacement.kind() {
            VariableEmplacements::Property(prop) => {
                return lang_err!(UnexpectedNode(Node::new(
                    Instructions::ValueOf(Expressions::Read(prop.clone())),
                    value.emplacement.location()
                )));
            }
            VariableEmplacements::Scope => Expressions::Literal(Literals::Word),
            VariableEmplacements::Any(expr) => expr.as_ref().clone(),
        };

        Ok(Self {
            key: Node::new(key, value.emplacement.location()),
            value: value.value.as_ref().clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Structure {
    pub entries: BoxedNodes<StructureEntry>,
}
#[derive(Debug, Clone)]
pub struct Array {
    pub entries: BoxedNodes<Expressions>,
}

// Because object and array has similar syntax, we parse them in the same place
#[derive(Debug, Clone)]
pub enum PrimaryObject {
    Struct(Structure),
    Arr(Array),
}

impl Parsable for PrimaryObject {
    type ResultKind = Self;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<Node>,
    ) -> crate::flylang::errors::LangResult<Node<Self::ResultKind>> {
        assert!(
            parser.analyser.min_len(1)
                && matches!(
                    parser.analyser.get()[0].kind(),
                    Tokens::Object(Toggleable::Openning)
                )
        );

        let token = parser.analyser.get()[0].clone();
        if previous.is_some() {
            return lang_err!(UnexpectedToken(token));
        }

        if let Some(slice) = parser.analyser.lookup(0, 2) {
            if matches!(slice[0].kind(), Tokens::VarDef(VarDefinition::Normal))
                && matches!(slice[1].kind(), Tokens::Object(Toggleable::Closing))
            {
                // Empty structure
                parser.analyser.increase(2);

                return Ok(Node::new(
                    Self::Struct(Structure { entries: vec![] }),
                    &parser.analyser_slice(),
                ));
            }
        };

        parser.analyser.next(0, 0);
        let branches = parser.branches(
            |_, token| matches!(token.kind(), Tokens::Object(Toggleable::Closing)),
            |_, token| matches!(token.kind(), Tokens::ArgSeparator),
            Some(vec![ParserBehaviors::AllowAnyVariableEmplacement(false)]),
        )?;

        let location =
            LangModuleSlice::from(&vec![token.location().clone(), parser.analyser_slice()]);
        if branches.len() == 1 && branches[0].is_empty() {
            // Empty array
            return Ok(Node::new(Self::Arr(Array { entries: vec![] }), &location));
        };

        let mut result: Option<Self> = None;

        for (index, branch) in branches.iter().enumerate() {
            if branch.len() != 1 {
                return lang_err!(UnableToParse(
                    location,
                    format!(
                        "Expected 1 single expression per object's item. Received {}",
                        branch.len()
                    )
                ));
            }
            let instruction = branch[0].clone();
            let Instructions::ValueOf(expression) = instruction.kind() else {
                return lang_err!(UnexpectedNode(instruction));
            };

            match expression {
                Expressions::Defined(Definables::Variable(variable)) => {
                    if index == 0 {
                        result = Some(Self::Struct(Structure {
                            entries: vec![
                                Node::new(
                                    StructureEntry::try_from(variable)?,
                                    instruction.location(),
                                )
                                .into(),
                            ],
                        }))
                    } else if let Some(Self::Struct(structure)) = result.as_mut() {
                        structure.entries.push(
                            Node::new(StructureEntry::try_from(variable)?, instruction.location())
                                .into(),
                        );
                    } else {
                        return lang_err!(UnexpectedNode(instruction));
                    }
                }
                _ => {
                    let entry = instruction.clone_as(|_, l| (expression.clone(), l));

                    if index == 0 {
                        result = Some(Self::Arr(Array {
                            entries: vec![entry.into()],
                        }))
                    } else if let Some(Self::Arr(arr)) = result.as_mut() {
                        arr.entries.push(entry.into());
                    } else {
                        return lang_err!(UnexpectedNode(instruction));
                    }
                }
            }
        }

        Ok(Node::new(
            result.expect("Empty objects has invalid tests"),
            &location,
        ))
    }
}
