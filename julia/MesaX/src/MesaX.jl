module MesaX
include("structs.jl")
include("parser.jl")
import Random, JSON

greet() = print("Hello World!")
greet_alien() = print("Hello ", Random.randstring(8))

catfood = ["foo", "bar", "baz"]

moose = (cat = "33", dog = 33)

export greet, greet_alien

##
#     q = Meta.parse("""function foo(i::Array{Int64})::Int64 
#     sum::Int64 = 0 
#     for x in i
#         sum += x
#     end
#     sum
# end """)
#     dogs = Meta.parse("""ba = rand(Int64, 50000000)""")
#     z = Meta.parse("""foo(ba)""")
#     zp = Meta.parse("""sum(ba)""")
#     Base.eval(q)
#     Base.eval(dogs)
#     @time Base.eval(z)
#     @time Base.eval(zp)
##


end # module
