use mesax::parser::*;


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
                    Box::from(Expression::Int(55)),
                )),
                Box::from(Expression::Infix(
                    "==".to_string(),
                    Box::from(Expression::DottedIdentifier(vec![
                        "CAT".to_string(),
                        "FOOD".to_string(),
                    ])),
                    Box::from(Expression::Str("hello".to_string())),
                )),
            )),
        ),
        (r#"$5221343%%%"#, Err(44)),
    ];

    for item in test_exprs {
        match (whole_expr(item.0), item.1) {
            (Ok(x), Ok(y)) if y == Expression::Int(42) || x == y => {
                println!("From {} Got {:?}\n", item.0, x);
                assert!(true)
            }
            (Ok(x), Ok(y)) => assert!(
                false,
                format!(
                    "For '{}'. Did not successfully compare {:?} and {:?}\n",
                    item.0, x, y
                )
            ),
            (Ok( x), Err(_)) => assert!(
                false,
                format!("Expecting error, but got {:?}\n", x)
            ),
            (Err(_), Err(_)) => assert!(true),
            (Err(x), _) => assert!(false, format!("Trying '{}', got Error {:#?}\n", item.0, x)),
        }
    }
}
