use crate::{
    flylang::{
        errors::{LangResult, RaisableErr, lang_err},
        lexer::{
            errors::{InvalidScopeEnding, UnclosedScope, UnexpectedCharacter, UnknownCharacter},
            ranges::CharacterRange,
            tokens::{
                Literals, ScopeTarget, Toggleable, Token, Tokens,
                representations::number::NumberRepresentation,
            },
        },
        module::{LangModule, char::LangModuleChar, slice::LangModuleSlice},
        parser::errors::Expected,
        utils::{analyser::Analyser, scoper::Scope},
    },
    utils::macros::empty_result,
};
use std::{cell::RefCell, mem::take, num::NonZero, rc::Rc};

pub mod errors;
pub mod ranges;
pub mod tokens;

/// Convert a module to a list of tokens
#[derive(Debug)]
pub struct Lexer {
    module: Rc<LangModule>,
    scope: Vec<Scope<LangModuleSlice>>,
    analyser: Analyser<LangModuleChar>,
    lexified: Vec<Token<Tokens>>,
}

impl Lexer {
    pub fn new(module: &Rc<LangModule>) -> Self {
        Self {
            module: Rc::clone(module),
            scope: vec![],
            analyser: Analyser::new(module.chars().collect()),
            lexified: vec![],
        }
    }
    /// Get the lexer's module
    pub fn module(&self) -> &Rc<LangModule> {
        &self.module
    }

    fn get_slice(&self) -> LangModuleSlice {
        LangModuleSlice::from(&self.analyser.get().to_vec())
    }

    /// Validate the current slice (`get_slice` method) with the given token.
    /// This inserts the created token to the lexified vector.
    fn validate_analyser(&mut self, kind: Tokens) {
        self.lexified.push(Token::new(kind, &self.get_slice()));

        self.analyser.next(0, 0);
    }

    /// Check the current slice, and validate it as a keyword, a defined literal (true/false/...) or a word if nothing found.
    /// Note: the word is the default value. Sending "01" to this method result to the word "01" (which is not valid).
    fn validate_word(&mut self) {
        assert!(
            !self.analyser.range().is_empty(),
            "The analyser is empty : cannot validate as a keyword/word."
        );

        self.validate_analyser(match self.get_slice().code() {
            "fn" => Tokens::Keyword(tokens::Keywords::Fn),
            "cs" => Tokens::Keyword(tokens::Keywords::Cs),
            "kind" => Tokens::Keyword(tokens::Keywords::Kind),
            "new" => Tokens::Keyword(tokens::Keywords::New),

            "while" => Tokens::Keyword(tokens::Keywords::While),
            "until" => Tokens::Keyword(tokens::Keywords::Until),
            "each" => Tokens::Keyword(tokens::Keywords::Each),

            "return" => Tokens::Keyword(tokens::Keywords::Return),
            "stop" => Tokens::Keyword(tokens::Keywords::Stop),
            "pass" => Tokens::Keyword(tokens::Keywords::Pass),

            "if" => Tokens::Keyword(tokens::Keywords::If),
            "else" => Tokens::Keyword(tokens::Keywords::Else),

            "true" => Tokens::Literal(tokens::Literals::True),
            "false" => Tokens::Literal(tokens::Literals::False),

            "use" => Tokens::Keyword(tokens::Keywords::Use),
            "in" => Tokens::Keyword(tokens::Keywords::In),

            _ => Tokens::Literal(tokens::Literals::Word),
        });
    }

    /// Parse the current slice as any literal.
    fn literal(&mut self) -> LangResult<()> {
        if !self.analyser.min_len(1) {
            return empty_result::ok!();
        }
        assert!(
            self.analyser.range().len() == 1,
            "Analyser must have a length of 1."
        );

        let character = self.analyser.get()[0].clone();
        match character.code() {
            c if ranges::in_ranges!(ranges::DECIMAL_RANGES, c) => self.number(),
            '"' | '\'' => self.string(),
            code => {
                if ranges::in_ranges!(ranges::VARIABLE_CHARACTER_RANGES, code)
                    && !ranges::in_ranges!(ranges::BANNED_FIRST_VARIABLE_CHARACTER_RANGES, code)
                {
                    while let Some(slice) = self.analyser.lookup(0, 1) {
                        if !ranges::in_ranges!(ranges::VARIABLE_CHARACTER_RANGES, slice[0].code()) {
                            break;
                        }

                        self.analyser.increase(1);
                    }

                    self.validate_word();
                    empty_result::ok!()
                } else {
                    lang_err!(UnknownCharacter(self.get_slice().into()))
                }
            }
        }
    }

    /// Assert that we're in a string expression. The analyser must be placed on the openning characters
    /// Parse the expression inside and return it.
    /// At the end, the cursor is placed in the closing expression character.
    fn string_expression(&mut self) -> LangResult<Vec<Token<Tokens>>> {
        let openner = self.get_slice();

        assert!(
            openner.code() == "&(",
            "The analyser is not placed on openning string expression characters."
        );

        // Split the `lexified` vector as [lexified, expression] after processing block.
        let split_at = self.lexified.len();
        let outer_scope = self.scope.len();
        self.scope.push(Scope::Block(openner.clone()));
        self.analyser.next(0, 0);

        while self.scope.len() > outer_scope {
            if self.analyser.process_finished() {
                return lang_err!(UnclosedScope(self.scope.pop().unwrap()));
            }

            self.process().unwrap_or_else(|e| e.controlled_raise());
        }

        // Because the expression has been saved to the global lexified array,
        // we take them, and set the added expression as the string's expression,
        // and we place the other as the lexified expressions
        let (lexified, expression_slice) = self.lexified.split_at(split_at);
        let mut expression = Vec::from(expression_slice);
        self.lexified = Vec::from(lexified);

        // Remove the closing character and set the cursor on it
        self.analyser
            .set(expression.pop().unwrap().location().range());

        Ok(expression)
    }

    /// Parse the current value as a string
    fn string(&mut self) -> LangResult<()> {
        self.analyser.min_len(1);
        assert!(
            self.analyser.range().len() == 1,
            "Expected the analyser to have a range length of 1."
        );

        let toggler = self.analyser.get()[0].clone();
        if !matches!(toggler.code(), '\'' | '"') {
            return lang_err!(UnexpectedCharacter(
                LangModuleSlice::from(&toggler).into(),
                Some("['\"]")
            ));
        }
        self.scope.push(Scope::String(self.get_slice()));
        self.analyser.next(0, 0);

        let allow_expressions = toggler.code() == '"';
        let mut content = vec![];
        let mut literal_value = String::from("");

        macro_rules! validate_literal_if_non_empty {
            () => {
                if !literal_value.is_empty() {
                    content.push(Token::new(
                        tokens::StringItem::Literal(take(&mut literal_value)),
                        &self.get_slice(),
                    ))
                }
            };
        }

        loop {
            if !self.analyser.able_to_increase(1) {
                return lang_err!(UnclosedScope(self.scope.pop().unwrap()));
            }
            self.analyser.increase(1);

            let analysing = self.analyser.get().last().unwrap();
            match analysing.code() {
                c if c == toggler.code() => {
                    let scope = self.scope.pop().unwrap();
                    if !matches!(scope, Scope::String(_)) {
                        return lang_err!(InvalidScopeEnding(
                            LangModuleSlice::from(analysing).into(),
                            scope
                        ));
                    }

                    // Go back to avoid taking the closing character
                    self.analyser
                        .set(self.analyser.range().start..self.analyser.range().end - 1);
                    validate_literal_if_non_empty!();

                    // Then re-include the closing character
                    self.analyser.increase(1);
                    break;
                }
                c if c == '&' && allow_expressions => {
                    let entering_expr: bool = if let Some(slice) = self.analyser.lookup(0, 1) {
                        slice[0].code() == '('
                    } else {
                        false
                    };

                    if !(entering_expr) {
                        literal_value += "&";
                        continue;
                    }

                    // Remove the openning expression characters to validate the current string
                    self.analyser
                        .set(self.analyser.range().start..self.analyser.range().end - 1);
                    validate_literal_if_non_empty!();
                    // Place the analyser on the openning characters
                    self.analyser.next(0, 2);
                    let expression_start = self.get_slice().range().start;
                    let expression = self.string_expression()?;
                    let expression_end = self.get_slice().range().end;

                    self.analyser.next(0, 0);
                    let mut slice = LangModuleSlice::new(self.module());
                    slice.set(expression_start..expression_end);

                    content.push(Token::new(
                        tokens::StringItem::Expression(Box::new(expression)),
                        &slice,
                    ));
                }
                '\\' => {
                    if !self.analyser.able_to_increase(1) {
                        return lang_err!(UnclosedScope(self.scope.pop().unwrap()));
                    }
                    self.analyser.increase(1);

                    /*
                        ? https://stackoverflow.com/a/1367339/15234457

                        \t Insert a tab in the text at this point.
                        \b Insert a backspace in the text at this point.
                        \n Insert a newline in the text at this point.
                        \r Insert a carriage return in the text at this point.
                        \f Insert a formfeed in the text at this point.
                        \s Insert a space in the text at this point.
                        \' Insert a single quote character in the text at this point.
                        \" Insert a double quote character in the text at this point.
                        \\ Insert a backslash character in the text at this point.
                    */
                    literal_value += &(match self.analyser.get().last().unwrap().code() {
                        't' => '\t',
                        'b' => r"\b".chars().next().unwrap(),
                        'n' => '\n',
                        'r' => '\r',
                        'f' => r"\f".chars().next().unwrap(),
                        's' => r"\s".chars().next().unwrap(),

                        c => c,
                    })
                    .to_string()
                }
                c => {
                    literal_value += &c.to_string();
                }
            }
        }
        // Recalibrate the analyser to match the whole string
        self.analyser
            .set(toggler.index()..self.analyser.range().end);
        self.validate_analyser(Tokens::Literal(tokens::Literals::String(content)));

        empty_result::ok!()
    }

    /// Parse the current slice as a number
    fn number(&mut self) -> LangResult<()> {
        let _start = self.analyser.range().start;
        self.analyser.set(_start.._start);

        let ambigus_as_num = match self.lexified.last() {
            Some(t) => matches!(
                t.kind(),
                Tokens::ArgSeparator
                    | Tokens::Operator(tokens::Operator::Substract)
                    | Tokens::BinaryOperator(_)
                    | Tokens::Comparison(_)
                    | Tokens::Block(Toggleable::Openning)
                    | Tokens::VarDef(_)
            ),
            None => true,
        };
        let mut allow_float = true;
        let mut allow_base_change = true;

        let numeric_ranges: RefCell<&CharacterRange> = RefCell::new(ranges::DECIMAL_RANGES);
        let is_numeric = |c| ranges::in_ranges!(numeric_ranges.borrow(), c);

        while let Some(next) = self.analyser.lookup(0, 1) {
            let modchar = next[0].clone();
            let mut must_followed_by_numeric = false;

            match modchar.code() {
                '.' => {
                    if !allow_float {
                        break;
                    }
                    if !ambigus_as_num {
                        if self.analyser.range().is_empty() {
                            return lang_err!(UnexpectedCharacter(
                                LangModuleSlice::from(&modchar).into(),
                                Some("[0-9_]")
                            ));
                        }
                        break;
                    }

                    allow_float = false;
                    must_followed_by_numeric = true;
                }
                '_' => {
                    if self.analyser.range().is_empty() {
                        return lang_err!(UnexpectedCharacter(
                            LangModuleSlice::from(&modchar).into(),
                            Some("[0-9.]")
                        ));
                    }
                }
                '-' => {
                    if !self.analyser.range().is_empty() {
                        break;
                    }

                    if !ambigus_as_num {
                        return lang_err!(UnexpectedCharacter(
                            LangModuleSlice::from(&modchar).into(),
                            Some("[0-9]")
                        ));
                    }

                    must_followed_by_numeric = true;
                }
                code => {
                    // Base changeing
                    if allow_base_change && matches!(code, 'b' | 'x') {
                        if self.analyser.range().is_empty()
                            || !self.get_slice().code().replace(['0', '-'], "").is_empty()
                        {
                            return lang_err!(UnexpectedCharacter(
                                LangModuleSlice::from(&modchar).into(),
                                Some("[0-9._-]")
                            ));
                        }

                        allow_base_change = false;
                        allow_float = false;
                        must_followed_by_numeric = true;

                        numeric_ranges.replace(match modchar.code() {
                            'b' => ranges::BINARY_RANGES,
                            'x' => ranges::HEXADECIMAL_RANGES,
                            _ => &numeric_ranges.borrow(),
                        });
                    } else if code.is_whitespace()
                        || ranges::in_ranges!(ranges::PONCTUATION_RANGES, code)
                    {
                        break;
                    } else if !is_numeric(code) {
                        return lang_err!(UnexpectedCharacter(
                            LangModuleSlice::from(&modchar).into(),
                            Some("[0-9._]")
                        ));
                    }
                }
            };

            if must_followed_by_numeric {
                if let Some(next) = self.analyser.lookup(1, 1) {
                    if is_numeric(next[0].code()) {
                        self.analyser.increase(1);
                    } else {
                        return lang_err!(UnexpectedCharacter(
                            LangModuleSlice::from(&next.to_vec()).into(),
                            Some("[0-9]")
                        ));
                    }
                } else {
                    return lang_err!(UnexpectedCharacter(
                        LangModuleSlice::from(&modchar).into(),
                        Some(".[0-9]")
                    ));
                }
            }
            self.analyser.increase(1);
        }
        self.validate_analyser(Tokens::Literal(tokens::Literals::Number));

        empty_result::ok!()
    }

    fn process(&mut self) -> LangResult<()> {
        if !self.analyser.min_len(1) {
            return Ok(());
        }

        // Skipping useless whitespace
        if self.analyser.range().len() == 1 && self.analyser.get()[0].code().is_whitespace() {
            self.analyser.next(0, 0);
            return self.process();
        }

        let slice = self.get_slice();
        match slice.code() {
            "!" => {
                self.validate_analyser(Tokens::Not);
            }
            "(" => {
                self.scope.push(Scope::Block(self.get_slice()));
                self.validate_analyser(Tokens::Block(Toggleable::Openning));
            }
            ")" => {
                let current_scope = self.scope.pop();
                if let Some(scope) = current_scope {
                    if scope.is(&Scope::Block(())) {
                        self.validate_analyser(Tokens::Block(Toggleable::Closing));
                    } else {
                        return lang_err!(InvalidScopeEnding(self.get_slice().into(), scope));
                    }
                } else {
                    return lang_err!(UnexpectedCharacter(self.get_slice().into(), None));
                }
            }
            "{" => {
                self.scope.push(Scope::Object(self.get_slice()));
                self.validate_analyser(Tokens::Object(Toggleable::Openning));
            }
            "}" => {
                let current_scope = self.scope.pop();
                if let Some(scope) = current_scope {
                    if scope.is(&Scope::Object(())) {
                        self.validate_analyser(Tokens::Object(Toggleable::Closing));
                    } else {
                        return lang_err!(InvalidScopeEnding(self.get_slice().into(), scope));
                    }
                } else {
                    return lang_err!(UnexpectedCharacter(self.get_slice().into(), None));
                }
            }
            "." => {
                let snapshot = self.analyser.range();
                if self.number().is_err() {
                    self.analyser.set(snapshot);
                    self.validate_analyser(Tokens::Accessor);
                }
            }
            "-" => {
                let snapshot = self.analyser.range();
                if self.number().is_err() {
                    self.analyser.set(snapshot);
                    self.validate_analyser(Tokens::Operator(tokens::Operator::Substract));
                }
            }
            "+" => {
                self.validate_analyser(Tokens::Operator(tokens::Operator::Add));
            }
            "*" => {
                let mut operator = tokens::Operator::Multiply;

                if let Some(slice) = self.analyser.lookup(0, 1)
                    && slice[0].code() == '*'
                {
                    self.analyser.increase(1);
                    operator = tokens::Operator::Power;
                }

                self.validate_analyser(Tokens::Operator(operator));
            }
            "/" => {
                let mut operator = tokens::Operator::Divide;

                if let Some(slice) = self.analyser.lookup(0, 1)
                    && slice[0].code() == '/'
                {
                    self.analyser.increase(1);
                    operator = tokens::Operator::EuclidianDivision;
                }

                self.validate_analyser(Tokens::Operator(operator));
            }
            "@" => {
                // @<varname-like>
                // or
                // @<+
                // or
                // @-<number>
                let mut amount: Option<NonZero<usize>> = None;
                while let Some(slice) = self.analyser.lookup(0, 1) {
                    if slice[0].code() == '<' {
                        self.analyser.increase(1);

                        amount = Some(if let Some(val) = amount {
                            val.checked_add(1).expect("Overflow error")
                        } else {
                            NonZero::new(1).unwrap()
                        })
                    } else {
                        break;
                    }
                }

                if let Some(amount) = amount {
                    self.validate_analyser(Tokens::ScopeTarget(ScopeTarget::Numbered(amount)));
                } else {
                    let expectation = String::from("multiple '<', an integer < 0 or a word");

                    // handle as a variable
                    if !self.analyser.able_to(0, 1) {
                        return lang_err!(Expected {
                            after: slice,
                            expected: Some(expectation),
                            but_found: None
                        });
                    }
                    self.analyser.next(0, 1);

                    self.literal()?;
                    let lexified = self.lexified.pop().unwrap();
                    self.analyser
                        .set(slice.range().start..lexified.location().range().end);

                    self.validate_analyser(match lexified.kind() {
                        Tokens::Literal(Literals::Word | Literals::True | Literals::False)
                        | Tokens::Keyword(_) => Tokens::ScopeTarget(ScopeTarget::Named(
                            lexified.location().code().to_string(),
                        )),
                        Tokens::Literal(Literals::Number) => {
                            let num = NumberRepresentation::from(lexified.location());

                            if num.negative || num.decimal.is_some() || num.integer == 0 {
                                return lang_err!(Expected {
                                    after: slice,
                                    expected: Some(expectation),
                                    but_found: Some(String::from(
                                        "a negative and/or decimal-based number"
                                    ))
                                });
                            };

                            Tokens::ScopeTarget(ScopeTarget::Numbered(
                                NonZero::new(num.integer as usize).unwrap(),
                            ))
                        }
                        _ => {
                            return lang_err!(Expected {
                                after: slice,
                                expected: Some(expectation),
                                but_found: Some(lexified.location().code().to_string())
                            });
                        }
                    });
                }
            }
            "%" => {
                self.validate_analyser(Tokens::Operator(tokens::Operator::Modulo));
            }
            ";" => {
                if let Some(last) = self.lexified.last()
                    && let Tokens::EndOfInstruction = last.kind()
                {
                    // Here we prevent following end of instruction (useless)
                    self.analyser.next(0, 0);
                }
                // Empty only if the analyser has been nexted (see above).
                if !self.analyser.range().is_empty() {
                    self.validate_analyser(Tokens::EndOfInstruction);
                }
            }
            "," => {
                self.validate_analyser(Tokens::ArgSeparator);
            }
            "#" => {
                self.validate_analyser(Tokens::Modifier);
            }
            ":" => {
                let constant = match self.analyser.lookup(0, 1) {
                    Some(kind) => matches!(kind[0].code(), ':'),
                    None => false,
                };
                if constant {
                    self.analyser.increase(1);
                }

                if let Some(previous) = self.lexified.last()
                    && let Tokens::Operator(operator) = previous.kind()
                {
                    if constant {
                        return lang_err!(UnexpectedCharacter(
                            self.get_slice().into(),
                            Some("<operator>: ...")
                        ));
                    }
                    let kind = Tokens::VarDef(tokens::VarDefinition::WithOperation(Token::new(
                        operator.clone(),
                        previous.location(),
                    )));

                    // Set the reference to also take the operator
                    self.analyser.set(
                        LangModuleSlice::from(&vec![previous.location().clone(), self.get_slice()])
                            .range(),
                    );

                    // Remove the operation token
                    self.lexified.pop();
                    self.validate_analyser(kind);

                    return empty_result::ok!();
                }

                self.validate_analyser(Tokens::VarDef(if constant {
                    tokens::VarDefinition::Constant
                } else {
                    tokens::VarDefinition::Normal
                }));
            }
            "&" => {
                self.validate_analyser(Tokens::BinaryOperator(tokens::BinaryOperator::And));
            }
            "?" => {
                self.validate_analyser(Tokens::BinaryOperator(tokens::BinaryOperator::Or));
            }
            "~" => {
                self.validate_analyser(Tokens::BinaryOperator(tokens::BinaryOperator::Xor));
            }

            "=" => {
                self.validate_analyser(Tokens::Comparison(tokens::Comparison::Equal));
            }
            "<" | ">" => {
                let mut strict = true;
                if let Some(next) = self.analyser.lookup(0, 1)
                    && next[0].code() == '='
                {
                    strict = false;
                    self.analyser.increase(1);
                }

                self.validate_analyser(Tokens::Comparison(match slice.code() {
                    ">" => tokens::Comparison::Greater(strict),
                    "<" => tokens::Comparison::Less(strict),
                    // Will never happen.
                    _ => panic!(),
                }));
            }
            "|" => {
                let mut stopper = '|';

                while let Some(slice) = self.analyser.lookup(0, 1) {
                    let modchar = &slice[0];
                    if modchar.code() == stopper {
                        if self.get_slice().range().len() == 1 {
                            stopper = '\n';
                        } else {
                            self.analyser.increase(1);
                            break;
                        }
                    }

                    self.analyser.increase(1);
                }

                // We do not add a comment as a token
                self.analyser.next(0, 0);
            }
            _ => return self.literal(),
        };

        empty_result::ok!()
    }

    /// Execute the lexer if needed and return the vector of tokens
    pub fn lexify(&mut self) -> &Vec<Token> {
        while !self.analyser.process_finished() {
            self.process().unwrap_or_else(|e| e.controlled_raise());
        }

        if !self.scope.is_empty() {
            UnclosedScope(self.scope.pop().unwrap()).controlled_raise();
        }

        &self.lexified
    }

    /// Clear the lexified tokens and rebuild it
    pub fn relexify(&mut self) -> &Vec<Token> {
        self.lexified = vec![];
        self.analyser.set(0..0);

        self.lexify()
    }
}
