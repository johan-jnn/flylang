use flylang::LangRunner;
pub mod behavior;
pub mod cli;

fn main() {
    let runner = LangRunner::create();
    dbg!(runner);
}
