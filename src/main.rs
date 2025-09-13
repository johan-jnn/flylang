use std::cell::RefCell;

use flylang::{flylang as lang, tests};

fn main() {
    let mut parser = lang::FlyLang::parser(lang::FlyLang::path(String::from("tests/misc.fly")));
    dbg!(&parser.parse());
}
