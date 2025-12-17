use clap::{Subcommand, command};

#[derive(Debug, Clone, Subcommand)]
pub enum LangParserCLI {
    /// List the current available parsers
    #[command()]
    List {},

    /// Search for parsers
    #[command()]
    Search {
        /// The name of the parser your searching for
        query: String,
    },

    /// Install new parser(s)
    #[command()]
    Install {
        /// Name of parsers you want to install
        names: Vec<String>,
    },

    /// Remove installed parser
    #[command()]
    Remove {
        /// Name of parsers you want to remove
        names: Vec<String>,
    },
}
