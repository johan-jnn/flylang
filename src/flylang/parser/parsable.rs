use crate::flylang::{
    errors::LangResult,
    parser::{Parser, ast::Node},
};

pub trait Parsable {
    type ResultKind;

    /// Parse the current analyser's range as the object.
    ///
    /// # Parameter
    ///
    /// ## `parser`
    /// The mutable parser
    ///
    /// ## `previous`
    /// The previous just parsed node
    ///
    /// # Analyser side-effects
    /// - The analyser is set to the whole parsed node's range (**!! : This is totally false for now**)
    ///
    /// # Behaviors
    /// - The parser's behaviors are not reset.
    fn parse(parser: &mut Parser, previous: Option<Node>) -> LangResult<Node<Self::ResultKind>>;
}
