use crate::parser::Expression;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum EvalStack {
    PushInt(i128),
    PushFloat(f64),
    PushStr(String),
    PerformOpr(String),
}

pub enum BuilderParams {
    Choice(bool), // A boolean true/false choice
    Other(String), // Some other structure expressed as a string -- complex stuff maybe as JSON?
    OtherMap(HashMap<String, String>) // an "Other" expressed as a map
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Types {
    Int, Float, Str
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct OperatorInfo {
    operator: String,
    num_type_param: u16,
    type_param_type: HashMap<u16, Types>,
    num_params: u16,
    param_type: HashMap<u16, Vec<Types>>,


}

type BuildResult = Result<Vec<EvalStack>, String>;

pub fn create_eval_stack(expr: &Expression, params: &HashMap<String, BuilderParams>) -> BuildResult {
    let mut to_populate: Vec<EvalStack> = vec![];

    match do_create_eval_stack(expr, params, &mut to_populate) {
        Ok(_) => Ok(to_populate),
        Err(bad) => Err(bad),
    }
}

fn do_create_eval_stack(
    expr: &Expression,
    params: &HashMap<String, BuilderParams>,
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
    use crate::eval::{eval, Value};
    use crate::parser::whole_expr_str;
    let ex = whole_expr_str("(20 * 2) + 1 + 1 ").unwrap();
    let res = create_eval_stack(&ex, &HashMap::new());
    println!("Res is {:?}", res);
    assert!(res.is_ok());
    let computed = eval(&res.unwrap());
    assert_eq!(computed, Ok(Value::Int(42)))
}
