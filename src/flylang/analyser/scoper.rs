use std::collections::{HashMap, hash_map::Values};

use crate::flylang::parser::ast::{
    definables::Definables, expressions::modified::ModifiedDefinable,
};

#[derive(Clone, Debug)]
pub enum Storable {
    Raw(Definables),
    Modifed(ModifiedDefinable),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScopeIdentifier {
    Id(usize),
    Name(String),
}

#[derive(Clone, Debug, Default)]
pub struct Scoper {
    scopes: HashMap<ScopeIdentifier, Vec<Storable>>,
    scope_order: Vec<ScopeIdentifier>,
}

impl Scoper {
    pub fn enter(&mut self, identifier: Option<String>) -> ScopeIdentifier {
        let scope_id = if let Some(name) = identifier {
            ScopeIdentifier::Name(name)
        } else {
            ScopeIdentifier::Id(self.scopes.len())
        };

        self.scope_order.push(scope_id.clone());
        self.scopes.insert(scope_id.clone(), vec![]);

        scope_id
    }

    pub fn quit(&mut self, target: Option<&ScopeIdentifier>) {
        let mut amount: usize = 1;

        if let Some(scope) = target {
            assert!(self.scopes.contains_key(scope));
            amount =
                self.scope_order.len() - self.scope_order.iter().position(|v| v == scope).unwrap();
        }

        for _ in 0..amount {
            if let Some(id) = self.scope_order.pop() {
                self.scopes.remove(&id);
            } else {
                break;
            }
        }
    }

    pub fn exists(&self, scope: &ScopeIdentifier) -> bool {
        self.scopes.contains_key(scope)
    }

    pub fn store(&mut self, value: Storable) -> &mut Self {
        if self.scopes.is_empty() {
            self.enter(None);
        }

        self.scopes
            .get_mut(self.scope_order.last().unwrap())
            .unwrap()
            .push(value);

        self
    }

    pub fn get_stored(
        &self,
    ) -> std::iter::Flatten<Values<'_, ScopeIdentifier, std::vec::Vec<Storable>>> {
        self.scopes.values().flatten()
    }
}
