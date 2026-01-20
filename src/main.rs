use flylang::LangRunner;

fn main() {
    flylang::utils::env::extend_env();

    let runner = LangRunner::create();
}
