#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParserBehaviors {
    /// If the node should parse it-self as lazy.
    /// Bellow is a table of the "evaluated-as" expression in function of the `lazy` value
    ///
    /// expression | lazy=`true` | lazy=`false`                
    /// ---|---|---
    /// `foo() + 1;` | `foo()` | `foo() + 1;`
    /// `(foo() + 1);` | `foo() + 1` | `foo() + 1;`
    /// `foo.props;` | `foo` | `foo.props;`
    /// `(foo.props)();` | `foo.props` | `(foo.props)();`
    Lazy,
    /// If included, the variable emplacement's expressions are not checked.
    /// If the value of the variant is `true`, it allows this rule recursivly (allowing syntax like `a(): b(): true`)
    AllowAnyVariableEmplacement(bool),
}
