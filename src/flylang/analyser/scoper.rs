use std::{
    collections::{HashMap, binary_heap::Iter, hash_map::Values},
    iter::Map,
};

use crate::flylang::parser::ast::{
    Node, definables::Definables, expressions::modified::ModifiedDefinable,
};

#[derive(Clone, Debug)]
pub enum Storable {
    Raw(Node<Definables>),
    Modifed(Node<ModifiedDefinable>),
}

impl Storable {
    pub fn defined(&self) -> &Node<Definables> {
        match self {
            Self::Raw(defined) => defined,
            Self::Modifed(modified) => &modified.kind().definable,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScopeIdentifier {
    Id(usize),
    Name(String),
}

#[derive(Debug, Clone)]
pub enum ScopeKind {
    Module,
    Condition,
    Loop,
    Function,
    Method,
}

#[derive(Debug, Clone)]
pub struct ScopeInfo {
    pub kind: ScopeKind,
    pub identifier: ScopeIdentifier,
    pub content: Vec<Storable>,
}

#[derive(Clone, Debug, Default)]
pub struct Scoper {
    scopes: Vec<ScopeInfo>,
}

impl Scoper {
    fn enter_module_if_empty(&mut self) {
        if self.scopes.is_empty() {
            self.enter(ScopeKind::Module, None);
        }
    }

    pub fn enter(&mut self, kind: ScopeKind, identifier: Option<String>) -> ScopeIdentifier {
        let scope_id = if let Some(name) = identifier {
            ScopeIdentifier::Name(name)
        } else {
            ScopeIdentifier::Id(self.scopes.len())
        };

        self.scopes.push(ScopeInfo {
            kind,
            identifier: scope_id.clone(),
            content: vec![],
        });

        scope_id
    }

    pub fn quit(&mut self, target: Option<&ScopeIdentifier>) {
        let mut amount: usize = 1;

        if let Some(scope) = target {
            assert!(self.exists(scope));
            amount = self.scopes.len()
                - self
                    .scopes
                    .iter()
                    .position(|s| &s.identifier == scope)
                    .unwrap();
        }

        self.quit_n(amount);
    }

    pub fn quit_n(&mut self, amount: usize) {
        for _ in 0..amount {
            if self.scopes.pop().is_none() {
                break;
            }
        }
    }

    pub fn exists(&self, scope: &ScopeIdentifier) -> bool {
        self.scopes.iter().any(|s| &s.identifier == scope)
    }

    pub fn store(&mut self, value: Storable) -> &mut Self {
        self.enter_module_if_empty();

        self.scopes.last_mut().unwrap().content.push(value);
        self
    }

    pub fn get_stored(&self) -> impl Iterator<Item = &Storable> {
        self.scopes.iter().flat_map(|info| &info.content)
    }

    pub fn current(&mut self) -> &ScopeInfo {
        self.enter_module_if_empty();

        self.try_current().unwrap()
    }
    pub fn try_current(&self) -> Option<&ScopeInfo> {
        self.scopes.last()
    }
}
