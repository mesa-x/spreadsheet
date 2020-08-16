package mesax.parsing

import net.liftweb.common.Box
import net.liftweb.common.Empty

import fastparse._, NoWhitespace._
import fastparse.Parsed.Success
import net.liftweb.common.Full
import net.liftweb.common.{Failure, ParamFailure}

sealed trait Expression {
  def parseInfo: Parser.ParseInfo

}

final case class ParsedInt(number: Long, parseInfo: Parser.ParseInfo)
    extends Expression {}

final case class ParsedFloat(number: Double, parseInfo: Parser.ParseInfo)
    extends Expression {}

final case class ParsedDecimal(number: BigDecimal, parseInfo: Parser.ParseInfo)
    extends Expression {}

final case class ParsedString(str: String, parseInfo: Parser.ParseInfo)
    extends Expression {}

final case class ParsedId(id: String, parseInfo: Parser.ParseInfo)
    extends Expression {}

final case class ParsedAddress(addr: String, parseInfo: Parser.ParseInfo)
    extends Expression {}

final case class ParsedRange(
    ul: String,
    lr: String,
    parseInfo: Parser.ParseInfo
) extends Expression {}

final case class DottedIdentifier(
    id: Seq[String],
    parseInfo: Parser.ParseInfo
) extends Expression {}

final case class ParsedFunction(
    func: String,
    annotation: Seq[Expression],
    params: Seq[Expression],
    parseInfo: Parser.ParseInfo
) extends Expression {}

final case class Paren(expr: Expression, parseInfo: Parser.ParseInfo)
    extends Expression {}
final case class Infix(
    opr: String,
    leftExpr: Expression,
    rightExpr: Expression,
    parseInfo: Parser.ParseInfo
) extends Expression {}
final case class Left(
    thing: String,
    expr1: Expression,
    expr2: Expression,
    parseInfo: Parser.ParseInfo
) extends Expression {}

final case class PositionInfo(
    start: Int,
    startLine: Int,
    end: Int,
    endLine: Int,
    text: String
)

object Parser {
  type ParseInfo = Box[PositionInfo]

  def parseInput(in: String): Box[Expression] =
    parse(in, parseAll(_)) match {
      case Parsed.Success(value, index) => Full(value)
      case x @ Parsed.Failure(label, index, extra) =>
        ParamFailure("Failed parse", Empty, Empty, x)
    }

  def ex_i(in: Long): Expression = ParsedInt(in, Empty)
  def ex_f(in: Double): Expression = ParsedFloat(in, Empty)
  def ex_str(in: String): Expression = ParsedString(in, Empty)
  def ex_id(in: String): Expression = ParsedId(in.toUpperCase(), Empty)
  def ex_inf(opr: String, e1: Expression, e2: Expression): Expression =
    Infix(opr, e1, e2, Empty)
  def ex_adr(in: String): Expression = ParsedAddress(in.toUpperCase(), Empty)
  def ex_rng(ul: String, lr: String): Expression =
    ParsedRange(ul.toUpperCase(), lr.toUpperCase(), Empty)
  def ex_paren(expr: Expression): Expression = Paren(expr, Empty)
  def ex_fun(
      in: String,
      annotation: Seq[Expression],
      params: Seq[Expression]
  ): Expression = ParsedFunction(in.toUpperCase(), annotation, params, Empty)

  def precedence(opr: String): Int = {
    opr match {
      case "+" | "-"                      => 100
      case "*" | "/"                      => 200
      case "==" | ">" | "<" | ">=" | "<=" => 20
      case "&&" | "||"                    => 10
      case _                              => 0
    }
  }

  def firstIdentifier[_: P] = CharIn("a-zA-Z")

  def addlIdentifier[_: P] = CharIn("a-zA-Z0-9_")

  def parseAll[_: P]: P[Expression] =
    (Start ~ firstEquals.? ~ parseExpr ~ parseWhitespace.? ~ End)
      .map(v => v._2)

  def parseExpr[_: P]: P[Expression] =
    (parseOpr | parseParen | parseFunction | parseRange |
      parseIdentifier | parseAddress | parseNumber | parseString)

  def parseMiniExpr[_: P]: P[Expression] =
    (parseParen | parseFunction | parseRange |
      parseIdentifier | parseAddress | parseNumber | parseString)

  def parseOpr[_: P]: P[Infix] =
    P(parseMiniExpr ~ findOpr ~ parseExpr).map({
      case (left, opr, right @ Infix(io, il, ir, rp))
      // deal with re-writing if the precidence of the operators calls for it
          if precedence(io) < precedence(opr) =>
        Infix(
          io,
          Infix(opr, left, il, Empty),
          ir,
          rp
        ) // FIXME figure out how to do the right thing with parse span info
      case (left, opr, right) =>
        Infix(opr, left, right, Empty)
    })

  def findOpr[_: P]: P[String] = P("+".! | "*".! | "/".!)
//       fn expr_mini(input: Span) -> IResult<Span, Expression> {
//     alt((
//         &parser_let,
//         &parser_paren,
//         &parser_dotted_identifier,
//         &parser_function,
//         &parser_address,
//         &parser_range,
//         &parser_identifier,
//         &parser_string,
//         &parser_float,
//         &parser_int,
//     ))(input)
// }

// fn expr(input: Span) -> IResult<Span, Expression> {
//     alt((
//         &parser_let,
//         &parser_opr_exp,
//         &parser_paren,
//         &parser_dotted_identifier,
//         &parser_function,
//         &parser_range,
//         &parser_identifier,
//         &parser_address,
//         &parser_string,
//         &parser_float,
//         &parser_int,
//     ))(input)
// }

  def firstEquals[_: P] = P(parseWhitespace.? ~ "=" ~ parseWhitespace.?)
  def parseParen[_: P]: P[Paren] =
    P(
      parseWhitespace.? ~ "(" ~ parseExpr ~ parseWhitespace.? ~ ")" ~ parseWhitespace.?
    ).map(v => Paren(v._2, Empty))
  def parseAddress[_: P]: P[ParsedAddress] =
    P(
      parseWhitespace.? ~ "$".?.! ~ CharIn("a-zA-Z")
        .rep(1)
        .! ~ "$".?.! ~ CharIn("0-9").rep(1).! ~ parseWhitespace.?
    ).map(v => ParsedAddress((v._2 + v._3 + v._4 + v._5).toUpperCase(), Empty))

  def parseRange[_: P]: P[ParsedRange] =
    P(parseWhitespace.? ~ parseAddress ~ ":" ~ parseAddress ~ parseWhitespace.?)
      .map(v => ParsedRange(v._2.addr, v._3.addr, Empty))

  def parseLineComment[_: P]: P[String] =
    P(
      ("//" ~ (!(CharIn("\n\r") | End) ~ AnyChar).rep(0) ~ (CharIn(
        "\r\n"
      ) | End)).!
    )
  def parseFunction[_: P]: P[ParsedFunction] =
    P(
      parseIdentifier ~ parseWhitespace.? ~ "(" ~ (parseExpr ~ parseWhitespace.? ~ ",".?)
        .map(v => v._1)
        .rep(0) ~ ")" ~ parseWhitespace.?
    ).map(v => ParsedFunction(v._1.id, Vector(), v._3, Empty))
  def parseComment[_: P]: P[String] =
    P("/*" ~ (!"*/" ~ (parseComment | AnyChar)).rep(0) ~ "*/").!

  def parseWhitespace[_: P] =
    P((parseLineComment | parseComment | CharIn(" \t\n\r")).rep(0))

  def parseInt[_: P]: P[Expression] =
    P((("+" | "-").? ~ CharIn("0-9").rep(1)).!.map(v => ex_i(v.toInt)))
  def parseFloat[_: P]: P[Expression] =
    P(
      (("+" | "-").? ~ CharIn("0-9").rep(1) ~ "." ~ CharIn("0-9").rep(1)).!.map(
        v => ex_f(v.toDouble)
      )
    )

  def parseIdentifier[_: P]: P[ParsedId] =
    P(
      parseWhitespace.? ~ (("_" ~ firstIdentifier ~ addlIdentifier.rep(0)) |
        (firstIdentifier ~ addlIdentifier.rep(0))).! ~ parseWhitespace.?
    ).map(v => ParsedId(v._2.toUpperCase(), Empty))
  def parseNumber[_: P]: P[Expression] =
    P(parseWhitespace.? ~ (parseFloat | parseInt) ~ parseWhitespace.?).map(v =>
      v._2
    )

  def parseRawString[_: P]: P[Expression] =
    P("\"" ~ (!"\"" ~ AnyChar).rep(0).! ~ "\"").map(v => ex_str(v))

  def parseString[_: P]: P[Expression] =
    P(parseWhitespace.? ~ parseRawString ~ parseWhitespace.?).map(v => v._2)
}
