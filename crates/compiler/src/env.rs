use std::collections::HashMap;

use crate::{statement::Statement, val::Val};

#[derive(Debug, PartialEq, Default)]
pub struct Env<'parent> {
    named: HashMap<String, NamedInfo>,
    parent: Option<&'parent Self>,
}

impl<'parent> Env<'parent> {
    pub(crate) fn create_child(&'parent self) -> Self {
        Self {
            named: HashMap::new(),
            parent: Some(self),
        }
    }

    pub(crate) fn store_binding(&mut self, name: String, val: Val) {
        self.named.insert(name, NamedInfo::Binding(val));
    }

    pub(crate) fn store_func(&mut self, name: String, params: Vec<String>, body: Statement) {
        self.named.insert(name, NamedInfo::Func { params, body });
    }

    pub(crate) fn get_binding(&self, name: &str) -> Result<Val, String> {
        self.get_named_info(name)
            .and_then(NamedInfo::into_binding)
            .ok_or_else(|| format!("binding with name ‘{}’ does not exist", name))
    }

    pub(crate) fn get_func(&self, name: &str) -> Result<(Vec<String>, Statement), String> {
        self.get_named_info(name)
            .and_then(NamedInfo::into_func)
            .ok_or_else(|| format!("function with name ‘{}’ does not exist", name))
    }

    fn get_named_info(&self, name: &str) -> Option<NamedInfo> {
        self.named
            .get(name)
            .cloned()
            .or_else(|| self.parent.and_then(|parent| parent.get_named_info(name)))
    }
}

#[derive(Debug, PartialEq, Clone)]
enum NamedInfo {
    Binding(Val),
    Func {
        params: Vec<String>,
        body: Statement,
    },
}

impl NamedInfo {
    fn into_binding(self) -> Option<Val> {
        if let Self::Binding(val) = self {
            Some(val)
        } else {
            None
        }
    }

    fn into_func(self) -> Option<(Vec<String>, Statement)> {
        if let Self::Func { params, body } = self {
            Some((params, body))
        } else {
            None
        }
    }
}
