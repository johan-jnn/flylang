use crate::flylang::{analyser::analysable::Analysable, parser::ast::{BoxedNode, Node, expressions::Expressions}};

#[derive(Debug, Clone)]
// Note: the ternary expressions are parsed in the conditionnal instruction.
pub struct Ternary {
    pub condition: BoxedNode<Expressions>,
    pub yes: BoxedNode<Expressions>,
    pub no: BoxedNode<Expressions>,
}

impl Analysable for Node<Ternary> {
    fn analyse<'a>(
        &self,
        analyser: &mut crate::flylang::analyser::LangAnalyser,
    ) -> crate::flylang::errors::LangResult<()> {
        Ok(())
    }
}
