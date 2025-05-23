#![allow(dead_code)]

mod binding_def;
mod expr;
mod statement;
mod val;

mod env;
mod utils;

pub use env::Env;
pub use val::Val;

pub struct Parse(statement::Statement);

impl Parse {
    fn eval(&self, env: &mut Env) -> Result<Val, String> {
        self.0.eval(env)
    }
}

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = statement::Statement::new(s)?;

    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        Err("Input was not consumed fully by parser".to_string())
    }
}
