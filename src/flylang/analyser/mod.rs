use std::{
    collections::{HashMap, hash_map::Values},
    iter::Flatten,
    rc::Rc,
};

use crate::{
    behavior::LangBehavior,
    flylang::{
        analyser::scoper::Scoper,
        module::LangModule,
        parser::{
            Parser,
            ast::{Branches, Node},
        },
    },
};

pub mod analysable;
pub mod scoper;

#[derive(Clone, Debug)]
pub struct LangAnalyser<'a> {
    module: Rc<LangModule>,
    parent: Option<Box<&'a Self>>,
    parsed: Branches,
    lang_behaviors: LangBehavior,

    defined: Scoper,
}

impl LangAnalyser<'_> {
    pub fn new(module: &Rc<LangModule>, parsed: Vec<Node>, behaviors: LangBehavior) -> Self {
        Self {
            module: Rc::clone(module),
            parsed,
            lang_behaviors: behaviors,
            parent: None,
            defined: Scoper::default(),
        }
    }
    pub fn get_module(&self) -> &LangModule {
        &self.module
    }
    pub fn used_behaviors(&self) -> &LangBehavior {
        &self.lang_behaviors
    }

    pub fn analyse(&mut self) {
        // for node in &self.parsed {
        //     node.kind()
        // }
    }
}

impl From<&mut Parser> for LangAnalyser<'_> {
    fn from(value: &mut Parser) -> Self {
        let parsed = value.parse().clone();
        let behaviors = value.used_behaviors().clone();

        Self::new(value.module(), parsed, behaviors)
    }
}
