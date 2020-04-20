pub use crate::parser::{Address, Expression, Range};

/// Creates an Expression::Str
pub fn ex_str(s: &str) -> Expression {
    Expression::Str(s.to_string(), None)
}

/// Creates an `Expression::Identifier`
pub fn ex_id(s: &str) -> Expression {
    Expression::Identifier(s.to_uppercase(), None)
}

/// Creates an `Expression::DottedIdentifier`
pub fn ex_dot(v: Vec<&str>) -> Expression {
    Expression::DottedIdentifier(v.iter().map(|s| s.to_uppercase()).collect(), None)
}

/// Creates an `Expression::Function`
pub fn ex_fun(name: &str, p1: Vec<Expression>, p2: Vec<Expression>) -> Expression {
    Expression::Function(name.to_uppercase(), p1, p2, None)
}

/// Creates an `Expression::Address`
pub fn ex_adr(ad: &str) -> Expression {
    Expression::Address(
        Address {
            addr: ad.to_uppercase(),
        },
        None,
    )
}

/// Creates an `Expression::Paren`
pub fn ex_paren(ex: Expression) -> Expression {
    Expression::Paren(Box::from(ex), None)
}

/// Creates an `Expression::Infix`
pub fn ex_inf(opr: &str, left: Expression, right: Expression) -> Expression {
    Expression::Infix(opr.to_uppercase(), Box::from(left), Box::from(right), None)
}

/// Creates an `Expression::Float`
pub fn ex_f(f: f64) -> Expression {
    Expression::Float(f, None)
}

/// Creates an `Expression::Int`
pub fn ex_i(i: i128) -> Expression {
    Expression::Int(i, None)
}

/// Create an `Expression::Let`
pub fn ex_let(var: &str, left: Expression, right: Expression) -> Expression {
    Expression::Let(var.to_uppercase(), Box::from(left), Box::from(right), None)
}

/// Creates an `Expression::Range`
pub fn ex_rng(ul: &str, lr: &str) -> Expression {
    Expression::Range(Range {
        upper_left: Address {
            addr: ul.to_uppercase(),
        },
        lower_right: Address {
            addr: lr.to_uppercase(),
        },
    }, None)
}
