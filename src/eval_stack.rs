use crate::eval::{eval, Value};
use crate::parser::{whole_expr_str, Expression};

#[derive(Debug, PartialEq, Clone)]
pub enum EvalStack {
    PushInt(i128),
    PushFloat(f64),
    PushStr(String),
    PerformOpr(String),
}

pub enum BuilderParams {}

type BuildResult = Result<Vec<EvalStack>, String>;

pub fn create_eval_stack(expr: &Expression, params: &Vec<BuilderParams>) -> BuildResult {
    let mut to_populate: Vec<EvalStack> = vec![];

    match do_create_eval_stack(expr, params, &mut to_populate) {
        Ok(_) => Ok(to_populate),
        Err(bad) => Err(bad),
    }
}

fn do_create_eval_stack(
    expr: &Expression,
    params: &Vec<BuilderParams>,
    to_populate: &mut Vec<EvalStack>,
) -> Result<(), String> {
    match expr {
        Expression::Int(i, _) => to_populate.push(EvalStack::PushInt(*i)),
        Expression::Float(f, _) => to_populate.push(EvalStack::PushFloat(*f)),
        Expression::Str(string, _) => to_populate.push(EvalStack::PushStr(string.clone())),

        // DottedIdentifier(Vec<String>, ParseInfo),
        // Identifier(String, ParseInfo),
        Expression::Paren(expr, _) => match do_create_eval_stack(expr, params, to_populate) {
            Ok(_) => (),
            Err(e) => return Err(e),
        },
        // Address(Address, ParseInfo),
        // Range(Range, ParseInfo),
        // Function(String, Vec<Expression>, Vec<Expression>, ParseInfo),
        Expression::Infix(opr, left, right, _) => {
            match do_create_eval_stack(left, params, to_populate)
                .and_then(|_| do_create_eval_stack(right, params, to_populate))
                .and_then(|_| {
                    to_populate.push(EvalStack::PerformOpr(opr.clone()));
                    Ok(())
                }) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        // Let(String, Box<Expression>, Box<Expression>, ParseInfo),
        _ => return Err(format!("Failed {:?}", expr)),
    }

    Ok(())
}

#[test]
fn test_create_stack() {
    let ex = whole_expr_str("(20 * 2) + 1 + 1 * \"dogs\" ").unwrap();
    let res = create_eval_stack(&ex, &vec![]);
    println!("Res is {:?}", res);
    assert!(res.is_ok());
    let computed = eval(&res.unwrap());
    assert_eq!(computed, Ok(Value::Int(42)))
}
