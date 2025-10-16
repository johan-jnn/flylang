use crate::flylang::{
    errors::lang_err,
    lexer::tokens::{Tokens, VarDefinition},
    module::slice::LangModuleSlice,
    parser::{
        ast::{
            BoxedNode, Node,
            expressions::{
                Expressions,
                literals::ParsedLiterals,
                operations::{Operation, Operations},
                property::ReadProperty,
            },
            instructions::Instructions,
        },
        errors::{Expected, UnexpectedNode, UnexpectedToken},
        mods::ParserBehaviors,
        parsable::Parsable,
    },
};

#[derive(Debug, Clone)]
pub enum VariableEmplacements {
    Scope,
    Property(ReadProperty),
    /// This should never be used.
    ///
    /// It is here to allow structure to have any expression as key (expect object access)
    Any(Box<Expressions>),
}

#[derive(Debug, Clone)]
pub struct DefineVariable {
    pub emplacement: Node<VariableEmplacements>,
    pub value: BoxedNode<Expressions>,
    pub readonly: bool,
}

impl Parsable for DefineVariable {
    type ResultKind = Self;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<crate::flylang::parser::ast::Node>,
    ) -> crate::flylang::errors::LangResult<crate::flylang::parser::ast::Node<Self::ResultKind>>
    {
        assert!(
            parser.analyser.min_len(1)
                && matches!(parser.analyser.get()[0].kind(), Tokens::VarDef(_))
        );

        let token = parser.analyser.get()[0].clone();
        let Tokens::VarDef(def_kind) = token.kind() else {
            // Condition verified above.
            panic!()
        };

        let Some(emplacement_instruction) = previous else {
            return lang_err!(UnexpectedToken(token));
        };
        let Instructions::ValueOf(emplacement_expression) = emplacement_instruction.kind() else {
            return lang_err!(UnexpectedNode(emplacement_instruction));
        };

        let any_recursive = parser
            .behaviors
            .contains(&ParserBehaviors::AllowAnyVariableEmplacement(true));
        let allow_any = any_recursive
            || parser
                .behaviors
                .contains(&ParserBehaviors::AllowAnyVariableEmplacement(false));

        let emplacement = match emplacement_expression {
            Expressions::Literal(ParsedLiterals::Word) => VariableEmplacements::Scope,
            Expressions::Read(property) => VariableEmplacements::Property(property.clone()),
            _ => {
                if allow_any {
                    VariableEmplacements::Any(Box::new(emplacement_expression.clone()))
                } else {
                    return lang_err!(UnexpectedNode(emplacement_instruction));
                }
            }
        };

        // Set the analyser on the value instruction
        if !parser.analyser.able_to(0, 1) {
            return lang_err!(Expected {
                after: token.location().clone(),
                but_found: None,
                expected: Some(String::from("variable's new value"))
            });
        }
        parser.analyser.next(0, 1);

        parser.behaviors.remove(&ParserBehaviors::Lazy);
        if !any_recursive {
            parser.behaviors.retain(|behavior| {
                !matches!(behavior, ParserBehaviors::AllowAnyVariableEmplacement(_))
            });
        }
        let expression = Expressions::parse(parser, None)?;
        let def_location = LangModuleSlice::from(&vec![
            emplacement_instruction.location().clone(),
            parser.analyser_slice(),
        ]);

        let value = if let VarDefinition::WithOperation(operator) = def_kind {
            Node::new(
                Expressions::Operation(Operations::Numeric(Operation {
                    operator: operator.clone(),
                    operands: (
                        Node::new(
                            emplacement_expression.clone(),
                            emplacement_instruction.location(),
                        )
                        .into(),
                        expression.into(),
                    ),
                })),
                &def_location,
            )
        } else {
            expression
        };

        Ok(Node::new(
            Self {
                emplacement: Node::new(emplacement, emplacement_instruction.location()),
                value: Box::new(value),
                readonly: matches!(def_kind, VarDefinition::Constant),
            },
            &def_location,
        ))
    }
}
