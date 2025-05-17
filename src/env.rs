use std::collections::HashMap;

use crate::val::Val;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Env {
    immutables: HashMap<String, Val>,
}

impl Env {
    pub(crate) fn store_immutable(&mut self, name: String, val: Val) {
        self.immutables.insert(name, val);
    }
}
