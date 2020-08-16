package mesax.parsing

// import org.specs2._
import Parser._
import net.liftweb.common.{Full, Empty}
import fastparse._, NoWhitespace._
import org.specs2.mutable._

class QuickStartSpec extends Specification {

  "The Parser" should {
    "Have correct precedence" in {
      precedence("&&") must be_<(precedence("+"))
    }
    "Parse a number" in {
      Parser.parseInput("""147""") must_== Full(ex_i(147))
    }

    "Parse a comment" in {
      parse("""/* comment */""", Parser.parseComment(_)) must_== Parsed.Success(
        "/* comment */",
        13
      )
    }

    "parse a number with comments" in {
      Parser.parseInput("""/* cats /*sub 
  
  
  */ */+33.9/* foo */""") must_== Full(ParsedFloat(33.9, Empty))
    }

    "parse an expression that starts with '='" in {
      Parser.parseInput("""=  147""") must_== Full(ex_i(147))
    }

    "parse a string" in {
      Parser.parseInput(""""Hello World"""") must_== Full(ex_str("Hello World"))
    }

    "Parse a number and comment" in {
      Parser.parseInput("""147 /* comment */""") must_== Full(ex_i(147))
    }
    "Parse a number and a comment to end of line" in {

      Parser.parseInput("""147 //# comment
             """) must_==
        Full(ex_i(147))
    }

    "Parse number and comment to end of line with no CR" in {
      Parser.parseInput("""146 //# comment   """) must_== Full(ex_i(146))
    }
    "Parse a string with comment" in {
      Parser.parseInput(""""Hello World" // I'm a line comment""") must_== Full(
        ex_str("Hello World")
      )
    }

    "Parse identifier" in {
      Parser.parseInput("""true""") must_== Full(ex_id("TRUE"))
    }

    "Parse function (if)" in {
      Parser.parseInput("""if(32, "yes", "no")""") must_==
        Full(
          ex_fun(
            "IF",
            Vector(),
            Vector(ex_i(32), ex_str("yes"), ex_str("no"))
          )
        )
    }

    "Parse function (if) with identified as first param" in {
      Parser.parseInput("""if(true, "yes", "no")""") must_==
        Full(
          ex_fun(
            "IF",
            Vector(),
            Vector(ex_id("true"), ex_str("yes"), ex_str("no"))
          )
        )
    }

    "Parse function (if) with trailing whitespace" in {
      Parser.parseInput("""=if(true, "yes", "no")    """) must_==
        Full(
          ex_fun(
            "if",
            Vector(),
            Vector(ex_id("true"), ex_str("yes"), ex_str("no"))
          )
        )
    }

    "Parse a string with leading whitespace" in {
      Parser.parseInput("""  "Hello World"""") must_== Full(
        ex_str("Hello World")
      )
    }
    "Parse a negative int" in {
      Parser.parseInput("""-32""") must_== Full(ex_i(-32))
    }

    "Parse a positive int with leading '+'" in {
      Parser.parseInput("""+32""") must_== Full(ex_i(32))
    }
    "Parse a floating point number" in {
      Parser.parseInput("""32.99""") must_== Full(ex_f(32.99))
    }
    "Parse a negative floating point number" in {
      Parser.parseInput("""-32.822""") must_== Full(ex_f(-32.822))
    }
  }
  "Parse an address, but it should be an identifer" in {
    Parser.parseInput("""A1""") must_== Full(ex_id("a1"))
  }

  "Parse an absolute address as an address" in {
    Parser.parseInput("""$A3""") must_== Full(ex_adr("$A3"))
  }
  "Parse a range" in {
    Parser.parseInput("""$A3:b77""") must_== Full(ex_rng("$a3", "b77"))
  }
  "Parse long address" in {
    Parser.parseInput("""$ABE3328282""") must_==
      Full(ex_adr("$ABE3328282"))
  }
  "Parse function that contains a range" in {
    Parser.parseInput("""SuM(a1:$B7)""") must_==
      Full(ex_fun("sum", Vector(), Vector(ex_rng("a1", "$b7"))))
  }
  "Parse a range inside a paren" in {
    Parser.parseInput("""(a1:$B7)""") must_== Full(
      ex_paren(ex_rng("a1", "$B7"))
    )
  }
  "Parse number inside paren" in {
    Parser.parseInput("""( 44 )""") must_== Full(ex_paren(ex_i(44)))
  }
  "Parse negative number inside paren" in {
    Parser.parseInput("""( -73.4)""") must_== Full(ex_paren(ex_f(-73.4)))
  }
  "Parse function inside paren" in {
    Parser.parseInput("""(sum(2,3,4))""") must_==
      Full(
        ex_paren(
          ex_fun(
            "sum",
            Vector(),
            Vector(ex_i(2), ex_i(3), ex_i(4))
          )
        )
      )
  }
  "Parse multi-line function inside paren" in {
    Parser.parseInput("""(sum(
              2,

              3,   4
          ))""") must_==
      Full(
        ex_paren(
          ex_fun(
            "sum",
            Vector(),
            Vector(ex_i(2), ex_i(3), ex_i(4))
          )
        )
      )
  }
  "Parse multi-line function with underscore identifier inside paren" in {
    Parser.parseInput("""(sum_dog(
              2, // I'm a comment

              /* I'm a multi-line comment

              */

              3,   4
          ))""") must_==
      Full(
        ex_paren(
          ex_fun(
            "sum_DOG",
            Vector(),
            Vector(ex_i(2), ex_i(3), ex_i(4))
          )
        )
      )
  }
  "Simple addition" in {
    Parser.parseInput("""3 + 39""") must_==
      Full(ex_inf("+", ex_i(3), ex_i(39)))
  }
  "Addition and division... do the precedence thing" in {
    Parser.parseInput("""3 + 39 / 42.1""") must_==
      Full(
        ex_inf(
          "+",
          ex_i(3),
          ex_inf("/", ex_i(39), ex_f(42.1))
        )
      )
  }
  "Addition and multiplication" in {
    Parser.parseInput("""3 + 39 * 42.1""") must_==
      Full(
        ex_inf(
          "+",
          ex_i(3),
          ex_inf("*", ex_i(39), ex_f(42.1))
        )
      )
  }
  "Multiplcation and addition" in {
    Parser.parseInput("""3 * 39 + 42.1""") must_==
      Full(ex_inf("+", ex_inf("*", ex_i(3), ex_i(39)), ex_f(42.1)))
  }
  "Parens and operators" in {
    Parser.parseInput("""(3 + 39)/ 42.1""") must_==
      Full(
        ex_inf(
          "/",
          ex_paren(ex_inf("+", ex_i(3), ex_i(39))),
          ex_f(42.1)
        )
      )
  }
  "Functions, ranges, and operators" in {
    Parser.parseInput("""IF(a1, SUM(a1:$b$7), 3 + 39)""") must_==
      Full(
        ex_fun(
          "if",
          Vector(),
          Vector(
            ex_id("a1"),
            ex_fun("sum", Vector(), Vector(ex_rng("a1", "$b$7"))),
            ex_inf("+", ex_i(3), ex_i(39))
          )
        )
      )
  }
}
// class ParseTest {

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
