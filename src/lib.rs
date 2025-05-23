#![allow(dead_code)]

mod binding_def;
mod expr;
mod statement;
mod val;

mod env;
mod utils;

// pub struct Parse(statement::Statement);

// pub fn parse(s: &str) -> Result<Parse, String> {
//     let (s, stmt) = statement::Statement::new(s)?;

//     if s.is_empty() {
//         Ok(Parse(stmt))
//     } else {
//         Err("input was not consumed fully by parser".to_string())
//     }
// }
