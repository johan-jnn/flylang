use std::path::PathBuf;

use flylang::LangRunner;

fn main() {
    flylang::utils::env::extend_env();
    let runner = LangRunner::create();

    match &runner.cli.command {
        flylang::cli::LangCommands::Exec { entrypoint, parser } => {
            let file = entrypoint.clone().expect("Default entry point not set.");

            let mut parser = flylang::flylang::FlyLang::parser(PathBuf::from(file));
            dbg!(parser.parse());
        }
        flylang::cli::LangCommands::Pkg { action } => todo!(),
        flylang::cli::LangCommands::Parser { action, directory } => todo!(),
        flylang::cli::LangCommands::Behavior {} => {
            dbg!(&runner.behavior);
            todo!()
        }
    }
}
