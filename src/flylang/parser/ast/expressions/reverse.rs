use crate::flylang::{analyser::analysable::Analysable, parser::ast::{BoxedNode, Node, expressions::Expressions}};

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

impl Analysable for Node<Reverse> {
    fn analyse<'a>(
        &self,
        analyser: &mut crate::flylang::analyser::LangAnalyser,
    ) -> crate::flylang::errors::LangResult<()> {
        Ok(())
    }
}
