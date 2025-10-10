use std::{collections::HashSet, mem::take, rc::Rc, vec};

use crate::flylang::{
    errors::{LangResult, lang_err},
    lexer::{
        Lexer,
        tokens::{ScopeTarget, Toggleable, Token, Tokens},
    },
    module::{LangModule, slice::LangModuleSlice},
    parser::{
        ast::{Branches, Node, instructions::Instructions},
        errors::Expected,
        mods::ParserBehaviors,
        parsable::Parsable,
    },
    utils::analyser::Analyser,
};

pub mod ast;
pub mod errors;
pub mod mods;
pub mod parsable;

#[derive(Debug)]
pub struct Parser {
    module: Rc<LangModule>,
    analyser: Analyser<Token<Tokens>>,
    parsed: Branches,
    behaviors: HashSet<ParserBehaviors>,
}
// ? only used in the `scope` method
type ScopeTokenMatcher = Box<dyn Fn(&Parser, &Token) -> bool>;

impl Parser {
    /// Create a new `Parser`
    /// Warning: the given stream must be valid : the openning/closing scopes will not be verified.
    pub fn new(module: &Rc<LangModule>, stream: Vec<Token<Tokens>>) -> Self {
        Self {
            module: Rc::clone(module),
            analyser: Analyser::new(stream),
            parsed: vec![],
            behaviors: HashSet::new(),
        }
    }
    pub fn module(&self) -> &Rc<LangModule> {
        &self.module
    }
    /// Get the module's slice of the analyser's range
    fn analyser_slice(&self) -> LangModuleSlice {
        assert!(
            !self.analyser.range().is_empty(),
            "Cannot create a slice from an empty analyser"
        );

        let slices: Vec<LangModuleSlice> = self
            .analyser
            .get()
            .iter()
            .map(|token| token.location().clone())
            .collect();

        LangModuleSlice::from(&slices)
    }

    /// Parse the next tokens as a scope.
    ///
    /// # Panics
    /// - If the length of the analyser, when calling this method, is > 1
    ///
    /// # Predictive return of `Err`
    /// - If the next instructions are not a new scope (`[@scope?](...)`)
    ///
    /// # Parameters
    /// Under the hood, this method uses the `branches` method. So to have more informations about the following parameters,
    /// see the `branches` method documentation.
    /// Only the default parameters' value are documented here, because the arguments are the same as the `branches` method.
    ///
    /// - `force_stop` -> `fn(_, token) => token == ')'`
    /// - `splitted_by` -> `fn(_, token) => token == ','`
    /// - `persistant_behaviors` -> `vec![]`
    ///
    /// # Analyser behavior
    /// If the result is valid, the analyser range is set from the openning scope character (`(`) to the character that closed it
    /// (it can be the end of the file, but in most cases it will be the closing scope character (`)`)).
    fn scope(
        &mut self,
        force_stop: Option<ScopeTokenMatcher>,
        splitted_by: Option<ScopeTokenMatcher>,
        persistant_behaviors: Option<Option<Vec<ParserBehaviors>>>,
    ) -> LangResult<(Option<Node<ScopeTarget>>, Vec<Branches>)> {
        assert!(self.analyser.range().len() <= 1);

        if !(self.analyser.min_len(1)
            && matches!(
                self.analyser.get()[0].kind(),
                Tokens::ScopeTarget(_) | Tokens::Block(Toggleable::Openning)
            ))
        {
            return lang_err!(Expected {
                after: if self.analyser.range().is_empty() {
                    LangModuleSlice::new_with(self.module(), self.module().tail_range())
                } else {
                    self.analyser_slice()
                },
                expected: Some("'(' or '@scope ('".to_string()),
                but_found: None
            });
        }

        let analysing_token = self.analyser.get()[0].clone();
        let scope = if let Tokens::ScopeTarget(_) = analysing_token.kind() {
            let target = ScopeTarget::parse(self, None)?.expect_definition()?.clone();

            if let Some(slice) = self.analyser.lookup(0, 1) {
                if !matches!(slice[0].kind(), Tokens::Block(Toggleable::Openning)) {
                    return lang_err!(Expected {
                        after: target.location().clone(),
                        expected: Some("'('".to_string()),
                        but_found: Some(slice[0].location().code().to_string())
                    });
                }
                self.analyser.next(0, 1);
            } else {
                return lang_err!(Expected {
                    after: LangModuleSlice::new_with(self.module(), self.module().tail_range()),
                    expected: Some("'('".to_string()),
                    but_found: None
                });
            };

            Some(target)
        } else {
            None
        };

        let openning_at = self.analyser.range().start;
        self.analyser.next(0, 0);

        let branches = self.branches(
            |p, t| {
                if let Some(f) = &force_stop {
                    f(p, t)
                } else {
                    matches!(t.kind(), Tokens::Block(Toggleable::Closing))
                }
            },
            |p, t| {
                if let Some(f) = &splitted_by {
                    f(p, t)
                } else {
                    matches!(t.kind(), Tokens::ArgSeparator)
                }
            },
            persistant_behaviors.unwrap_or(Some(vec![])),
        )?;
        self.analyser.set(openning_at..self.analyser.range().end);

        Ok((scope, branches))
    }

    /// Parse multiple instructions, and ends if the `force_stop` function returns `true` or the analyser stream is finished.
    ///
    /// # Panics
    /// - If the length of the analyser, when call this method, is > 1.
    ///
    /// # Parameters
    /// Note: to marke a function as "not given", simply type it as `|_, _| false`
    ///
    /// ## `force_stop`
    /// A function that takes the parser and the current analysing token and should return a boolean.
    /// If this boolean is `true`, then the branch does not analyse/parse the token and return the
    /// already parsed ones.
    ///
    /// If this parameter is not given, this parses till the end of the stream.
    ///
    /// ## `splitted_by`
    /// The exact same principle than the `force_stop` parameter.
    /// But this time, if the function returns `true`, the token is skipped and the next instructions
    /// are placed in a new array of nodes.
    ///
    /// If this paramter is not given (or the splitting condition is never filled),
    /// you'll receive 1 array of branches as a `Ok()` result.
    ///
    /// ## `persistant_behaviors`
    /// If you want some parser's behaviors to not being deleted for each instructions, set them in this vector.
    ///
    /// # Analyser side-effects
    /// - The analyser is set to the token that has terminated the method processus (or to empty if the processus is finished.)
    ///
    /// # Behaviors
    /// - For each instructions, the parser behavior's is reset to the persistant ones (or empty).
    /// - After branches are analysed, it is reset to one before the method has been called
    fn branches(
        &mut self,
        force_stop: impl Fn(&Self, &Token) -> bool,
        splitted_by: impl Fn(&Self, &Token) -> bool,
        persistant_behaviors: Option<Vec<ParserBehaviors>>,
    ) -> LangResult<Vec<Branches>> {
        assert!(self.analyser.range().len() <= 1);
        let mut result = vec![];
        let mut analysing = vec![];

        let reset = self.behaviors.clone();
        let behaviors = HashSet::from_iter(persistant_behaviors.unwrap_or_default());

        while self.analyser.min_len(1) {
            self.behaviors = behaviors.clone();

            let token = &self.analyser.get()[0];
            if force_stop(self, token) {
                break;
            };

            if splitted_by(self, token) {
                result.push(take(&mut analysing));
                self.analyser.next(0, 0);
                continue;
            };

            if !matches!(token.kind(), Tokens::EndOfInstruction) {
                analysing.push(Instructions::parse(self, None)?);
            }

            self.analyser.next(0, 0);
        }
        self.behaviors = reset;
        result.push(take(&mut analysing));

        Ok(result)
    }

    /// Execute the parser and return the vector of instructions
    pub fn parse(&mut self) -> &Branches {
        let branches = self.branches(
            |state, _| state.analyser.process_finished(),
            |_, _| false,
            None,
        );
        self.parsed = branches.unwrap_or_else(|e| e.raise()).pop().unwrap();

        &self.parsed
    }
    /// Clear the parsed instructions and rebuild it
    pub fn reparse(&mut self) -> &Branches {
        self.parsed = vec![];
        self.analyser.set(0..0);

        self.parse()
    }
}

impl From<&mut Lexer> for Parser {
    fn from(value: &mut Lexer) -> Self {
        let stream = value.lexify().to_vec();
        Self::new(value.module(), stream)
    }
}
