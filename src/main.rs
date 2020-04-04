#[macro_use]
extern crate combine;
use combine::error::{ParseError, StreamError};
use combine::parser::char::{alpha_num, char, digit, letter, string};
use combine::parser::repeat::take_until;
use combine::stream::{Stream, StreamErrorFor};
use combine::{
    attempt, between, choice, eof, many, many1, none_of, optional, parser, satisfy, sep_by,
    skip_many, value, Parser,
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

parser! {
    fn parser_comment[I]()(I) -> String
     where [ I: Stream<Item = char>]
     {
         between(string("/*"), string("*/"),
          take_until(attempt(string("*/"))).map(|c: String| c)
         )
     }
}

parser! {
    fn parser_eof[I]()(I) -> &'static str
     where [ I: Stream<Item = char>]
     {
         value("").and(eof()).map(|(_,_)| "")
     }
}

parser! {
    fn parser_comment_eol[I]()(I) -> String
     where [ I: Stream<Item = char>]
     {
         string("//#").and(take_until(attempt(string("\r\n").or(string("\n")).or(parser_eof()).or(string("\n"))))).map(|(_, c): (&'static str, String)| c)
     }
}

parser! {
    fn parser_whitespace[I]()(I) -> String
     where [ I: Stream<Item = char>, ]
     {
         satisfy(|c: char| c.is_whitespace() || c == '\n' || c == '\r' || c == '\t').map(|c: char| c.to_string())
     }
}

parser! {
    fn parser_let[I]()(I) -> Expression
     where [ I: Stream<Item = char>, ]
     {
         (parser_whitespaces(),
         string("let"),
         parser_identifier(),
         parser_whitespaces(),
         string("="),
         expr(),
         string(";"),
         expr(),).map(|(_, _, id, _, _, assignment, _, main)| Expression::Let(id, Box::from(assignment), Box::from(main)))
     }
}

parser! {
    fn parser_whitespaces[I]()(I) -> ()
     where [ I: Stream<Item = char>, ]
     {
         skip_many(attempt(parser_comment_eol()).or(attempt(parser_comment())).or(parser_whitespace()))
     }
}

parser! {
    fn parser_sign[I]()(I) -> Option<char>
     where [ I: Stream<Item = char>, ]
     {
         optional(char('-').or(char('+')))
     }
}

parser! {
    fn parser_raw_opr[I]()(I) -> String
    where [ I: Stream<Item = char>, ]
    {
        (string("&&").
        or(string("=="))).map(|s| s.to_string())
        .or(char('+').or(char('-')).or(char('*')).
         or(char('>')).
        or(char('/')).or(char('^')).or(char('&')).map(|c: char| c.to_string()))
    }
}

parser! {
    fn parser_opr[I]()(I) -> String
    where [ I: Stream<Item = char>, ]
    {
        between(optional(parser_whitespaces()), optional(parser_whitespaces()), parser_raw_opr())
    }
}

parser! {
   fn parser_int[I]()(I) -> Expression
    where [ I: Stream<Item = char>, ]
    {
       between(optional(parser_whitespaces()), optional(parser_whitespaces()), (parser_sign(), many1(digit())))
       .and_then(|(sign, i): (Option<char>, String)| {
            i.parse::<i128>().
            map_err( StreamErrorFor::<I>::other).
            map(|i|
               Expression::Int(i * match sign {
                   Some('-') => -1i128,
                   _ => 1i128,
               }))

        })
    }
}

parser! {
   fn parser_float[I]()(I) -> Expression
    where [ I: Stream<Item = char>, ]
    {
       between(optional(parser_whitespaces()), optional(parser_whitespaces()), (parser_sign(), (many1(digit()), char('.'), many1(digit()))))
       .and_then(|(sign, i): (Option<char>, (String, char, String))| {
           let first: String = sign.map(|c| c.to_string()).unwrap_or(String::from(""));
           let front = first + &i.0 + ".";
           let all = front + &i.2;

            all.parse::<f64>().
            map_err( StreamErrorFor::<I>::other).
            map(|i|
               Expression::Float(i))

        })
    }
}
fn opt_char_to_string(oc: Option<char>) -> String {
    oc.map(|c| c.to_string()).unwrap_or(String::from(""))
}

parser! {
   fn parser_address[I]()(I) -> Address
    where [ I: Stream<Item = char>, ]
    {
       between(optional(parser_whitespaces()), optional(parser_whitespaces()), (optional(char('$')), many1(letter()), optional(char('$')), many1(digit())))
       .map(|(ab_col, col, ab_row, row): (Option<char>, String, Option<char>, String)| {
           Address{addr: (opt_char_to_string(ab_col) +
           &col +
           &opt_char_to_string(ab_row) +
       &row).to_uppercase()}

        })
    }
}

parser! {
   fn parser_range[I]()(I) -> Range
    where [ I: Stream<Item = char>, ]
    {
       between(optional(parser_whitespaces()), optional(parser_whitespaces()), (parser_address(), char(':'), parser_address()))
       .map(|(ul, _, lr)| {
           Range{
               upper_left: ul,
               lower_right: lr
           }

        })
    }
}

parser! {
   fn parser_paren[I]()(I) -> Expression
    where [ I: Stream<Item = char>, ]
    {
       between((parser_whitespaces(),
        char('('),
         parser_whitespaces()),
         (parser_whitespaces(),
         char(')'),
         parser_whitespaces()), expr()).map(|e| Expression::Paren(Box::from(e)))
    }
}

parser! {
   fn parser_string[I]()(I) -> Expression
    where [ I: Stream<Item = char>, ]
    {
       between(
           (optional(parser_whitespaces()), char('"')),
           (char('"'), optional(parser_whitespaces())),
           many(none_of("\"".chars())),
       )
       .map(|s| Expression::Str(s))
    }
}

parser! {
   fn parser_comma_list[I]()(I) -> Vec<Expression>
    where [ I: Stream<Item = char>, ]
    {
       sep_by(expr(), (optional(parser_whitespaces()), char(','), optional(parser_whitespaces())))
       .map(|v: Vec<Expression>| v)
    }
}

parser! {
   fn parser_dotted_identifier[I]()(I) -> Vec<String>
    where [ I: Stream<Item = char>, ]
    {
        (parser_identifier(), many1((parser_whitespaces(), char('.'),  parser_identifier(), parser_whitespaces()))).
        map(|(id, more): (String, Vec<((), char, String, ())>)| {
            let mut ret = Vec::new();
            ret.push(id);
            for x in &more {
                ret.push(x.2.clone())
            }
            ret
        })
    }
}

parser! {
   fn parser_identifier[I]()(I) -> String
    where [ I: Stream<Item = char>, ]
    {
       (parser_whitespaces(), letter(), many(alpha_num().or(char('_'))), parser_whitespaces())
       .map(|(_, c, st, _): ((), char, String, ())| (c.to_string() + &st).to_uppercase())
    }
}

parser! {
   fn parser_function[I]()(I) -> Expression
    where [ I: Stream<Item = char>, ]
    {
       (parser_identifier(), optional(between(char('['), (char(']'), parser_whitespaces()), parser_comma_list())),
        between(char('('), (char(')'), parser_whitespaces()), parser_comma_list()))
       .map(|(id, notations, exprs)| Expression::Function(id.to_uppercase(), notations.unwrap_or(vec![]), exprs))
    }
}

parser! {
   fn parser_opr_exp[I]()(I) -> Expression
    where [ I: Stream<Item = char>, ]
    {
       (expr_mini(), parser_opr(), expr())
       .map(|(left, opr, right)| {

        match right {
            Expression::Infix(o2, sub_left, sub_right) if precedence(&o2) < precedence(&opr) =>
            Expression::Infix(o2, Box::from(Expression::Infix(opr, Box::from(left), sub_left)), sub_right),
            _ => Expression::Infix(opr, Box::from(left), Box::from(right))
        }
        })
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
    Expression(Box<Expression>),
    Let(String, Box<Expression>, Box<Expression>),
}

// `impl Parser` can be used to create reusable parsers with zero overhead
fn expr_<I>() -> impl Parser<Input = I, Output = Expression>
where
    I: Stream<Item = char>,
    // Necessary due to rust-lang/rust#24159
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(parser_let()),
        attempt(parser_opr_exp()),
        attempt(parser_paren()),
        attempt(parser_dotted_identifier().map(Expression::DottedIdentifier)),
        attempt(parser_function()),
        attempt(parser_range().map(|r| Expression::Range(r))),
        attempt(parser_address().map(|a| Expression::Address(a))),
        attempt(parser_identifier().map(Expression::Identifier)),
        attempt(parser_string()),
        attempt(parser_float()),
        attempt(parser_int()),
    ))
}

// `impl Parser` can be used to create reusable parsers with zero overhead
fn expr_mini<I>() -> impl Parser<Input = I, Output = Expression>
where
    I: Stream<Item = char>,
    // Necessary due to rust-lang/rust#24159
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        attempt(parser_let()),
        attempt(parser_paren()),
        attempt(parser_dotted_identifier().map(Expression::DottedIdentifier)),
        attempt(parser_function()),
        attempt(parser_range().map(|r| Expression::Range(r))),
        attempt(parser_address().map(|a| Expression::Address(a))),
        attempt(parser_identifier().map(Expression::Identifier)),
        attempt(parser_string()),
        attempt(parser_float()),
        attempt(parser_int()),
    ))
}

parser! {
    fn expr[I]()(I) -> Expression
    where [I: Stream<Item = char>,
    // Necessary due to rust-lang/rust#24159
    I::Error: ParseError<I::Item, I::Range, I::Position>]
    {
        expr_()
    }
}

parser! {
    fn whole_expr[I]()(I) -> Expression
    where [I: Stream<Item = char>,
    // Necessary due to rust-lang/rust#24159
    I::Error: ParseError<I::Item, I::Range, I::Position>]
    {
        (optional(string("=")), expr(), eof()).map(|(_, e, _)| e)
    }
}

#[test]
fn test_parsing() {
    let test_exprs: Vec<(&str, Result<Expression, i32>)> = vec![
        (r#"147"#, Ok(Expression::Int(147))),
        (r#"=  147"#, Ok(Expression::Int(147))),
        (
            r#""Hello World""#,
            Ok(Expression::Str(String::from("Hello World"))),
        ),
        (r#"147 /* comment */"#, Ok(Expression::Int(147))),
        (
            r#"147 //# comment 
         "#,
            Ok(Expression::Int(147)),
        ),
        (r#"147 //# comment   "#, Ok(Expression::Int(147))),
        (
            r#""Hello World""#,
            Ok(Expression::Str(String::from("Hello World"))),
        ),
        (r#"true"#, Ok(Expression::Identifier(String::from("TRUE")))),
        (
            r#"if(32, "yes", "no")"#,
            Ok(Expression::Function(
                String::from("IF"),
                vec![],
                vec![
                    Expression::Int(32),
                    Expression::Str(String::from("yes")),
                    Expression::Str(String::from("no")),
                ],
            )),
        ),
        (
            r#"if(true, "yes", "no")"#,
            Ok(Expression::Function(
                String::from("IF"),
                vec![],
                vec![
                    Expression::Identifier(String::from("TRUE")),
                    Expression::Str(String::from("yes")),
                    Expression::Str(String::from("no")),
                ],
            )),
        ),
        (
            r#"=if(true, "yes", "no")    "#,
            Ok(Expression::Function(
                String::from("IF"),
                vec![],
                vec![
                    Expression::Identifier(String::from("TRUE")),
                    Expression::Str(String::from("yes")),
                    Expression::Str(String::from("no")),
                ],
            )),
        ),
        (
            r#"  "Hello World""#,
            Ok(Expression::Str(String::from("Hello World"))),
        ),
        (r#"-32"#, Ok(Expression::Int(-32))),
        (r#"+32"#, Ok(Expression::Int(32))),
        (r#"32.99"#, Ok(Expression::Float(32.99))),
        (r#"-32.822"#, Ok(Expression::Float(-32.822))),
        (
            r#"A1"#,
            Ok(Expression::Address(Address {
                addr: String::from("A1"),
            })),
        ),
        (
            r#"$A3"#,
            Ok(Expression::Address(Address {
                addr: String::from("$A3"),
            })),
        ),
        (
            r#"$A3:b77"#,
            Ok(Expression::Range(Range {
                upper_left: Address {
                    addr: String::from("$A3"),
                },
                lower_right: Address {
                    addr: String::from("B77"),
                },
            })),
        ),
        (
            r#"$ABE3328282"#,
            Ok(Expression::Address(Address {
                addr: String::from("$ABE3328282"),
            })),
        ),
        (
            r#"SuM(a1:$B7)"#,
            Ok(Expression::Function(
                String::from("SUM"),
                vec![],
                vec![Expression::Range(Range {
                    upper_left: Address {
                        addr: String::from("A1"),
                    },
                    lower_right: Address {
                        addr: String::from("$B7"),
                    },
                })],
            )),
        ),
        (
            r#"(a1:$B7)"#,
            Ok(Expression::Paren(Box::from(Expression::Range(Range {
                upper_left: Address {
                    addr: String::from("A1"),
                },
                lower_right: Address {
                    addr: String::from("$B7"),
                },
            })))),
        ),
        (
            r#"( 44 )"#,
            Ok(Expression::Paren(Box::from(Expression::Int(44)))),
        ),
        (
            r#"( -73.4)"#,
            Ok(Expression::Paren(Box::from(Expression::Float(-73.4)))),
        ),
        (
            r#"(sum(2,3,4))"#,
            Ok(Expression::Paren(Box::from(Expression::Function(
                String::from("SUM"),
                vec![],
                vec![Expression::Int(2), Expression::Int(3), Expression::Int(4)],
            )))),
        ),
        (
            r#"(sum(
                2,
                
                3,   4
            ))"#,
            Ok(Expression::Paren(Box::from(Expression::Function(
                String::from("SUM"),
                vec![],
                vec![Expression::Int(2), Expression::Int(3), Expression::Int(4)],
            )))),
        ),
        (
            r#"(sum_dog(
                2,
                
                3,   4
            ))"#,
            Ok(Expression::Paren(Box::from(Expression::Function(
                String::from("SUM_DOG"),
                vec![],
                vec![Expression::Int(2), Expression::Int(3), Expression::Int(4)],
            )))),
        ),
        (
            r#"3 + 39"#,
            Ok(Expression::Infix(
                String::from("+"),
                Box::from(Expression::Int(3)),
                Box::from(Expression::Int(39)),
            )),
        ),
        (
            r#"3 + 39 / 42.1"#,
            Ok(Expression::Infix(
                String::from("+"),
                Box::from(Expression::Int(3)),
                Box::from(Expression::Infix(
                    String::from("/"),
                    Box::from(Expression::Int(39)),
                    Box::from(Expression::Float(42.1)),
                )),
            )),
        ),
        (
            r#"3 + 39 * 42.1"#,
            Ok(Expression::Infix(
                String::from("+"),
                Box::from(Expression::Int(3)),
                Box::from(Expression::Infix(
                    String::from("*"),
                    Box::from(Expression::Int(39)),
                    Box::from(Expression::Float(42.1)),
                )),
            )),
        ),
        (
            r#"SELECT[DISTINCT](
                
                ITEMS(foo, bar, baz * 3), /* I like yaks */
                FROM(cats, dogs), //# End of line comment
                GROUP_BY(),
                HAVING(),
                order_BY()
            )"#,
            Ok(Expression::Int(42)),
        ),
        (
            r#"
        
        let foobar = a1 + 3; //# cache foobar
        foobar * 5"#,
            Ok(Expression::Let(
                "FOOBAR".to_string(),
                Box::from(Expression::Infix(
                    "+".to_string(),
                    Box::from(Expression::Address(Address {
                        addr: "A1".to_string(),
                    })),
                    Box::from(Expression::Int(3)),
                )),
                Box::from(Expression::Infix(
                    "*".to_string(),
                    Box::from(Expression::Identifier("FOOBAR".to_string())),
                    Box::from(Expression::Int(5)),
                )),
            )),
        ),
        (
            r#"3 * 39 + 42.1"#,
            Ok(Expression::Infix(
                String::from("+"),
                Box::from(Expression::Infix(
                    String::from("*"),
                    Box::from(Expression::Int(3)),
                    Box::from(Expression::Int(39)),
                )),
                Box::from(Expression::Float(42.1)),
            )),
        ),
        (
            r#"(3 + 39)/ 42.1"#,
            Ok(Expression::Infix(
                String::from("/"),
                Box::from(Expression::Paren(Box::from(Expression::Infix(
                    String::from("+"),
                    Box::from(Expression::Int(3)),
                    Box::from(Expression::Int(39)),
                )))),
                Box::from(Expression::Float(42.1)),
            )),
        ),
        (
            r#"IF(a1, SUM(a1:$b$7), 3 + 39)"#,
            Ok(Expression::Function(
                String::from("IF"),
                vec![],
                vec![
                    Expression::Address(Address {
                        addr: String::from("A1"),
                    }),
                    Expression::Function(
                        String::from("SUM"),
                        vec![],
                        vec![Expression::Range(Range {
                            upper_left: Address {
                                addr: String::from("A1"),
                            },
                            lower_right: Address {
                                addr: String::from("$B$7"),
                            },
                        })],
                    ),
                    Expression::Infix(
                        String::from("+"),
                        Box::from(Expression::Int(3)),
                        Box::from(Expression::Int(39)),
                    ),
                ],
            )),
        ),
        (
            r#"foo.bar.bar"#,
            Ok(Expression::DottedIdentifier(vec![
                "FOO".to_string(),
                "BAR".to_string(),
                "BAR".to_string(),
            ])),
        ),
        (
            r#"55 + foo.bar "#,
            Ok(Expression::Infix(
                "+".to_string(),
                Box::from(Expression::Int(55)),
                Box::from(Expression::DottedIdentifier(vec![
                    "FOO".to_string(),
                    "BAR".to_string(),
                ])),
            )),
        ),
        (
            r#"false && true "#, // && cat.food == "Woof"
            Ok(Expression::Infix(
                "&&".to_string(),
                Box::from(Expression::Identifier("FALSE".to_string())),
                Box::from(Expression::Identifier("TRUE".to_string())),
            )),
        ),
        (
            r#"foo.bar.baz > 55 && cat.food == "hello" "#, // && cat.food == "Woof"
            Ok(Expression::Infix(
                "&&".to_string(),
                Box::from(Expression::Infix(
                    ">".to_string(),
                    Box::from(Expression::DottedIdentifier(vec![
                        "FOO".to_string(),
                        "BAR".to_string(),
                        "BAZ".to_string(),
                    ])),
                    Box::from(Expression::Int(55)))),
                Box::from(
                Expression::Infix("==".to_string(),
                Box::from(Expression:: DottedIdentifier(vec!["CAT".to_string(), "FOOD".to_string()])),
                Box::from(Expression:: Str("hello".to_string())))))),
            
        ),
        (r#"$5221343%%%"#, Err(44)),
    ];

    for item in test_exprs {
        match (whole_expr().parse(item.0), item.1) {
            (Ok((x, "")), Ok(y)) if y == Expression::Int(42) || x == y => {
                println!("From {} Got {:?}", item.0, x);
                assert!(true)
            }
            (Ok((x, "")), Ok(y)) => assert!(
                false,
                format!(
                    "For '{}'. Did not successfully compare {:?} and {:?}",
                    item.0, x, y
                )
            ),
            (Ok((_, x)), _) => assert!(
                false,
                format!("Failed to parse whole thing... remaining '{}'", x)
            ),
            (Err(_), Err(_)) => assert!(true),
            (Err(x), _) => assert!(false, format!("Trying '{}', got Error {:#?}", item.0, x)),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
