use crate::flylang::{
    errors::{ErrorType, RaisableErr},
    parser::ast::{Node, definables::Definables},
};

pub struct AlreadyDefined {
    pub defined_node: Node<Definables>,
    pub defining_node: Node<Definables>,
}
impl RaisableErr for AlreadyDefined {
    fn _code(&self) -> i32 {
        1
    }
    fn _kind(&self) -> crate::flylang::errors::ErrorType {
        ErrorType::Stop
    }
}
