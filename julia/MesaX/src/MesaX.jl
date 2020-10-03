module MesaX

import Random, JSON

greet() = print("Hello World!")
greet_alien() = print("Hello ", Random.randstring(8))

catfood = ["foo", "bar", "baz"]

moose = (cat = "33", dog = 33)

struct UUID
    id::UInt64
end

mutable struct Cell
    value
end
mutable struct ItemMut
    name::String
end

struct Item
    id::UUID
    cells::Array{Cell}
    other::ItemMut
end

mutable struct CategoryMut
    name::String
end
struct Category
    id::UUID
    items::Array{Item}
    other::CategoryMut
end

mutable struct WorksheetMut 
    name::String
end

struct Worksheet
    id::UUID
    other::WorksheetMut 
    categories::Array{Category}
end

struct Workbook
    sheets::Dict{UUID,Worksheet}
end


function foo2(i::Array{Int64})::Int64 
    sum = 0 
    for x in i
        sum += x
    end
    sum
end 

##
    q = Meta.parse("""function foo(i::Array{Int64})::Int128 
    sum::Int128 = 0 
    for x in i
        sum += x
    end
    sum
end """)
    dogs = Meta.parse("""ba = rand(Int64, 50000000)""")
    z = Meta.parse("""foo(ba)""")
    zp = Meta.parse("""sum(ba)""")
    Base.eval(q)
    Base.eval(dogs)
    @time Base.eval(z)
    @time Base.eval(zp)
##
end # module
