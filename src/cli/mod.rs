use clap::{Parser, command};

use crate::cli::{parser::LangParserCLI, pkg::LangPkgCLI};
mod parser;
mod pkg;

#[derive(Parser, Clone, Debug)]
pub enum LangCommands {
    /// Parse a flylang code file
    #[command()]
    Exec {
        /// The .fly file to parse
        entrypoint: String,

        /// The parser's name to use.
        /// Depending on the selected parser, you may have to pass additionnal arguments.
        /// See `flylang parser list` to view the available parsers
        #[arg(short, long, default_value = "flylang-interpreter")]
        parser: Option<String>,
    },

    /// Package manager system
    #[command()]
    Pkg {
        #[command(subcommand)]
        action: LangPkgCLI,
    },

    /// Manage the flylang's parsers
    #[command()]
    Parser {
        #[command(subcommand)]
        action: LangParserCLI,

        /// Overwrite the default parsers' location
        #[arg(short, long)]
        directory: Option<String>,
    },

    /// Show the current behaviors of flylang
    #[command()]
    Behavior {},
}

/// Flylang parser.
/// Use this CLI to execute or compile flylang code.
#[derive(Debug, Clone, Parser)]
#[command(version, about, verbatim_doc_comment, author)]
pub struct LangCLI {
    /// The language's behavior file to use.
    /// Default to <entrypoint_directory>/flylang.toml (or the global's one).
    /// Must be a .toml file
    #[arg(long, short, required = false)]
    pub behavior_file: Option<String>,

    #[command(subcommand)]
    pub command: LangCommands,
}
