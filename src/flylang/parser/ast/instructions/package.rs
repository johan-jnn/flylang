use crate::flylang::{
    errors::lang_err, lexer::{ranges::{IS_FILE_LOCATION_IF_STARTS_WITH, in_ranges}, tokens::{Keywords, Literals, Toggleable, Tokens}}, module::slice::LangModuleSlice, parser::{
        ast::{
            Node,
            expressions::{Expressions, literals::{ParsedLiterals, ParsedStringItem, Word}}, instructions::Instructions,
        },
        errors::{Expected, UnableToParse, UnexpectedNode, UnexpectedToken},
        parsable::Parsable,
    }
};

#[derive(Debug, Clone)]
pub enum PackageSource {
    Package,
    File,
}

#[derive(Debug, Clone)]
pub enum PackageIncludedContent {
    All,
    Only(Vec<Node<Word>>)
}

#[derive(Debug, Clone)]
pub enum PackageContentEmplacement {
    Global,
    Variable(Node<Word>),
}

#[derive(Debug, Clone)]
pub struct Package {
    pub source: PackageSource,
    pub included: PackageIncludedContent,
    pub emplacement: PackageContentEmplacement,
}

impl Parsable for Package {
    type ResultKind = Self;
    fn parse(
        parser: &mut crate::flylang::parser::Parser,
        previous: Option<Node>,
    ) -> crate::flylang::errors::LangResult<Node<Self::ResultKind>> {
        assert!(
            parser.analyser.min_len(1)
                && matches!(
                    parser.analyser.get()[0].kind(),
                    Tokens::Keyword(Keywords::Use)
                )
        );

        let token = parser.analyser.get()[0].clone();
        if previous.is_some() {
            return lang_err!(UnexpectedToken(token));
        }

        if !parser.analyser.able_to(0, 1) {
            return lang_err!(Expected {
                after: token.location().clone(),
                expected: Some("Package location".into()),
                but_found: None
            });
        }

        parser.analyser.next(0, 1);
        let src_token = parser.analyser.get()[0].clone();
        if !matches!(src_token.kind(), Tokens::Literal(_)) {
            return lang_err!(UnexpectedToken(src_token));
        };
        let src = ParsedLiterals::parse(parser, None)?;
        let ParsedLiterals::String(src_values) = src.kind() else {
            return lang_err!(UnexpectedToken(src_token));
        };

        let Some(src_location) = (if src_values.len() == 1 {
            if let ParsedStringItem::Literal(val) = src_values[0].kind().clone() {
                Some(val)
            } else {
                None
            }
        } else {
            None
        }) else {
            return lang_err!(
                UnableToParse(
                    src_token.location().clone(), 
                    "Expected a string to describe where to find the package. Note that the string must not include expressions in it.".into()
                )
            );
        };

        if src_location.is_empty() {
            return lang_err!(UnableToParse(src_token.location().clone(), "Invalid package path.".into()));
        }

        let source = if in_ranges!(IS_FILE_LOCATION_IF_STARTS_WITH, src_location.chars().next().unwrap()) {
            PackageSource::File
        } else {
            PackageSource::Package
        };

        let mut included  = PackageIncludedContent::All;
        if let Some(next) = parser.analyser.lookup(0, 1) {
            if matches!(next[0].kind(), Tokens::Block(Toggleable::Openning)) {
                parser.analyser.next(1, 0);
                let res = parser.branches(
                    |_, t| matches!(t.kind(), Tokens::Block(Toggleable::Closing)),
                     |_, t| matches!(t.kind(), Tokens::ArgSeparator), 
                     None
                    )?;
                
                let mut included_vec: Vec<Node<Word>> = vec![];
                
                for branch in res {
                    if branch.len() != 1 {
                        return lang_err!(UnableToParse(parser.analyser_slice(), "Invalid syntax inside the block".into()));
                    };

                    let include = branch[0].clone();
                    if !matches!(include.kind(), Instructions::ValueOf(Expressions::Literal(ParsedLiterals::Word))) {
                        return lang_err!(UnexpectedNode(include));
                    };
                    included_vec.push(include.clone_as(|_, l| (Word, l)));
                };

                included = PackageIncludedContent::Only(included_vec);
            }
        };

        let mut emplacement = PackageContentEmplacement::Global;
        if let Some(next) = parser.analyser.lookup(0, 1) {
            if matches!(next[0].kind(), Tokens::Keyword(Keywords::In)) {
                if let Some(renext) = parser.analyser.lookup(1, 1) {
                    if matches!(renext[0].kind(), Tokens::Literal(Literals::Word)) {
                        emplacement = PackageContentEmplacement::Variable(Node::new(Word, renext[0].location()));
                        parser.analyser.next(0, 2);
                    }
                }
            }   
        }

        let location = LangModuleSlice::from(&vec![
            token.location().clone(),
            parser.analyser_slice()
        ]);

        Ok(Node::new(
            Self {
                source,
                included,
                emplacement
            },
            &location
        ))
    }
}
