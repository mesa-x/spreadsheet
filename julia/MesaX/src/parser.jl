module Parser

using CombinedParsers
using TextParse
    
# @syntax subterm = Either{Rational{Int}}(Any[TextParse.Numeric(Int)]);
# @syntax for parenthesis in subterm
#     mult         = evaluate |> join(subterm, CharIn("*/"), infix=:prefix )
#     @syntax term = evaluate |> join(mult,    CharIn("+-"), infix=:prefix )
#     Sequence(2,'(',term,')')
# end;

end