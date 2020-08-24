package mesax.base

/**
  * Define a Value for Mesa X
  */
sealed trait Value {
    /**
      * Get the type of the value
      *
      * @return the Type of the Value
      */
  def theType: Type 
}

final case class IntValue(v: Int) extends Value {
    def theType = IntType
}
final case class FloatValue(v: Double) extends Value {
    def theType = FloatType
}
final case class StringValue(v: String) extends Value {
    def theType: Type = StringType
}
final case class BoolValue(v: Boolean) extends Value {
    def theType = BoolType
}
final case class OptionalValue(v: Value) extends Value {
    def theType = OptionalType(v.theType)
}
final case class TupleValue(v: Seq[Value]) extends Value {
    def theType = TupleType(v.toVector.map(_.theType))
}

// final case class ArrayType(contained: Type) extends Type {}
// final case class MapType(contained: Map[String, Type]) extends Type {}
// final case object ErrorType extends Type {}
// final case class SumType(contained: Vector[Type]) extends Type {}