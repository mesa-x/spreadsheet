use nom::{
    branch::alt,
    // tuple,
    // delimited,
    // one_of,
    bytes::complete::{is_a, tag, take},
    character::complete::{alpha1, alphanumeric0},
    error::{ErrorKind},
    multi::{many0, many1},
    sequence::{delimited, tuple}, // sequence::tuple
    Err,
    IResult,
};

fn precedence(opr: &str) -> i32 {
    match opr {
        "+" | "-" => 100,
        "*" | "/" => 200,
        "==" | ">" => 20,
        "&&" => 10,
        _ => 0,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Address {
    addr: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Range {
    upper_left: Address,
    lower_right: Address,
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
    // Expression(Box<Expression>),
    Let(String, Box<Expression>, Box<Expression>),
}

/// Join a `Vec` of `String` into a `String`
fn vec_string_to_string(v: &Vec<String>) -> String {
    let mut r2 = v.iter().fold(String::from(""), |mut sum, the_str| {
        sum.push_str(the_str);
        sum
    });
    r2
}

/// Join a `Vec` of `&str` into a `String`
fn vec_str_to_string(v: &Vec<&str>) -> String {
    let mut r2 = v.iter().fold(String::from(""), |mut sum, the_str| {
        sum.push_str(the_str);
        sum
    });
    r2
}

/// Take a function that takes a `&str` and returns an `IResult<&str, &str>`
/// and wrap it in something that returns an `IResult<&str, String>`
fn str_func_to_string_func(
    the_fn: &dyn Fn(&str) -> IResult<&str, &str>,
) -> &dyn Fn(&str) -> IResult<&str, String> {
    &(|the_str: &str| Ok((the_str, String::from(the_str))))
}

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
            x @ Err(_) => take(1u32)(s).map(|(x, res)| (x, res.to_string())),
            Ok(_) => Err(Err::Error((s, ErrorKind::Tag))),
        }
    }
}

fn parser_comment(input: &str) -> IResult<&str, String> {
    tuple((&parser_whitespaces,
    delimited(
        tag("/*"),
        many0(alt((&parser_comment, &not_str_tag("*/")))),
        tag("*/"),
    ),
    &parser_whitespaces))(input)
    .map(|(x, (_, v, _))| (x, format!("/*{}*/", vec_string_to_string(&v))))
}

#[test]
fn test_parser_comment() {
    assert_eq!(
        parser_comment("/*foo*/"), Ok(("", "/*foo*/".to_string())),
        "Looking for a valid comment"
    );
    assert_eq!(
        parser_comment("/* /* foo 32 */ */"),
        Ok(("", "/*/* foo 32 */*/".to_string()))
    );
}

// parser! {
//     fn parser_eof[I]()(I) -> &'static str
//      where [ I: Stream<Token = char>]
//      {
//          value("").and(eof()).map(|(_,_)| "")
//      }
// }

// parser! {
//     fn parser_comment_eol[I]()(I) -> String
//      where [ I: Stream<Token = char>]
//      {
//          string("//#").and(take_until(attempt(string("\r\n").or(string("\n")).or(parser_eof()).or(string("\n"))))).map(|(_, c): (&'static str, String)| c)
//      }
// }

 fn parser_whitespaces(input: &str) -> IResult<&str, String> {
     many0(is_a(" \t\n\r"))(input).map(|(x,y)| (x, vec_str_to_string(&y)))
 }

// fn parser_let(input: &str) -> IResult<&str, Expression> {
//     tuple(parser_whitespaces, tag("let"), )(input)
// }

// parser! {
//     fn parser_let[I]()(I) -> Expression
//      where [ I: Stream<Token = char>, ]
//      {
//          (parser_whitespaces(),
//          string("let"),
//          parser_identifier(),
//          parser_whitespaces(),
//          string("="),
//          expr(),
//          string(";"),
//          expr(),).map(|(_, _, id, _, _, assignment, _, main)| Expression::Let(id, Box::from(assignment), Box::from(main)))
//      }
// }

// parser! {
//     fn parser_whitespaces[I]()(I) -> ()
//      where [ I: Stream<Token = char>, ]
//      {
//          skip_many(attempt(parser_comment_eol()).or(attempt(parser_comment())).or(parser_whitespace()))
//      }
// }

// parser! {
//     fn parser_sign[I]()(I) -> Option<char>
//      where [ I: Stream<Token = char>, ]
//      {
//          optional(char('-').or(char('+')))
//      }
// }

// parser! {
//     fn parser_raw_opr[I]()(I) -> String
//     where [ I: Stream<Token = char>, ]
//     {
//         (string("&&").
//         or(string("=="))).map(|s| s.to_string())
//         .or(char('+').or(char('-')).or(char('*')).
//          or(char('>')).
//         or(char('/')).or(char('^')).or(char('&')).map(|c: char| c.to_string()))
//     }
// }

// parser! {
//     fn parser_opr[I]()(I) -> String
//     where [ I: Stream<Token = char>, ]
//     {
//         between(optional(parser_whitespaces()), optional(parser_whitespaces()), parser_raw_opr())
//     }
// }

// parser! {
//    fn parser_int[I]()(I) -> Expression
//     where [ I: Stream<Token = char>, ]
//     {
//        between(optional(parser_whitespaces()), optional(parser_whitespaces()), (parser_sign(), many1(digit())))
//        .and_then(|(sign, i): (Option<char>, String)| {
//             i.parse::<i128>().
//             map_err( StreamErrorFor::<I>::other).
//             map(|i|
//                Expression::Int(i * match sign {
//                    Some('-') => -1i128,
//                    _ => 1i128,
//                }))

//         })
//     }
// }

// parser! {
//    fn parser_float[I]()(I) -> Expression
//     where [ I: Stream<Token = char>, ]
//     {
//        between(optional(parser_whitespaces()), optional(parser_whitespaces()), (parser_sign(), (many1(digit()), char('.'), many1(digit()))))
//        .and_then(|(sign, i): (Option<char>, (String, char, String))| {
//            let first: String = sign.map(|c| c.to_string()).unwrap_or(String::from(""));
//            let front = first + &i.0 + ".";
//            let all = front + &i.2;

//             all.parse::<f64>().
//             map_err( StreamErrorFor::<I>::other).
//             map(|i|
//                Expression::Float(i))

//         })
//     }
// }
// fn opt_char_to_string(oc: Option<char>) -> String {
//     oc.map(|c| c.to_string()).unwrap_or(String::from(""))
// }

// parser! {
//    fn parser_address[I]()(I) -> Address
//     where [ I: Stream<Token = char>, ]
//     {
//        between(optional(parser_whitespaces()), optional(parser_whitespaces()), (optional(char('$')), many1(letter()), optional(char('$')), many1(digit())))
//        .map(|(ab_col, col, ab_row, row): (Option<char>, String, Option<char>, String)| {
//            Address{addr: (opt_char_to_string(ab_col) +
//            &col +
//            &opt_char_to_string(ab_row) +
//        &row).to_uppercase()}

//         })
//     }
// }

// parser! {
//    fn parser_range[I]()(I) -> Range
//     where [ I: Stream<Token = char>, ]
//     {
//        between(optional(parser_whitespaces()), optional(parser_whitespaces()), (parser_address(), char(':'), parser_address()))
//        .map(|(ul, _, lr)| {
//            Range{
//                upper_left: ul,
//                lower_right: lr
//            }

//         })
//     }
// }

// parser! {
//    fn parser_paren[I]()(I) -> Expression
//     where [ I: Stream<Token = char>, ]
//     {
//        between((parser_whitespaces(),
//         char('('),
//          parser_whitespaces()),
//          (parser_whitespaces(),
//          char(')'),
//          parser_whitespaces()), expr()).map(|e| Expression::Paren(Box::from(e)))
//     }
// }

// parser! {
//    fn parser_string[I]()(I) -> Expression
//     where [ I: Stream<Token = char>, ]
//     {
//        between(
//            (optional(parser_whitespaces()), char('"')),
//            (char('"'), optional(parser_whitespaces())),
//            many(none_of("\"".chars())),
//        )
//        .map(|s| Expression::Str(s))
//     }
// }

// parser! {
//    fn parser_comma_list[I]()(I) -> Vec<Expression>
//     where [ I: Stream<Token = char>, ]
//     {
//        sep_by(expr(), (optional(parser_whitespaces()), char(','), optional(parser_whitespaces())))
//        .map(|v: Vec<Expression>| v)
//     }
// }

// parser! {
//    fn parser_dotted_identifier[I]()(I) -> Vec<String>
//     where [ I: Stream<Token = char>, ]
//     {
//         (parser_identifier(), many1((parser_whitespaces(), char('.'),  parser_identifier(), parser_whitespaces()))).
//         map(|(id, more): (String, Vec<((), char, String, ())>)| {
//             let mut ret = Vec::new();
//             ret.push(id);
//             for x in &more {
//                 ret.push(x.2.clone())
//             }
//             ret
//         })
//     }
// }

fn parser_dotted_identifier(input: &str) -> IResult<&str, Vec<String>> {
    tuple((&parser_identifier, many1(tuple((&parser_whitespaces,  nom::character::complete::char('.'), &parser_identifier, &parser_whitespaces)))))(input).
    map(|(rest, (first, other))| {
        let mut ret = Vec::new();
        ret.push(first);
        for (_, _, x, _) in other {
            ret.push(x);
        };
        (rest, ret)
    })
}

#[test]
fn test_parser_dotted_identifier() {
    assert_eq!(
        parser_dotted_identifier("x.y"), Ok(("", vec!["x".to_string(), "y".to_string()])),
        "single letter variable"
    );

    assert_eq!(
        parser_dotted_identifier("  x  .
        
        y"), Ok(("", vec!["x".to_string(), "y".to_string()])),
        "single letter variable"
    );

    assert_eq!(
        parser_identifier("  frog32xx "),
        Ok(("", "frog32xx".to_string()))
    );

    assert_eq!(
        parser_identifier("  frog32xx
        
        
        $$$"),
        Ok(("$$$", "frog32xx".to_string()))
    );
}

fn concat_str(x: &str, y: &str) -> String {
    let mut ret = x.to_string();
    ret.push_str(y);
    ret
}

fn parser_identifier(input: &str) -> IResult<&str, String> {
    tuple((&parser_whitespaces, &alpha1, &alphanumeric0, &parser_whitespaces))(input).map(|(rest, (_, x, y, _))|
(rest, concat_str(x, y)))
}

#[test]
fn test_parser_identifier() {
    assert_eq!(
        parser_identifier("x"), Ok(("", "x".to_string())),
        "single letter variable"
    );
    assert_eq!(
        parser_identifier("  frog32xx "),
        Ok(("", "frog32xx".to_string()))
    );

    assert_eq!(
        parser_identifier("  frog32xx
        
        
        $$$"),
        Ok(("$$$", "frog32xx".to_string()))
    );
}

// parser! {
//    fn parser_function[I]()(I) -> Expression
//     where [ I: Stream<Token = char>, ]
//     {
//        (parser_identifier(), optional(between(char('['), (char(']'), parser_whitespaces()), parser_comma_list())),
//         between(char('('), (char(')'), parser_whitespaces()), parser_comma_list()))
//        .map(|(id, notations, exprs)| Expression::Function(id.to_uppercase(), notations.unwrap_or(vec![]), exprs))
//     }
// }

// parser! {
//    fn parser_opr_exp[I]()(I) -> Expression
//     where [ I: Stream<Token = char>, ]
//     {
//        (expr_mini(), parser_opr(), expr())
//        .map(|(left, opr, right)| {

//         match right {
//             Expression::Infix(o2, sub_left, sub_right) if precedence(&o2) < precedence(&opr) =>
//             Expression::Infix(o2, Box::from(Expression::Infix(opr, Box::from(left), sub_left)), sub_right),
//             _ => Expression::Infix(opr, Box::from(left), Box::from(right))
//         }
//         })
//     }
// }

// // `impl Parser` can be used to create reusable parsers with zero overhead
// fn expr_<I>() -> impl Parser<Input = I, Output = Expression>
// where
//     I: Stream<Token = char>,
//     // Necessary due to rust-lang/rust#24159
//     I::Error: ParseError<I::Token, I::Range, I::Position>,
// {
//     choice((
//         attempt(parser_let()),
//         attempt(parser_opr_exp()),
//         attempt(parser_paren()),
//         attempt(parser_dotted_identifier().map(Expression::DottedIdentifier)),
//         attempt(parser_function()),
//         attempt(parser_range().map(|r| Expression::Range(r))),
//         attempt(parser_address().map(|a| Expression::Address(a))),
//         attempt(parser_identifier().map(Expression::Identifier)),
//         attempt(parser_string()),
//         attempt(parser_float()),
//         attempt(parser_int()),
//     ))
// }

// // `impl Parser` can be used to create reusable parsers with zero overhead
// fn expr_mini<I>() -> impl Parser<Input = I, Output = Expression>
// where
//     I: Stream<Token = char>,
//     // Necessary due to rust-lang/rust#24159
//     I::Error: ParseError<I::Token, I::Range, I::Position>,
// {
//     choice((
//         attempt(parser_let()),
//         attempt(parser_paren()),
//         attempt(parser_dotted_identifier().map(Expression::DottedIdentifier)),
//         attempt(parser_function()),
//         attempt(parser_range().map(|r| Expression::Range(r))),
//         attempt(parser_address().map(|a| Expression::Address(a))),
//         attempt(parser_identifier().map(Expression::Identifier)),
//         attempt(parser_string()),
//         attempt(parser_float()),
//         attempt(parser_int()),
//     ))
// }

// parser! {
//     fn expr[I]()(I) -> Expression
//     where [I: Stream<Token = char>,
//     // Necessary due to rust-lang/rust#24159
//     I::Error: ParseError<I::Token, I::Range, I::Position>]
//     {
//         expr_()
//     }
// }

// parser! {
//     pub fn whole_expr[I]()(I) -> Expression
//     where [I: Stream<Token = char>,
//     // Necessary due to rust-lang/rust#24159
//     I::Error: ParseError<I::Token, I::Range, I::Position>]
//     {
//         (optional(string("=")), expr(), eof()).map(|(_, e, _)| e)
//     }
// }

fn main() {
    println!("Hello, world!");
}
