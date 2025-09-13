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
    /// ## `lazy`
    /// If the node should parse it-self as lazy.
    /// Bellow is a table of the "evaluated-as" expression in function of the `lazy` value
    ///
    /// expression | lazy=`true` | lazy=`false`                
    /// ---|---|---
    /// `foo() + 1;` | `foo()` | `foo() + 1;`
    /// `(foo() + 1);` | `foo() + 1` | `foo() + 1;`
    /// `foo.props;` | `foo` | `foo.props;`
    /// `(foo.props)();` | `foo.props` | `(foo.props)();`
    ///
    /// # Analyser side-effects
    /// - The analyser is set to the whole parsed node's range (**!! : This is totally false for now**)
    fn parse(
        parser: &mut Parser,
        previous: Option<Node>,
        lazy: bool,
    ) -> LangResult<Node<Self::ResultKind>>;
}
