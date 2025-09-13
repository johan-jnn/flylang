use crate::flylang::{module::slice::LangModuleSlice, parser::ast::instructions::Instructions};

pub mod definables;
pub mod expressions;
pub mod instructions;

#[derive(Debug, Clone)]
pub struct Node<K = Instructions> {
    kind: K,
    location: LangModuleSlice,
}
pub type BoxedNode<K = Instructions> = Box<Node<K>>;
pub type Branches<K = Instructions> = Vec<Node<K>>;
pub type BoxedBranches<K = Instructions> = Box<Branches<K>>;
pub type BoxedNodes<K = Instructions> = Vec<BoxedNode<K>>;

impl<K> Node<K> {
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
impl<K: Clone> Node<K> {
    pub fn clone_as<Rk>(
        &self,
        with: impl Fn(K, LangModuleSlice) -> (Rk, LangModuleSlice),
    ) -> Node<Rk> {
        let (k, s) = with(self.kind.clone(), self.location.clone());
        Node::new(k, &s)
    }
}
