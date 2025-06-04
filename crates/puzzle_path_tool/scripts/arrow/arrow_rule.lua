local arrow = {
    id = IdField.new(),
    x = IntField.new(),
    y = IntField.new(),
}

local line_cell = {
    id = IdField.new(),
    x = IntField.new(),
    y = IntField.new(),
    arrow = RefField.new(arrow.id),
}

local tables = {
    arrow = arrow,
    line_cell = line_cell,
}

return tables
