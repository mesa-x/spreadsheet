use mesax::parser::*;
use mesax::parser_util::*;

#[test]
fn test_parsing() {
    let test_exprs: Vec<(&str, Result<Expression, i32>)> = vec![
        (r#"147"#, Ok(ex_i(147))),
        (r#"=  147"#, Ok(ex_i(147))),
        (r#""Hello World""#, Ok(ex_str("Hello World"))),
        (r#"147 /* comment */"#, Ok(ex_i(147))),
        (
            r#"147 //# comment 
             "#,
            Ok(ex_i(147)),
        ),
        (r#"147 //# comment   "#, Ok(ex_i(147))),
        (r#""Hello World""#, Ok(ex_str("Hello World"))),
        (r#"true"#, Ok(ex_id("TRUE"))),
        (
            r#"if(32, "yes", "no")"#,
            Ok(ex_fun(
                "IF",
                vec![],
                vec![ex_i(32), ex_str("yes"), ex_str("no")],
            )),
        ),
        (
            r#"if(true, "yes", "no")"#,
            Ok(ex_fun(
                "IF",
                vec![],
                vec![ex_id("true"), ex_str("yes"), ex_str("no")],
            )),
        ),
        (
            r#"=if(true, "yes", "no")    "#,
            Ok(ex_fun(
                "if",
                vec![],
                vec![ex_id("true"), ex_str("yes"), ex_str("no")],
            )),
        ),
        (r#"  "Hello World""#, Ok(ex_str("Hello World"))),
        (r#"-32"#, Ok(ex_i(-32))),
        (r#"+32"#, Ok(ex_i(32))),
        (r#"32.99"#, Ok(ex_f(32.99))),
        (r#"-32.822"#, Ok(ex_f(-32.822))),
        (r#"A1"#, Ok(ex_id("a1"))),
        (r#"$A3"#, Ok(ex_adr("$A3"))),
        (r#"$A3:b77"#, Ok(ex_rng("$a3", "b77"))),
        (r#"$ABE3328282"#, Ok(ex_adr("$ABE3328282"))),
        (
            r#"SuM(a1:$B7)"#,
            Ok(ex_fun("sum", vec![], vec![ex_rng("a1", "$b7")])),
        ),
        (r#"(a1:$B7)"#, Ok(ex_paren(ex_rng("a1", "$B7")))),
        (r#"( 44 )"#, Ok(ex_paren(ex_i(44)))),
        (r#"( -73.4)"#, Ok(ex_paren(ex_f(-73.4)))),
        (
            r#"(sum(2,3,4))"#,
            Ok(ex_paren(ex_fun(
                "sum",
                vec![],
                vec![ex_i(2), ex_i(3), ex_i(4)],
            ))),
        ),
        (
            r#"(sum(
                    2,
                    
                    3,   4
                ))"#,
            Ok(ex_paren(ex_fun(
                "sum",
                vec![],
                vec![ex_i(2), ex_i(3), ex_i(4)],
            ))),
        ),
        (
            r#"(sum_dog(
                    2,
                    
                    3,   4
                ))"#,
            Ok(ex_paren(ex_fun(
                "sum_DOG",
                vec![],
                vec![ex_i(2), ex_i(3), ex_i(4)],
            ))),
        ),
        (r#"3 + 39"#, Ok(ex_inf("+", ex_i(3), ex_i(39)))),
        (
            r#"3 + 39 / 42.1"#,
            Ok(ex_inf("+", ex_i(3), ex_inf("/", ex_i(39), ex_f(42.1)))),
        ),
        (
            r#"3 + 39 * 42.1"#,
            Ok(ex_inf("+", ex_i(3), ex_inf("*", ex_i(39), ex_f(42.1)))),
        ),
        (
            r#"SELECT[DISTINCT](
                    
                    ITEMS(cats.foo, dogs.bar, cats.baz * 3), /* I like yaks */
                    FROM(cats, dogs), //# End of line comment
                    GROUP_BY(),
                    HAVING(),
                    order_BY()
                )"#,
            Ok(ex_fun(
                "select",
                vec![ex_id("distinct")],
                vec![
                    ex_fun(
                        "items",
                        vec![],
                        vec![
                            ex_dot(vec!["cats", "foo"]),
                            ex_dot(vec!["dogs", "bar"]),
                            ex_inf("*", ex_dot(vec!["cats", "baz"]), ex_i(3)),
                        ],
                    ),
                    ex_fun("from", vec![], vec![ex_id("cats"), ex_id("dogs")]),
                    ex_fun("group_BY", vec![], vec![]),
                    ex_fun("having", vec![], vec![]),
                    ex_fun("ORDER_BY", vec![], vec![]),
                ],
            )),
        ),
        (
            r#"
            
            let foobar = a1 + 3; //# cache foobar
            foobar * 5"#,
            Ok(ex_let(
                "foobar",
                ex_inf("+", ex_adr("a1"), ex_i(3)),
                ex_inf("*", ex_id("foobar"), ex_i(5)),
            )),
        ),
        (
            r#"3 * 39 + 42.1"#,
            Ok(ex_inf("+", ex_inf("*", ex_i(3), ex_i(39)), ex_f(42.1))),
        ),
        (
            r#"(3 + 39)/ 42.1"#,
            Ok(ex_inf(
                "/",
                ex_paren(ex_inf("+", ex_i(3), ex_i(39))),
                ex_f(42.1),
            )),
        ),
        (
            r#"IF(a1, SUM(a1:$b$7), 3 + 39)"#,
            Ok(ex_fun(
                "if",
                vec![],
                vec![
                    ex_id("a1"),
                    ex_fun("sum", vec![], vec![ex_rng("a1", "$b$7")]),
                    ex_inf("+", ex_i(3), ex_i(39)),
                ],
            )),
        ),
        (r#"foo.bar.bar"#, Ok(ex_dot(vec!["foo", "BAR", "BaR"]))),
        (
            r#"55 + foo.bar "#,
            Ok(ex_inf("+", ex_i(55), ex_dot(vec!["foo", "BAR"]))),
        ),
        (
            r#"false && true  && 
            
            
            cat. /* comment */ food == 
            
            
            "Woof" "#,
            Ok(ex_inf(
                "&&",
                ex_id("false"),
                ex_inf(
                    "&&",
                    ex_id("true"),
                    ex_inf("==", ex_dot(vec!["cat", "FOOD"]), ex_str("Woof")),
                ),
            )),
        ),
        (
            r#"foo.bar.baz > 55 && cat.food == "hello" "#, // && cat.food == "Woof"
            Ok(ex_inf(
                "&&",
                ex_inf(">", ex_dot(vec!["foo", "bar", "baz"]), ex_i(55)),
                ex_inf("==", ex_dot(vec!["cat", "food"]), ex_str("hello")),
            )),
        ),
        (r#"$5221343%%%"#, Err(44)),
    ];

    for item in test_exprs {
        match (whole_expr_str(item.0), item.1) {
            (Ok(x), Ok(y)) if x == y => {
                assert!(true)
            }
            (Ok(x), Ok(y)) => assert!(
                false,
                "For '{}'. Did not successfully compare:\n{:?}\nand\n{:?}\n",
                item.0, x, y
            ),
            (Ok(x), Err(_)) => assert!(false, "Expecting error, but got {:?}\n", x),
            (Err(_), Err(_)) => assert!(true),
            (Err(x), _) => assert!(false, "Trying '{}', got Error {:#?}\n", item.0, x),
        }
    }
}
