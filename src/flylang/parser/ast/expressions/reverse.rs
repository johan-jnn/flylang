use crate::flylang::parser::ast::{BoxedNode, expressions::Expressions};

#[derive(Debug, Clone)]
pub enum ReverseKind {
    Sign,
    Boolean,
}
#[derive(Debug, Clone)]
pub struct Reverse {
    pub kind: ReverseKind,
    pub expression: BoxedNode<Expressions>,
}
