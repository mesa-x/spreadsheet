package mesax.parsing

import org.specs2._
import Parser._
import net.liftweb.common.Full
import fastparse._, NoWhitespace._

class QuickStartSpec extends Specification {
  def is = s2"""

 This is my first specification
   it is working                 $a1
   really working!               $a2
   Precedence                    $test_precedence
                                 """

  def test_precedence = precedence("&&") must be_<(precedence("+"))

  println("Parsing: " + Parser.parseInput("""/* cats /*sub 
  
  
  */ */+33.9/* foo */"""))

  println(
    "Comment parsing: " + parse("""/* comment */""", Parser.parseComment(_))
  )

  lazy val toTest = Vector(
    ("""147""", Full(ex_i(147))),
    ("""=  147""", Full(ex_i(147))),
    (""""Hello World"""", Full(ex_str("Hello World"))),
    ("""147 /* comment */""", Full(ex_i(147))),
    (
      """147 //# comment
             """,
      Full(ex_i(147))
    ),
    ("""147 //# comment   """, Full(ex_i(147))),
    (""""Hello World"""", Full(ex_str("Hello World"))),
    ("""true""", Full(ex_id("TRUE"))),
    (
      """if(32, "yes", "no")""",
      Full(
        ex_fun(
          "IF",
          Vector(),
          Vector(ex_i(32), ex_str("yes"), ex_str("no"))
        )
      )
    ),
    (
      """if(true, "yes", "no")""",
      Full(
        ex_fun(
          "IF",
          Vector(),
          Vector(ex_id("true"), ex_str("yes"), ex_str("no"))
        )
      )
    ),
    (
      """=if(true, "yes", "no")    """,
      Full(
        ex_fun(
          "if",
          Vector(),
          Vector(ex_id("true"), ex_str("yes"), ex_str("no"))
        )
      )
    ),
    ("""  "Hello World"""", Full(ex_str("Hello World"))),
    ("""-32""", Full(ex_i(-32))),
    ("""+32""", Full(ex_i(32))),
    ("""32.99""", Full(ex_f(32.99))),
    ("""-32.822""", Full(ex_f(-32.822))),
    ("""A1""", Full(ex_id("a1"))),
    ("""$A3""", Full(ex_adr("$A3"))),
    ("""$A3:b77""", Full(ex_rng("$a3", "b77"))),
    (
      """$ABE3328282""",
      Full(ex_adr("$ABE3328282"))
    ),
    (
      """SuM(a1:$B7)""",
      Full(ex_fun("sum", Vector(), Vector(ex_rng("a1", "$b7"))))
    )
  )

  def e1 = 1 must_== 1
  def e2 = 2 must_== 2
  def a1 = Parser.parseInput("""147""") must_== Full(ex_i(147))
  def a2 = Parser.parseInput("""=  147""") must_== Full(ex_i(147))
}

// class ParseTest {

// #[test)
// fn test_parsing() {
//     let test_exprs: Vec<(&str, Result<Expression, i32>)> = Vector(
//         ("""147""", Full(ex_i(147))),
//         ("""=  147""", Full(ex_i(147))),
//         (""""Hello World"""", Full(ex_str("Hello World"))),
//         ("""147 /* comment */""", Full(ex_i(147))),
//         (
//             """147 //# comment
//              """,
//             Full(ex_i(147)),
//         ),
//         ("""147 //# comment   """, Full(ex_i(147))),
//         (""""Hello World"""", Full(ex_str("Hello World"))),
//         ("""true""", Full(ex_id("TRUE"))),
//         (
//             """if(32, "yes", "no")""",
//             Full(ex_fun(
//                 "IF",
//                 Vector(),
//                 Vector(ex_i(32), ex_str("yes"), ex_str("no")),
//             )),
//         ),
//         (
//             """if(true, "yes", "no")""",
//             Full(ex_fun(
//                 "IF",
//                 Vector(),
//                 Vector(ex_id("true"), ex_str("yes"), ex_str("no")),
//             )),
//         ),
//         (
//             """=if(true, "yes", "no")    """,
//             Full(ex_fun(
//                 "if",
//                 Vector(),
//                 Vector(ex_id("true"), ex_str("yes"), ex_str("no")),
//             )),
//         ),
//         ("""  "Hello World"""", Full(ex_str("Hello World"))),
//         ("""-32""", Full(ex_i(-32))),
//         ("""+32""", Full(ex_i(32))),
//         ("""32.99""", Full(ex_f(32.99))),
//         ("""-32.822""", Full(ex_f(-32.822))),
//         ("""A1""", Full(ex_id("a1"))),
//         ("""$A3""", Full(ex_adr("$A3"))),
//         ("""$A3:b77""", Full(ex_rng("$a3", "b77"))),
//         (
//             """$ABE3328282""",
//             Full(ex_adr("$ABE3328282"),
//             ),
//         ),
//         (
//             """SuM(a1:$B7)""",
//             Full(ex_fun("sum", Vector(), Vector(ex_rng("a1", "$b7")))),
//         ),
//         ("""(a1:$B7)""", Full(ex_paren(ex_rng("a1", "$B7")))),
//         ("""( 44 )""", Full(ex_paren(ex_i(44)))),
//         ("""( -73.4)""", Full(ex_paren(ex_f(-73.4)))),
//         (
//             """(sum(2,3,4))""",
//             Full(ex_paren(ex_fun(
//                 "sum",
//                 Vector(),
//                 Vector(ex_i(2), ex_i(3), ex_i(4)),
//             ))),
//         ),
//         (
//             """(sum(
//                     2,

//                     3,   4
//                 ))""",
//             Full(ex_paren(ex_fun(
//                 "sum",
//                 Vector(),
//                 Vector(ex_i(2), ex_i(3), ex_i(4)),
//             ))),
//         ),
//         (
//             """(sum_dog(
//                     2,

//                     3,   4
//                 ))""",
//             Full(ex_paren(ex_fun(
//                 "sum_DOG",
//                 Vector(),
//                 Vector(ex_i(2), ex_i(3), ex_i(4)),
//             ))),
//         ),
//         (
//             """3 + 39""",
//             Full(ex_inf("+", ex_i(3), ex_i(39))),
//         ),
//         (
//             """3 + 39 / 42.1""",
//             Full(ex_inf(
//                 "+",
//                 ex_i(3),
//                 ex_inf("/", ex_i(39), ex_f(42.1)),
//             )),
//         ),
//         (
//             """3 + 39 * 42.1""",
//             Full(ex_inf(
//                 "+",
//                 ex_i(3),
//                 ex_inf("*", ex_i(39), ex_f(42.1)),
//             )),
//         ),
//         (
//             """SELECT[DISTINCT)(

//                     ITEMS(cats.foo, dogs.bar, cats.baz * 3), /* I like yaks */
//                     FROM(cats, dogs), //# End of line comment
//                     GROUP_BY(),
//                     HAVING(),
//                     order_BY()
//                 )""",
//             Full(ex_fun(
//                 "select",
//                 Vector(ex_id("distinct")),
//                 Vector(
//                     ex_fun(
//                         "items",
//                         Vector(),
//                         Vector(
//                             ex_dot(Vector("cats","foo")),
//                             ex_dot(Vector("dogs","bar")),
//                             ex_inf("*", ex_dot(Vector("cats","baz")), ex_i(3)),
//                         ),
//                     ),
//                     ex_fun("from", Vector(), Vector(ex_id("cats"), ex_id("dogs"))),
//                     ex_fun("group_BY", Vector(), Vector()),
//                     ex_fun("having", Vector(), Vector()),
//                     ex_fun("ORDER_BY", Vector(), Vector()),
//                 ),
//             )),
//         ),
//         (
//             """

//             let foobar = a1 + 3; //# cache foobar
//             foobar * 5""",
//             Full(ex_let(
//                 "foobar",
//                 ex_inf("+", ex_adr("a1"), ex_i(3)),
//                 ex_inf("*", ex_id("foobar"), ex_i(5)),
//             )),
//         ),
//         (
//             """3 * 39 + 42.1""",
//             Full(ex_inf("+", ex_inf("*", ex_i(3), ex_i(39)), ex_f(42.1))),
//         ),
//         (
//             """(3 + 39)/ 42.1""",
//             Full(ex_inf(
//                 "/",
//                 ex_paren(ex_inf("+", ex_i(3), ex_i(39))),
//                 ex_f(42.1),
//             )),
//         ),
//         (
//             """IF(a1, SUM(a1:$b$7), 3 + 39)""",
//             Full(ex_fun(
//                 "if",
//                 Vector(),
//                 Vector(
//                     ex_id("a1"),
//                     ex_fun("sum", Vector(), Vector(ex_rng("a1", "$b$7"))),
//                     ex_inf("+", ex_i(3), ex_i(39)),
//                 ),
//             )),
//         ),
//         ("""foo.bar.bar""", Full(ex_dot(Vector("foo", "BAR", "BaR")))),
//         (
//             """55 + foo.bar """,
//             Full(ex_inf("+", ex_i(55), ex_dot(Vector("foo", "BAR")))),
//         ),
//         (
//             """false && true  &&

//             cat. /* comment */ food ==

//             "Woof" """,
//             Full(ex_inf(
//                 "&&",
//                 ex_id("false"),
//                 ex_inf(
//                     "&&",
//                     ex_id("true"),
//                     ex_inf("==", ex_dot(Vector("cat", "FOOD")), ex_str("Woof")),
//                 ),
//             )),
//         ),
//         (
//             """foo.bar.baz > 55 && cat.food == "hello" """, // && cat.food == "Woof"
//             Full(ex_inf(
//                 "&&",
//                 ex_inf(">", ex_dot(Vector("foo", "bar", "baz")), ex_i(55)),
//                 ex_inf("==", ex_dot(Vector("cat", "food")), ex_str("hello")),
//             )),
//         ),
//         ("""$5221343%%%""", Err(44)),
//     );

//     for item in test_exprs {
//         match (whole_expr_str(item.0), item.1) {
//             (Full(x), Full(y)) if x == y => {
//                 assert!(true)
//             }
//             (Full(x), Full(y)) => assert!(
//                 false,
//                 format!(
//                     "For '{}'. Did not successfully compare:\n{:?}\nand\n{:?}\n",
//                     item.0, x, y
//                 )
//             ),
//             (Full(x), Err(_)) => assert!(false, format!("Expecting error, but got {:?}\n", x)),
//             (Err(_), Err(_)) => assert!(true),
//             (Err(x), _) => assert!(false, format!("Trying '{}', got Error {:#?}\n", item.0, x)),
//         }
//     }
// }

// }
