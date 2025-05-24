use crate::env::Env;
use crate::utils;
use crate::val::Val;

use super::func_call::FuncCall;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BindingUsage {
    pub(crate) name: String,
}

impl BindingUsage {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = utils::extract_ident(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
            },
        ))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        env.get_binding(&self.name).or_else(|error_msg| {
            if env.get_func(&self.name).is_ok() {
                FuncCall {
                    callee: self.name.clone(),
                    params: Vec::new(),
                }
                .eval(env)
            } else {
                Err(error_msg)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        binding_def::BindingDef,
        expr::{Expr, Number},
        statement::Statement,
    };

    use super::*;

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Statement::new("let a = 10"),
            Ok((
                "",
                Statement::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expr::Number(Number(10)),
                }),
            )),
        );
    }

    #[test]
    fn eval_existing_binding_usage() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(10));

        assert_eq!(
            BindingUsage {
                name: "foo".to_string(),
            }
            .eval(&env),
            Ok(Val::Number(10)),
        );
    }

    #[test]
    fn eval_non_existent_binding_usage() {
        let empty_env = Env::default();

        assert_eq!(
            BindingUsage {
                name: "i_dont_exist".to_string(),
            }
            .eval(&empty_env),
            Err("binding with name ‘i_dont_exist’ does not exist".to_string()),
        );
    }

    #[test]
    fn eval_binding_usage() {
        let mut env = Env::default();
        env.store_binding("ten".to_string(), Val::Number(10));

        assert_eq!(
            Expr::BindingUsage(BindingUsage {
                name: "ten".to_string(),
            })
            .eval(&env),
            Ok(Val::Number(10)),
        );
    }
}
