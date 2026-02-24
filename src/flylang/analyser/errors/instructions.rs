use crate::flylang::{
    errors::{ErrorType, RaisableErr},
    parser::ast::{Node, definables::Definables, instructions::r#use::Package},
};

pub struct PackageNotFound(pub Node<Package>);
impl RaisableErr for PackageNotFound {
    fn _code(&self) -> i32 {
        1
    }
    fn _kind(&self) -> crate::flylang::errors::ErrorType {
        ErrorType::Stop
    }
}
