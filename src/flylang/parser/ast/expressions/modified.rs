use crate::flylang::{
    errors::lang_err,
    lexer::tokens::Tokens,
    module::slice::LangModuleSlice,
    parser::{
        ast::{
            Node,
            definables::Definables,
            expressions::{
                Expressions,
                call::Call,
                literals::{ParsedLiterals, Word},
            },
            instructions::Instructions,
        },
        errors::{UnableToParse, UnexpectedToken},
        parsable::Parsable,
    },
};

#[derive(Clone, Debug)]
pub enum Modifier {
    DefinedElement,
    CallReturn(Call),
}

#[derive(Clone, Debug)]
pub struct ModifiedDefinable {
    pub definable: Node<Definables>,
    pub modified_by: Vec<Node<Modifier>>,
}

impl Parsable for ModifiedDefinable {
    type ResultKind = Self;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<Node>,
    ) -> crate::flylang::errors::LangResult<Node<Self::ResultKind>> {
        assert!(
            parser.analyser.min_len(1)
                && matches!(parser.analyser.get()[0].kind(), Tokens::Modifier)
        );

        let modifier_token = parser.analyser.get()[0].clone();
        parser.analyser.next(0, 0);

        if previous.is_some() {
            return lang_err!(UnexpectedToken(modifier_token));
        }

        let (target, modifiers) = parser.scope(None, None, None)?;
        if let Some(target) = target {
            return lang_err!(UnableToParse(
                target.location().clone(),
                String::from("Invalid syntax")
            ));
        };

        let mut words = vec![];
        for modifier in modifiers {
            if modifier.is_empty() {
                return lang_err!(UnableToParse(
                    modifier_token.location().clone(),
                    String::from("Invalid syntax in the modifiers list")
                ));
            }

            let modifier_range: Vec<LangModuleSlice> =
                modifier.iter().map(|n| n.location().clone()).collect();
            let slice = LangModuleSlice::from(&modifier_range);

            if modifier.len() != 1 {
                return lang_err!(UnableToParse(slice, String::from("Expected function")));
            }
            let node = &modifier[0];

            words.push(Node::new(
                match &node.kind() {
                    Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Word)) => {
                        Modifier::DefinedElement
                    }
                    Instructions::ValueOf(Expressions::ReturnOf(call)) => {
                        Modifier::CallReturn(call.clone())
                    }
                    _ => return lang_err!(UnableToParse(slice, String::from("Expected function"))),
                },
                node.location(),
            ));
        }

        parser.analyser.next(0, 0);
        if !parser.analyser.able_to_increase(1) {
            return lang_err!(UnableToParse(
                LangModuleSlice::new_with(parser.module(), parser.module().tail_range()),
                String::from("Expected function")
            ));
        }

        let expression = Expressions::parse(parser, previous)?;
        let definable = if let Expressions::Defined(def) = expression.kind() {
            Node::new(def.clone(), expression.location())
        } else {
            return lang_err!(UnableToParse(
                expression.location().clone(),
                String::from("Expected a function or variable definition")
            ));
        };
        let location = LangModuleSlice::from(&vec![
            modifier_token.location().clone(),
            definable.location().clone(),
        ]);

        Ok(Node::new(
            Self {
                definable,
                modified_by: words,
            },
            &location,
        ))
    }
}
