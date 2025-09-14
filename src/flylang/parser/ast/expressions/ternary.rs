use crate::flylang::parser::ast::{BoxedNode, expressions::Expressions};

#[derive(Debug, Clone)]
// Note: the ternary expressions are parsed in the conditionnal instruction.
pub struct Ternary {
    pub condition: BoxedNode<Expressions>,
    pub yes: BoxedNode<Expressions>,
    pub no: BoxedNode<Expressions>,
}
