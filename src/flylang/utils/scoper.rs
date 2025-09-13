use std::{
    fmt::{Debug, Display},
    mem::discriminant,
};

#[derive(Debug, Clone)]
pub enum Scope<Data> {
    Block(Data),
    String(Data),
    Object(Data),
    Module(Data),
}

impl<T> Scope<T> {
    pub fn without_data(&self) -> Scope<()> {
        match self {
            Scope::Block(_) => Scope::Block(()),
            Scope::Module(_) => Scope::Module(()),
            Scope::Object(_) => Scope::Object(()),
            Scope::String(_) => Scope::String(()),
        }
    }
    pub fn data(&self) -> &T {
        match self {
            Scope::Block(v) => v,
            Scope::Module(v) => v,
            Scope::Object(v) => v,
            Scope::String(v) => v,
        }
    }
    /// Return true if the the two scopes has the same discriminent
    pub fn is(&self, scope: &Scope<()>) -> bool {
        discriminant(&self.without_data()) == discriminant(scope)
    }
}
impl<T> Display for Scope<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scope::Block(_) => write!(f, "Block"),
            Scope::String(_) => write!(f, "String"),
            Scope::Object(_) => write!(f, "Object"),
            Scope::Module(_) => write!(f, "Module"),
        }
    }
}
