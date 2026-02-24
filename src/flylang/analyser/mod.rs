use std::rc::Rc;

use crate::{
    behavior::LangBehavior,
    flylang::{
        analyser::{analysable::Analysable, scoper::Scoper},
        module::LangModule,
        parser::{
            Parser,
            ast::{Branches, Node},
        },
    },
};

pub mod analysable;
pub mod errors;
pub mod scoper;

#[derive(Clone, Debug)]
pub struct LangAnalyser {
    module: Rc<LangModule>,
    parent: Option<Rc<LangModule>>,
    parsed: Branches,
    lang_behaviors: LangBehavior,

    pub defined: Scoper,
}

impl LangAnalyser {
    pub fn new(module: &Rc<LangModule>, parsed: Vec<Node>, behaviors: LangBehavior) -> Self {
        Self {
            module: Rc::clone(module),
            parsed,
            lang_behaviors: behaviors,
            parent: None,
            defined: Scoper::default(),
        }
    }
    pub fn get_module(&self) -> &Rc<LangModule> {
        &self.module
    }
    pub fn used_behaviors(&self) -> &LangBehavior {
        &self.lang_behaviors
    }
    pub fn is_children_of(&mut self, module: Rc<LangModule>) -> &mut Self {
        self.parent = Some(module);
        self
    }

    pub fn analyse(&mut self) -> bool {
        for node in self.parsed.clone() {
            if let Err(e) = node.analyse(self) {
                e.controlled_raise();
            }
        }

        true
    }
}

impl From<&mut Parser> for LangAnalyser {
    fn from(value: &mut Parser) -> Self {
        let parsed = value.parse().clone();
        let behaviors = value.used_behaviors().clone();

        Self::new(value.module(), parsed, behaviors)
    }
}
