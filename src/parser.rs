use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take, take_till},
    character::complete::{alpha1, char, digit1},
    combinator::opt,
    error::ErrorKind,
    error::ParseError,
    multi::{many0, many1, separated_list0},
    sequence::{delimited, tuple}, // sequence::tuple
    AsChar,
    Err,
    IResult,
    InputTakeAtPosition,
};

use crate::util::*;
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Address {
    pub addr: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Range {
    pub upper_left: Address,
    pub lower_right: Address,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PositionInfo {
    pub start: usize,
    pub start_line: u32,
    pub end: usize,
    pub end_line: u32,
    pub text: String,
}

pub type ParseInfo = Option<Box<PositionInfo>>;

fn parse_info(start: &Span, end: &Span) -> ParseInfo {
    let inner = PositionInfo {
        start: start.location_offset(),
        start_line: start.location_line(),
        end: end.location_offset(),
        end_line: end.location_line(),
        text: start.fragment()[..(end.location_offset() - start.location_offset())].to_string(),
    };
    Some(Box::from(inner))
}

#[derive(Debug, Clone)]
pub enum Expression {
    Int(i128, ParseInfo),
    Float(f64, ParseInfo),
    Str(String, ParseInfo),
    DottedIdentifier(Vec<String>, ParseInfo),
    Identifier(String, ParseInfo),
    Paren(Box<Expression>, ParseInfo),
    Address(Address, ParseInfo),
    Range(Range, ParseInfo),
    Function(String, Vec<Expression>, Vec<Expression>, ParseInfo),
    Infix(String, Box<Expression>, Box<Expression>, ParseInfo),
    Let(String, Box<Expression>, Box<Expression>, ParseInfo),
}

impl PartialEq for Expression {
    fn eq(self: &Expression, other: &Expression) -> bool {
        match (self, other) {
            (Expression::Int(x, _), Expression::Int(y, _)) if x == y => true,
            (Expression::Float(x, _), Expression::Float(y, _)) if x == y => true,
            (Expression::Str(x, _), Expression::Str(y, _)) if x == y => true,
            (Expression::DottedIdentifier(x, _), Expression::DottedIdentifier(y, _)) if x == y => {
                true
            }
            (Expression::Paren(x, _), Expression::Paren(y, _)) if x == y => true,
            (Expression::Identifier(x, _), Expression::Identifier(y, _)) if x == y => true,
            (Expression::Address(x, _), Expression::Address(y, _)) if x == y => true,
            (Expression::Range(x, _), Expression::Range(y, _)) if x == y => true,
            (Expression::Function(x1, x2, x3, _), Expression::Function(y1, y2, y3, _))
                if x1 == y1 && x2 == y2 && x3 == y3 =>
            {
                true
            }
            (Expression::Infix(x1, x2, x3, _), Expression::Infix(y1, y2, y3, _))
                if x1 == y1 && x2 == y2 && x3 == y3 =>
            {
                true
            }
            (Expression::Let(x1, x2, x3, _), Expression::Let(y1, y2, y3, _))
                if x1 == y1 && x2 == y2 && x3 == y3 =>
            {
                true
            }

            _ => false,
        }
    }
}

/// return a function that matches a tag, but returns an `IResult<&str, String>`
fn str_tag<'a>(to_match: &str) -> impl Fn(Span<'a>) -> IResult<Span<'a>, String> {
    let the_str = to_match.to_string();

    move |s: Span| {
        let s2: &str = &the_str;
        tag(s2)(s).map(|(x, y)| (x, y.to_string()))
    }
}

fn not_str_tag(to_match: &str) -> impl Fn(Span) -> IResult<Span, String> {
    let the_str = to_match.to_string();

    move |s: Span| {
        let s2: &str = &the_str;
        match str_tag(s2)(s) {
            Err(_) => take(1u32)(s).map(|(x, res)| (x, res.to_string())),
            Ok(_) => Err(Err::Error(nom::error::Error::new(s, ErrorKind::Tag))),
        }
    }
}

fn parser_comment(input: Span) -> IResult<Span, String> {
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

fn parser_comment_as_str(input: Span) -> IResult<Span, Span> {
    parser_comment(input).map(|(x, _)| (x, Span::new("")))
}

#[test]
fn test_parser_comment() {
    assert_eq!(
        parser_comment(Span::new("/*foo*/")).map(|(_, y)| y),
        Ok("/*foo*/".to_string()),
        "Looking for a valid comment"
    );
    assert_eq!(
        parser_comment(Span::new("/* /* foo 32 */ */")).map(|(_, y)| y),
        Ok("/*/* foo 32 */*/".to_string())
    );
}

/// Parse a single line comment
fn parser_comment_eol(input: Span) -> IResult<Span, Span> {
    tuple((
        tag("//#"),
        take_till(|c| c == '\r' || c == '\n'),
        &parser_whitespaces,
    ))(input)
    .map(|(rest, _)| (rest, Span::new("")))
}

/// Join a `Vec` of `&str` into a `String`
fn vec_span_to_string(v: &Vec<Span>) -> String {
    let r2 = v.iter().fold(String::from(""), |mut sum, the_str| {
        sum.push_str(the_str.fragment());
        sum
    });
    r2
}

fn parser_whitespaces(input: Span) -> IResult<Span, String> {
    many0(is_a(" \t\n\r"))(input).map(|(x, y)| (x, vec_span_to_string(&y)))
}

fn parser_comment_whitespaces(input: Span) -> IResult<Span, ()> {
    opt(many0(alt((
        is_a(" \t\n\r"),
        &parser_comment_as_str,
        &parser_comment_eol,
    ))))(input)
    .map(|(x, _)| (x, ()))
}

fn parser_let(input: Span) -> IResult<Span, Expression> {
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
            Expression::Let(
                id.to_uppercase(),
                Box::from(e1),
                Box::from(e2),
                parse_info(&input, &rest),
            ),
        )
    })
}

fn parser_raw_opr(input: Span) -> IResult<Span, Span> {
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

fn parser_opr(input: Span) -> IResult<Span, Span> {
    tuple((
        &parser_comment_whitespaces,
        &parser_raw_opr,
        &parser_comment_whitespaces,
    ))(input)
    .map(|(rest, (_, o, _))| (rest, o))
}

fn parser_sign(input: Span) -> IResult<Span, Option<Span>> {
    opt(alt((tag("+"), tag("-"))))(input)
}

fn parser_int(input: Span) -> IResult<Span, Expression> {
    match tuple((
        opt(&parser_comment_whitespaces),
        &parser_sign,
        digit1,
        opt(&parser_comment_whitespaces),
    ))(input)
    {
        Ok((rest, (_, sign, i, _))) => {
            let sign_mult: i128 = match sign {
                Some(zz) if zz == Span::new("-") => -1i128,
                _ => 1i128,
            };

            match i.fragment().parse::<i128>() {
                Ok(i2) => Ok((
                    rest,
                    Expression::Int(i2 * sign_mult, parse_info(&input, &rest)),
                )),
                Result::Err(_) => {
                    Result::Err(Err::Error(nom::error::Error::new(input, ErrorKind::Digit)))
                }
            }
        }
        Err(x) => Err(x),
    }
}

fn parser_float(input: Span) -> IResult<Span, Expression> {
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
        let front = first + sig.fragment() + ".";
        let all = front + fr.fragment();

        match all.parse::<f64>() {
            Ok(i2) => Ok((rest, Expression::Float(i2, parse_info(&input, &rest)))),
            Result::Err(_) => {
                Result::Err(Err::Error(nom::error::Error::new(input, ErrorKind::Digit)))
            }
        }
    })
}

fn opt_char_to_string(oc: Option<char>) -> String {
    oc.map(|c| c.to_string()).unwrap_or(String::from(""))
}

fn parser_address_addr(input: Span) -> IResult<Span, Address> {
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
                addr: (opt_char_to_string(ab_col)
                    + col.fragment()
                    + &opt_char_to_string(ab_row)
                    + row.fragment())
                .to_uppercase(),
            },
        )
    })
}

fn parser_address(input: Span) -> IResult<Span, Expression> {
    parser_address_addr(input)
        .map(|(rest, a)| (rest, Expression::Address(a, parse_info(&input, &rest))))
}

fn parser_range(input: Span) -> IResult<Span, Expression> {
    tuple((&parser_address_addr, tag(":"), &parser_address_addr))(input).map(
        |(rest, (ul, _, lr))| {
            (
                rest,
                Expression::Range(
                    Range {
                        upper_left: ul,
                        lower_right: lr,
                    },
                    parse_info(&input, &rest),
                ),
            )
        },
    )
}

fn parser_paren(input: Span) -> IResult<Span, Expression> {
    tuple((
        &parser_comment_whitespaces,
        char('('),
        &parser_comment_whitespaces,
        &expr,
        &parser_comment_whitespaces,
        char(')'),
        &parser_comment_whitespaces,
    ))(input)
    .map(|(rest, (_, _, _, r, _, _, _))| {
        (
            rest,
            Expression::Paren(Box::from(r), parse_info(&input, &rest)),
        )
    })
}

fn parser_string(input: Span) -> IResult<Span, Expression> {
    delimited(
        tuple((opt(&parser_comment_whitespaces), tag("\""))),
        many0(is_not("\"")),
        tuple((tag("\""), opt(&parser_comment_whitespaces))),
    )(input)
    .map(|(rest, v)| {
        (
            rest,
            Expression::Str(vec_span_to_string(&v), parse_info(&input, &rest)),
        )
    })
}

fn parser_comma_list(input: Span) -> IResult<Span, Vec<Expression>> {
    separated_list0(tag(","), &expr)(input)
}

fn parser_dotted_identifier(input: Span) -> IResult<Span, Expression> {
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
        (
            rest,
            Expression::DottedIdentifier(ret, parse_info(&input, &rest)),
        )
    })
}

#[test]
fn test_parser_dotted_identifier() {
    assert_eq!(
        parser_dotted_identifier(Span::new("x.y")).map(|(_, y)| y),
        Ok(Expression::DottedIdentifier(
            vec!["X".to_string(), "Y".to_string()],
            None
        )),
        "single letter variable"
    );

    assert_eq!(
        parser_dotted_identifier(Span::new(
            "  x  .
        
        y"
        ))
        .map(|(_, y)| y),
        Ok(Expression::DottedIdentifier(
            vec!["X".to_string(), "Y".to_string()],
            None
        )),
        "single letter variable"
    );

    assert_eq!(
        parser_dotted_identifier(Span::new("  frog32xx. moose ")).map(|(_, y)| y),
        Ok(Expression::DottedIdentifier(
            vec!["FROG32XX".to_string(), "MOOSE".to_string()],
            None
        ))
    );

    assert_eq!(
        parser_dotted_identifier(Span::new(
            "  frog32xx /*
        
        a comment */
        .cat
        $$$"
        ))
        .map(|(x, y)| if x.fragment() == &"$$$" {
            y
        } else {
            Expression::DottedIdentifier(vec!["didn't slurp $$$".to_string()], None)
        }),
        Ok(Expression::DottedIdentifier(
            vec!["FROG32XX".to_string(), "CAT".to_string()],
            None
        ))
    );
}

fn concat_str(x: &str, y: &str) -> String {
    x.to_string() + y
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

fn parser_identifier_string(input: Span) -> IResult<Span, String> {
    tuple((
        &parser_comment_whitespaces,
        &alpha1,
        &alphanumeric_or_underscore0,
        &parser_comment_whitespaces,
    ))(input)
    .map(|(rest, (_, x, y, _))| (rest, concat_str(x.fragment(), y.fragment())))
}

fn parser_identifier(input: Span) -> IResult<Span, Expression> {
    parser_identifier_string(input).map(|(rest, x)| {
        (
            rest,
            Expression::Identifier(x.to_uppercase(), parse_info(&input, &rest)),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser_util::ex_id;
    #[test]
    fn test_parser_identifier() {
        assert_eq!(
            parser_identifier(Span::new("x")).map(|(_, y)| y),
            Ok(ex_id("x")),
            "single letter variable"
        );

        assert_eq!(
            parser_identifier(Span::new("  frog32xx ")).map(|(_, y)| y),
            Ok(ex_id("FROG32XX"))
        );

        assert_eq!(
            parser_identifier(Span::new(
                "  frog32xx
        
        
        $$$"
            ))
            .map(|(_, y)| y),
            Ok(ex_id("FROG32XX"))
        );
    }
}

fn parser_function(input: Span) -> IResult<Span, Expression> {
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
                parse_info(&input, &rest),
            ),
        )
    })
}

fn parser_opr_exp(input: Span) -> IResult<Span, Expression> {
    tuple((&expr_mini, parser_opr, &expr))(input).map(|(rest, (left, oprs, right))| {
        let opr = oprs.to_string();
        let rexp = match right {
            Expression::Infix(o2, sub_left, sub_right, _) if precedence(&o2) < precedence(&opr) => {
                Expression::Infix(
                    o2,
                    Box::from(Expression::Infix(opr, Box::from(left), sub_left, None)), // FIXME -- can we get the parse info for the span?
                    sub_right,
                    parse_info(&input, &rest),
                )
            }
            _ => Expression::Infix(
                opr,
                Box::from(left),
                Box::from(right),
                parse_info(&input, &rest),
            ),
        };
        (rest, rexp)
    })
}

fn expr_mini(input: Span) -> IResult<Span, Expression> {
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

fn expr(input: Span) -> IResult<Span, Expression> {
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

// pub fn tvs(input: Vec<&str>) -> Vec<String> {
//     input.iter().map(|s| s.to_string().to_uppercase()).collect()
// }

#[test]
fn test_expr() {
    use crate::parser_util::{ex_dot, ex_fun, ex_id};

    assert_eq!(
        expr(Span::new(" foo /* cat */ ")).map(|(_, y)| y),
        Ok(ex_id("FOO"))
    );

    assert_eq!(
        expr(Span::new(
            " cat(dog, 
            
            moose.cat, /*mo
            
            
            oo*/ rat.s.s) /* meow */"
        ))
        .map(|(_, y)| y),
        Ok(ex_fun(
            "cat",
            vec![],
            vec![
                ex_id("DOG"),
                ex_dot(vec!["moose", "cat"]),
                ex_dot(vec!["rat", "s", "s"])
            ]
        ))
    );
}

pub fn whole_expr_str<'a>(input: &'a str) -> Result<Expression, nom::Err<nom::error::Error<Span>>> {
    whole_expr(Span::new(input))
}

pub fn whole_expr(input: Span) -> Result<Expression, nom::Err<nom::error::Error<Span>>> {
    match tuple((opt(tag("=")), &expr))(input) {
        Ok((zz, (_, e))) if zz.fragment() == &"" => Ok(e),
        Ok((rest, _)) => Result::Err(nom::Err::Error(nom::error::Error::new(
            rest,
            ErrorKind::Complete,
        ))),
        Result::Err(x) => Result::Err(x), // Err(Err(_, error)) => Err(error)
    }
}
