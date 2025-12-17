use crate::cli::LangCLI;
use clap::Parser;
pub mod behavior;
pub mod cli;

fn main() {
    let config = LangCLI::parse();
    dbg!(config);
}
