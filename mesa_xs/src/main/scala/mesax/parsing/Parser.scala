package mesax.parsing

import net.liftweb.common.Box
import net.liftweb.common.Empty
import fastparse._, NoWhitespace._
import fastparse.Parsed.Success
import net.liftweb.common.Full

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
    id: Vector[String],
    parseInfo: Parser.ParseInfo
) extends Expression {}

final case class ParsedFunction(
    func: String,
    annotation: Vector[Expression],
    params: Vector[Expression],
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
      case Parsed.Failure(label, index, extra) =>
        Empty // FIXME -- make a failure

    }

  def ex_i(in: Long): Expression = ParsedInt(in, Empty)
  def ex_f(in: Double): Expression = ParsedFloat(in, Empty)
  def ex_str(in: String): Expression = ParsedString(in, Empty)
  def ex_id(in: String): Expression = ParsedId(in.toUpperCase(), Empty)
  def ex_adr(in: String): Expression = ParsedAddress(in.toUpperCase(), Empty)
  def ex_rng(ul: String, lr: String): Expression =
    ParsedRange(ul.toUpperCase(), lr.toUpperCase(), Empty)
  def ex_fun(
      in: String,
      annotation: Vector[Expression],
      params: Vector[Expression]
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

  def parseAll[_: P]: P[Expression] =
    (Start ~ firstEquals.? ~ parseNumber ~ End).map(v => v._2)

  def firstEquals[_: P] = P(parseWhitespace.? ~ "=" ~ parseWhitespace.?)

  def parseComment[_: P]: P[String] =
    P("/*" ~ (!"*/" ~ (parseComment | AnyChar)).rep(0) ~ "*/").!

  def parseWhitespace[_: P] = P(parseComment | " ").?

  def parseInt[_: P]: P[Expression] =
    P((("+" | "-").? ~ CharIn("0-9").rep(1)).!.map(v => ex_i(v.toInt)))
  def parseFloat[_: P]: P[Expression] =
    P(
      (("+" | "-").? ~ CharIn("0-9").rep(1) ~ "." ~ CharIn("0-9").rep(1)).!.map(
        v => ex_f(v.toDouble)
      )
    )
  def parseNumber[_: P]: P[Expression] =
    P(parseWhitespace.? ~ (parseFloat | parseInt) ~ parseWhitespace.?).map(v =>
      v._2
    )
}
