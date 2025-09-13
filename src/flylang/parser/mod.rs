use std::{mem::take, ops::Range, rc::Rc, vec};

use crate::flylang::{
    errors::{LangResult, RaisableErr, lang_err},
    lexer::{
        Lexer,
        tokens::{self, Token, Tokens},
    },
    module::{LangModule, slice::LangModuleSlice},
    parser::{
        ast::{Branches, Node, instructions::Instructions},
        errors::EmptyScope,
        parsable::Parsable,
    },
    utils::analyser::Analyser,
};

pub mod ast;
pub mod errors;
pub mod parsable;

#[derive(Debug)]
pub struct Parser {
    module: Rc<LangModule>,
    analyser: Analyser<Token<Tokens>>,
    parsed: Branches,
}

impl Parser {
    /// Create a new `Parser`
    /// Warning: the given stream must be valid : the openning/closing scopes will not be verified.
    pub fn new(module: &Rc<LangModule>, stream: Vec<Token<Tokens>>) -> Self {
        Self {
            module: Rc::clone(module),
            analyser: Analyser::new(stream),
            parsed: vec![],
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
    /// # Analyser side-effects
    /// - The analyser is set to the token that has terminated the method processus (or to empty if the processus is finished.)
    fn branches(
        &mut self,
        force_stop: impl Fn(&Self, &Token) -> bool,
        splitted_by: impl Fn(&Self, &Token) -> bool,
    ) -> LangResult<Vec<Branches>> {
        assert!(self.analyser.range().len() <= 1);
        let mut result = vec![];
        let mut analysing = vec![];

        while self.analyser.min_len(1) {
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
                analysing.push(Instructions::parse(self, None, false)?);
            }

            self.analyser.next(0, 0);
        }
        result.push(take(&mut analysing));

        Ok(result)
    }

    /// Execute the parser and return the vector of instructions
    pub fn parse(&mut self) -> &Branches {
        let branches = self.branches(|state, _| state.analyser.process_finished(), |_, _| false);
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
