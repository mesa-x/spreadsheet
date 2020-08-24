package mesax.base

/**
  * Definition of a Type in Mesa X
  */
sealed trait Type {
  
}

final case object IntType extends Type {}
final case object FloatType extends Type {}
final case object StringType extends Type {}
final case object BoolType extends Type {}
final case class OptionalType(contained: Type) extends Type{}
final case class TupleType(contained: Vector[Type]) extends Type {}
final case class ArrayType(contained: Type) extends Type {}
final case class MapType(contained: Map[String, Type]) extends Type {}
final case object ErrorType extends Type {}
final case class SumType(contained: Vector[Type]) extends Type {}
