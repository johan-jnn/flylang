use std::path::Path;

use clap::Parser;

use crate::{behavior::LangBehavior, cli::LangCLI};

pub mod behavior;
pub mod cli;
pub mod flylang;
pub mod utils;

#[derive(Debug, Clone)]
pub struct LangRunner {
    pub cli: LangCLI,
    pub behavior: LangBehavior,
}

impl LangRunner {
    pub fn create() -> Self {
        let cli = LangCLI::parse();
        let base_behavior_file = cli.behavior_file.unwrap_or("./flylang.toml".into());

        Self {
            cli: LangCLI::parse(),
            behavior: LangBehavior::new_parsed(Path::new(&base_behavior_file)),
        }
    }
}
