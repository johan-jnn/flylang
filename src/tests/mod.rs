use crate::flylang::module::LangModule;
use std::path::PathBuf;

pub fn modules() {
    let load = PathBuf::from("tests/modules/load.fly");
    let warn = PathBuf::from("tests/modules/warn.ext");

    LangModule::new(load).unwrap();
    LangModule::new(warn).unwrap();

    println!("Modules' tests passed.");
}

pub fn all() {
    modules();

    println!("All tests passed.");
}
