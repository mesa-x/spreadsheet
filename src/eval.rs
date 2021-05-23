use crate::definitions::Value;
use crate::eval_stack::EvalStack;

pub fn eval(instructions: &Vec<EvalStack>) -> Result<Value, String> {
    let mut stack: Vec<Value> = vec![];

    for inst in instructions {
        match inst {
            EvalStack::PushInt(i) => stack.push(Value::Int(*i)),
            EvalStack::PushFloat(f) => stack.push(Value::Float(*f)),
            EvalStack::PushStr(s) => stack.push(Value::Str(s.clone())),
            EvalStack::PerformOpr(opr) => match perform_opr(opr, &mut stack) {
                Err(e) => return Err(e),
                _ => (),
            },
        }
    }

    if stack.len() == 1 {
        return Ok(
            stack.pop().unwrap(), /* unwrap ok here because length test */
        );
    };
    Err(format!("Could not eval... stack ended at {:?}", &stack))
}

fn perform_opr(opr: &str, stack: &mut Vec<Value>) -> Result<(), String> {
    match opr {
        "+" => match (stack.pop(), stack.pop()) {
            (Some(Value::Int(i1)), Some(Value::Int(i2))) => {
                stack.push(Value::Int(i1 + i2));
                Ok(())
            }
            (Some(Value::Float(f1)), Some(Value::Float(f2))) => {
                stack.push(Value::Float(f1 + f2));
                Ok(())
            }
            (i1, i2) => Err(format!(
                "Failed operator '+' for stack items {:?} and {:?}",
                i1, i2
            )),
        },
        "/" => match (stack.pop(), stack.pop()) {
            (Some(Value::Int(i1)), Some(Value::Int(i2))) => {
                stack.push(Value::Int(i1 / i2));
                Ok(())
            }
            (Some(Value::Float(f1)), Some(Value::Float(f2))) => {
                stack.push(Value::Float(f1 / f2));
                Ok(())
            }
            (i1, i2) => Err(format!(
                "Failed operator '/' for stack items {:?} and {:?}",
                i1, i2
            )),
        },
        "*" => match (stack.pop(), stack.pop()) {
            (Some(Value::Int(i1)), Some(Value::Int(i2))) => {
                stack.push(Value::Int(i1 * i2));
                Ok(())
            }
            (Some(Value::Float(f1)), Some(Value::Float(f2))) => {
                stack.push(Value::Float(f1 * f2));
                Ok(())
            }
            (i1, i2) => Err(format!(
                "Failed operator '*' for stack items {:?} and {:?}",
                i1, i2
            )),
        },
        _ => Err(format!("Could not find operator {}", opr)),
    }
}
