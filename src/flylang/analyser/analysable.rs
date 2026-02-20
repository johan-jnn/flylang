use crate::flylang::{analyser::LangAnalyser, errors::LangResult};

pub trait Analysable {
    /// Analyse the current object and check for issues.
    ///
    /// # Parameter
    ///
    /// ## `analyser`
    /// The mutable analyser
    fn analyse<'a>(&self, analyser: &mut LangAnalyser<'a>) -> LangResult<()>;
}
