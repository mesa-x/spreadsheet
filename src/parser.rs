use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take, take_till},
    character::complete::{alpha1, char, digit1},
    combinator::opt,
    error::ErrorKind,
    error::ParseError,
    multi::{many0, many1, separated_list},
    sequence::{delimited, tuple}, // sequence::tuple
    AsChar,
    Err,
    IResult,
    InputTakeAtPosition,
};

use crate::util::*;

fn precedence(opr: &str) -> i32 {
    match opr {
        "+" | "-" => 100,
        "*" | "/" => 200,
        "==" | ">" | "<" | ">=" | "<=" => 20,
        "&&" | "||" => 10,
        _ => 0,
    }
}

#[test]
fn test_precedence() {
    assert!(precedence("&&") < precedence("+"))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Address {
    pub addr: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Range {
    pub upper_left: Address,
    pub lower_right: Address,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Int(i128),
    Float(f64),
    Str(String),
    DottedIdentifier(Vec<String>),
    Identifier(String),
    Paren(Box<Expression>),
    Address(Address),
    Range(Range),
    Function(String, Vec<Expression>, Vec<Expression>),
    Infix(String, Box<Expression>, Box<Expression>),
    Let(String, Box<Expression>, Box<Expression>),
}



/// return a function that matches a tag, but returns an `IResult<&str, String>`
fn str_tag(to_match: &str) -> impl Fn(&str) -> IResult<&str, String> {
    let the_str = to_match.to_string();

    move |s: &str| {
        let s2: &str = &the_str;
        tag(s2)(s).map(|(x, y)| (x, y.to_string()))
    }
}

fn not_str_tag(to_match: &str) -> impl Fn(&str) -> IResult<&str, String> {
    let the_str = to_match.to_string();

    move |s: &str| {
        let s2: &str = &the_str;
        match str_tag(s2)(s) {
            Err(_) => take(1u32)(s).map(|(x, res)| (x, res.to_string())),
            Ok(_) => Err(Err::Error((s, ErrorKind::Tag))),
        }
    }
}

fn parser_comment(input: &str) -> IResult<&str, String> {
    tuple((
        &parser_whitespaces,
        delimited(
            tag("/*"),
            many0(alt((&parser_comment, &not_str_tag("*/")))),
            tag("*/"),
        ),
        &parser_whitespaces,
    ))(input)
    .map(|(x, (_, v, _))| (x, format!("/*{}*/", vec_string_to_string(&v))))
}

fn parser_comment_as_str(input: &str) -> IResult<&str, &str> {
    parser_comment(input).map(|(x, _)| (x, ""))
}

#[test]
fn test_parser_comment() {
    assert_eq!(
        parser_comment("/*foo*/"),
        Ok(("", "/*foo*/".to_string())),
        "Looking for a valid comment"
    );
    assert_eq!(
        parser_comment("/* /* foo 32 */ */"),
        Ok(("", "/*/* foo 32 */*/".to_string()))
    );
}

/// Parse a single line comment
fn parser_comment_eol(input: &str) -> IResult<&str, &str> {
    tuple((
        tag("//#"),
        take_till(|c| c == '\r' || c == '\n'),
        &parser_whitespaces,
    ))(input)
    .map(|(rest, _)| (rest, ""))
}

fn parser_whitespaces(input: &str) -> IResult<&str, String> {
    many0(is_a(" \t\n\r"))(input).map(|(x, y)| (x, vec_str_to_string(&y)))
}

fn parser_comment_whitespaces(input: &str) -> IResult<&str, ()> {
    opt(many0(alt((
        is_a(" \t\n\r"),
        &parser_comment_as_str,
        &parser_comment_eol,
    ))))(input)
    .map(|(x, _)| (x, ()))
}

fn parser_let(input: &str) -> IResult<&str, Expression> {
    tuple((
        &parser_comment_whitespaces,
        tag("let"),
        &parser_identifier_string,
        &parser_comment_whitespaces,
        tag("="),
        &expr,
        tag(";"),
        &expr,
    ))(input)
    .map(|(rest, (_, _, id, _, _, e1, _, e2))| {
        (
            rest,
            Expression::Let(id.to_uppercase(), Box::from(e1), Box::from(e2)),
        )
    })
}

fn parser_raw_opr(input: &str) -> IResult<&str, &str> {
    alt((
        tag("&&"),
        tag("=="),
        tag("+"),
        tag("-"),
        tag("*"),
        tag(">"),
        tag("/"),
        tag("^"),
    ))(input)
}

fn parser_opr(input: &str) -> IResult<&str, &str> {
    tuple((
        &parser_comment_whitespaces,
        &parser_raw_opr,
        &parser_comment_whitespaces,
    ))(input)
    .map(|(rest, (_, o, _))| (rest, o))
}

fn parser_sign(input: &str) -> IResult<&str, Option<&str>> {
    opt(alt((tag("+"), tag("-"))))(input)
}

fn parser_int(input: &str) -> IResult<&str, Expression> {
    match tuple((
        opt(&parser_comment_whitespaces),
        &parser_sign,
        digit1,
        opt(&parser_comment_whitespaces),
    ))(input)
    {
        Ok((rest, (_, sign, i, _))) => {
            let sign_mult: i128 = match sign {
                Some("-") => -1i128,
                _ => 1i128,
            };

            match i.parse::<i128>() {
                Ok(i2) => Ok((rest, Expression::Int(i2 * sign_mult))),
                Result::Err(_) => Result::Err(Err::Error((input, ErrorKind::Digit))),
            }
        }
        Err(x) => Err(x),
    }
}

fn parser_float(input: &str) -> IResult<&str, Expression> {
    tuple((
        opt(&parser_comment_whitespaces),
        &parser_sign,
        digit1,
        tag("."),
        digit1,
        opt(&parser_comment_whitespaces),
    ))(input)
    .and_then(|(rest, (_, sign, sig, _, fr, _))| {
        let first: String = sign.map(|c| c.to_string()).unwrap_or(String::from(""));
        let front = first + sig + ".";
        let all = front + fr;

        match all.parse::<f64>() {
            Ok(i2) => Ok((rest, Expression::Float(i2))),
            Result::Err(_) => Result::Err(Err::Error((input, ErrorKind::Digit))),
        }
    })
}

fn opt_char_to_string(oc: Option<char>) -> String {
    oc.map(|c| c.to_string()).unwrap_or(String::from(""))
}

fn parser_address_addr(input: &str) -> IResult<&str, Address> {
    tuple((
        opt(&parser_comment_whitespaces),
        opt(char('$')),
        alpha1,
        opt(char('$')),
        digit1,
        opt(parser_comment_whitespaces),
    ))(input)
    .map(|(rest, (_, ab_col, col, ab_row, row, _))| {
        (
            rest,
            Address {
                addr: (opt_char_to_string(ab_col) + &col + &opt_char_to_string(ab_row) + &row)
                    .to_uppercase(),
            },
        )
    })
}

fn parser_address(input: &str) -> IResult<&str, Expression> {
    parser_address_addr(input).map(|(rest, a)| (rest, Expression::Address(a)))
}

fn parser_range(input: &str) -> IResult<&str, Expression> {
    tuple((&parser_address_addr, tag(":"), &parser_address_addr))(input).map(
        |(rest, (ul, _, lr))| {
            (
                rest,
                Expression::Range(Range {
                    upper_left: ul,
                    lower_right: lr,
                }),
            )
        },
    )
}

fn parser_paren(input: &str) -> IResult<&str, Expression> {
    tuple((
        &parser_comment_whitespaces,
        char('('),
        &parser_comment_whitespaces,
        &expr,
        &parser_comment_whitespaces,
        char(')'),
        &parser_comment_whitespaces,
    ))(input)
    .map(|(rest, (_, _, _, r, _, _, _))| (rest, Expression::Paren(Box::from(r))))
}

fn parser_string(input: &str) -> IResult<&str, Expression> {
    delimited(
        tuple((opt(&parser_comment_whitespaces), tag("\""))),
        many0(is_not("\"")),
        tuple((tag("\""), opt(&parser_comment_whitespaces))),
    )(input)
    .map(|(rest, v)| (rest, Expression::Str(vec_str_to_string(&v))))
}

fn parser_comma_list(input: &str) -> IResult<&str, Vec<Expression>> {
    separated_list(tag(","), &expr)(input)
}

fn parser_dotted_identifier(input: &str) -> IResult<&str, Expression> {
    tuple((
        &parser_identifier_string,
        many1(tuple((
            &parser_comment_whitespaces,
            nom::character::complete::char('.'),
            &parser_identifier_string,
            &parser_comment_whitespaces,
        ))),
    ))(input)
    .map(|(rest, (first, other))| {
        let mut ret: Vec<String> = Vec::new();
        ret.push(first.to_uppercase());
        for (_, _, x, _) in other {
            ret.push(x.to_uppercase());
        }
        (rest, Expression::DottedIdentifier(ret))
    })
}

#[test]
fn test_parser_dotted_identifier() {
    assert_eq!(
        parser_dotted_identifier("x.y"),
        Ok((
            "",
            Expression::DottedIdentifier(vec!["X".to_string(), "Y".to_string()])
        )),
        "single letter variable"
    );

    assert_eq!(
        parser_dotted_identifier(
            "  x  .
        
        y"
        ),
        Ok((
            "",
            Expression::DottedIdentifier(vec!["X".to_string(), "Y".to_string()])
        )),
        "single letter variable"
    );

    assert_eq!(
        parser_dotted_identifier("  frog32xx. moose "),
        Ok((
            "",
            Expression::DottedIdentifier(vec!["FROG32XX".to_string(), "MOOSE".to_string()])
        ))
    );

    assert_eq!(
        parser_dotted_identifier(
            "  frog32xx /*
        
        a comment */
        .cat
        $$$"
        ),
        Ok((
            "$$$",
            Expression::DottedIdentifier(vec!["FROG32XX".to_string(), "CAT".to_string()])
        ))
    );
}

fn concat_str(x: &str, y: &str) -> String {
    let mut ret = x.to_string();
    ret.push_str(y);
    ret
}

 fn alphanumeric_or_underscore0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position_complete(|item| {
        let ci = item.clone();
        !(item.is_alphanum() || ci.as_char() == '_')
    })
}

fn parser_identifier_string(input: &str) -> IResult<&str, String> {
    tuple((
        &parser_comment_whitespaces,
        &alpha1,
        &alphanumeric_or_underscore0,
        &parser_comment_whitespaces,
    ))(input)
    .map(|(rest, (_, x, y, _))| (rest, concat_str(x, y)))
}

fn parser_identifier(input: &str) -> IResult<&str, Expression> {
    parser_identifier_string(input)
        .map(|(rest, x)| (rest, Expression::Identifier(x.to_uppercase())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_identifier() {
        assert_eq!(
            parser_identifier("x"),
            Ok(("", Expression::Identifier("X".to_string()))),
            "single letter variable"
        );
        assert_eq!(
            parser_identifier("  frog32xx "),
            Ok(("", Expression::Identifier("FROG32XX".to_string())))
        );

        assert_eq!(
            parser_identifier(
                "  frog32xx
        
        
        $$$"
            ),
            Ok(("$$$", Expression::Identifier("FROG32XX".to_string())))
        );
    }
}

fn parser_function(input: &str) -> IResult<&str, Expression> {
    tuple((
        &parser_identifier_string,
        opt(delimited(tag("["), &parser_comma_list, tag("]"))), // FIXME whitespace
        delimited(tag("("), &parser_comma_list, tag(")")),      // FIXME whitespace
        &parser_comment_whitespaces,
    ))(input)
    .map(|(rest, (func_name, opt_type_param, params, _))| {
        (
            rest,
            Expression::Function(
                func_name.to_uppercase(),
                match opt_type_param {
                    Some(x) => x,
                    _ => vec![],
                },
                params,
            ),
        )
    })
}

fn parser_opr_exp(input: &str) -> IResult<&str, Expression> {
    tuple((&expr_mini, parser_opr, &expr))(input).map(|(rest, (left, oprs, right))| {
        let opr = oprs.to_string();
        let rexp = match right {
            Expression::Infix(o2, sub_left, sub_right) if precedence(&o2) < precedence(&opr) => {
                Expression::Infix(
                    o2,
                    Box::from(Expression::Infix(opr, Box::from(left), sub_left)),
                    sub_right,
                )
            }
            _ => Expression::Infix(opr, Box::from(left), Box::from(right)),
        };
        (rest, rexp)
    })
}

fn expr_mini(input: &str) -> IResult<&str, Expression> {
    alt((
        &parser_let,
        &parser_paren,
        &parser_dotted_identifier,
        &parser_function,
        &parser_address,
        &parser_range,
        &parser_identifier,
        &parser_string,
        &parser_float,
        &parser_int,
    ))(input)
}

fn expr(input: &str) -> IResult<&str, Expression> {
    alt((
        &parser_let,
        &parser_opr_exp,
        &parser_paren,
        &parser_dotted_identifier,
        &parser_function,
        &parser_range,
        &parser_identifier,
        &parser_address,
        &parser_string,
        &parser_float,
        &parser_int,
    ))(input)
}

pub fn ei(s: &str) -> Expression {
    Expression::Identifier(s.to_string())
}

pub fn tvs(input: Vec<&str>) -> Vec<String> {
    input.iter().map(|s| s.to_string().to_uppercase()).collect()
}

pub fn edi(vs: Vec<&str>) -> Expression {
    Expression::DottedIdentifier(tvs(vs))
}

#[test]
fn test_expr() {
    assert_eq!(expr(" foo /* cat */ "), Ok(("", ei("FOO"))));

    assert_eq!(
        expr(
            " cat(dog, 
            
            moose.cat, /*mo
            
            
            oo*/ rat.s.s) /* meow */"
        ),
        Ok((
            "",
            Expression::Function(
                "CAT".to_string(),
                vec![],
                vec![
                    ei("DOG"),
                    edi(vec!["moose", "cat"]),
                    edi(vec!["rat", "s", "s"])
                ]
            )
        ))
    );
}

pub fn whole_expr(input: &str) -> Result<Expression, nom::Err<(&str, ErrorKind)>> {
    match tuple((opt(tag("=")), &expr))(input) {
        Ok(("", (_, e))) => Ok(e),
        Ok((rest, _)) => Result::Err(nom::Err::Error((rest, ErrorKind::Complete))),
        Result::Err(x) => Result::Err(x), // Err(Err(_, error)) => Err(error)
    }
}
