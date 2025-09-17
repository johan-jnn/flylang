use std::num::NonZero;

use crate::flylang::module::slice::LangModuleSlice;
use enum_variant_type::EnumVariantType;

pub mod representations;

#[derive(Debug, Clone)]
pub enum Toggleable {
    Openning,
    Closing,
}

#[derive(Debug, Clone)]
pub enum VarDefinition {
    Normal,
    WithOperation(Token<Operator>),
    Constant,
}

#[derive(Debug, Clone)]
pub enum StringItem {
    Literal(std::string::String),
    Expression(Box<Vec<Token<Tokens>>>),
}

#[derive(Debug, Clone, EnumVariantType)]
pub enum Literals {
    #[evt(derive(Debug, Clone))]
    Word,
    #[evt(derive(Debug, Clone))]
    True,
    #[evt(derive(Debug, Clone))]
    False,
    #[evt(derive(Debug, Clone))]
    Number,
    #[evt(derive(Debug, Clone))]
    Empty,
    #[evt(derive(Debug, Clone))]
    String(Vec<Token<StringItem>>),
}

#[derive(Debug, Clone)]
pub enum Keywords {
    Fn,
    If,
    Cs,
    Else,
    While,
    Until,
    Each,
    Stop,
    Return,
    Pass,
}

#[derive(Debug, Clone)]
pub enum ScopeTarget {
    Named(std::string::String),
    Numbered(NonZero<usize>),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
    Power,
    Modulo,
    EuclidianDivision,
}

#[derive(Debug, Clone)]
// ! Order is important :
// ! Greater index will be executed first
pub enum BinaryOperator {
    And,
    Xor,
    Or,
}

#[derive(Debug, Clone)]
pub enum Comparison {
    Equal,
    /// Value is `true` if it is strict
    Less(bool),
    /// Value is `true` if it is strict
    Greater(bool),
}

#[derive(Debug, Clone)]
pub enum Tokens {
    Literal(Literals),
    Keyword(Keywords),
    Not,
    Operator(Operator),
    BinaryOperator(BinaryOperator),
    Comparison(Comparison),
    Block(Toggleable),
    Object(Toggleable),
    Accessor,
    Modifier,
    EndOfInstruction,
    ArgSeparator,
    VarDef(VarDefinition),
    ScopeTarget(ScopeTarget),
}

#[derive(Debug, Clone)]
pub struct Token<K = Tokens> {
    kind: K,
    location: LangModuleSlice,
}
impl<K> Token<K> {
    pub fn new(kind: K, location: &LangModuleSlice) -> Self {
        Self {
            kind,
            location: location.clone(),
        }
    }
    pub fn kind(&self) -> &K {
        &self.kind
    }
    pub fn location(&self) -> &LangModuleSlice {
        &self.location
    }
}
