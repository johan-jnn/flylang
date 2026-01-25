use crate::flylang::{
    errors::{LangResult, lang_err}, lexer::{ranges::{IS_FILE_LOCATION_IF_STARTS_WITH, in_ranges}, tokens::{Keywords, Literals, Toggleable, Token, Tokens}}, module::slice::LangModuleSlice, parser::{
        Parser, ast::{
            Node,
            expressions::{Expressions, literals::{ParsedLiterals, ParsedStringItem, Word}}, instructions::Instructions,
        }, errors::{Expected, UnableToParse, UnexpectedNode, UnexpectedToken}, parsable::Parsable
    }
};

#[derive(Debug, Clone)]
pub enum PackageSource {
    Package(String),
    File(String),
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

impl Package {
    fn parse_getters(parser: &mut crate::flylang::parser::Parser) -> LangResult<PackageIncludedContent> {
        if !(
            parser.analyser.min_len(1)
            && matches!(
                parser.analyser.get()[0].kind(),
                Tokens::Object(Toggleable::Openning)
                | Tokens::Block(Toggleable::Openning)
            )
        ) {
            return Ok(PackageIncludedContent::All)
        }

        let ender: fn(&Parser, &Token) -> bool=  if matches!(parser.analyser.get()[0].kind(), Tokens::Object(_)) {
            |_, t| matches!(t.kind(), Tokens::Object(Toggleable::Closing)) 
        } else {
            |_, t| matches!(t.kind(), Tokens::Block(Toggleable::Closing))
        };
        
        // Skip openner
        parser.analyser.next(0, 0);
        let res = parser.branches(
            ender,
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

        if let Some(expect_from) = parser.analyser.lookup(0, 1) 
        && matches!(expect_from[0].kind(), Tokens::Keyword(Keywords::From)) {
            parser.analyser.next(1, 0);
            return Ok(PackageIncludedContent::Only(included_vec))
        }

        lang_err!(Expected {
            after: 
                if parser.analyser.min_len(1) {
                    parser.analyser_slice()
                } else {
                    LangModuleSlice::new_with(parser.module(), parser.module().tail_range())
                },
            but_found:
                parser.analyser.lookup(1, 1).map(
                    |v| LangModuleSlice::from(
                        &v.iter()
                        .map(|t| t.location().clone()).collect::<Vec<LangModuleSlice>>()
                    )
                    .code()
                    .to_string()
                ),
            expected: 
                Some(String::from("from <module>"))
            })

    }

    fn parse_location(parser: &mut crate::flylang::parser::Parser) -> LangResult<PackageSource> {
        if !(
            parser.analyser.min_len(1)
            && matches!(
                parser.analyser.get()[0].kind(),
                Tokens::Literal(_)
            )
        ) {
            return lang_err!(UnexpectedToken (
                if parser.analyser.min_len(1) {
                    parser.analyser.get().last().unwrap().clone()
                } else {
                    parser.analyser.stream().last().unwrap().clone()
                }
            ));
        }

        let src_token = parser.analyser.get()[0].clone();
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
            return lang_err!(UnableToParse(src_token.location().clone(), "Invalid package path (It must be a string that does not contain expressions).".into()));
        }

        if in_ranges!(IS_FILE_LOCATION_IF_STARTS_WITH, src_location.chars().next().unwrap()) {
            Ok(PackageSource::File(src_location))
        } else {
            Ok(PackageSource::Package(src_location))
        }

    }

    fn parse_emplacement(parser: &mut crate::flylang::parser::Parser) -> LangResult<PackageContentEmplacement> {
        let mut emplacement = PackageContentEmplacement::Global;

        if let Some(next) = parser.analyser.lookup(0, 1) {
            if  matches!(next[0].kind(), Tokens::Keyword(Keywords::In))
                && let Some(renext) = parser.analyser.lookup(1, 1)
                    && matches!(renext[0].kind(), Tokens::Literal(Literals::Word)) {
                        emplacement = PackageContentEmplacement::Variable(Node::new(Word, renext[0].location()));
                        parser.analyser.next(0, 2);
                    }
            else if !matches!(next[0].kind(), Tokens::EndOfInstruction) {
                return lang_err!(UnexpectedToken(next[0].clone()));
            }
        }

        Ok(emplacement)
    }
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
        parser.analyser.next(0, 1);

        let included = Self::parse_getters(parser)?;
        let source = Self::parse_location(parser)?;
        let emplacement = Self::parse_emplacement(parser)?;

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
