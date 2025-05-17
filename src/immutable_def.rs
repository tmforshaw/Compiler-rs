use crate::env::Env;
use crate::expr::Expr;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct ImmutableDef {
    pub name: String,
    pub val: Expr,
}

impl ImmutableDef {
    pub fn new(s: &str) -> (&str, Self) {
        let s = utils::tag("let", s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, name) = utils::extract_ident(s); // Unimplemented!
        let (s, _) = utils::extract_whitespace(s);

        let s = utils::tag("=", s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, val) = Expr::new(s);

        (
            s,
            Self {
                name: name.to_string(),
                val,
            },
        )
    }

    pub(crate) fn eval(&self, env: &mut Env) {
        env.store_immutable(self.name.clone(), self.val.eval());
    }
}

mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            ImmutableDef::new("let a = 10 / 2"),
            (
                "",
                ImmutableDef {
                    name: "a".to_string(),
                    val: Expr {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div,
                    },
                },
            ),
        );
    }
}
