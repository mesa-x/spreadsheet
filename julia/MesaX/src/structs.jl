module Structs

import Random

struct UUID
    id::UInt64
end

new_uuid() = UUID(Random.rand(UInt64))


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

new_item() = Item(new_uuid(), [], ItemMut(""))

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
mutable struct Line
    value::Any
    items::Array{Item}
end

struct Worksheet
    id::UUID
    other::WorksheetMut 
    categories::Array{Category}
end

struct Workbook
    sheets::Dict{UUID,Worksheet}
end

yak(x::Int64)::Int64 = x * 40

export Workbook, new_uuid

end # module