local arrow = {
    id = field.id(),
    x = field.int(),
    y = field.int(),
}

local line_cell = {
    id = field.id(),
    x = field.int(),
    y = field.int(),
    arrow = field.ref(arrow.id),
}

local tables = {
    arrow = arrow,
    line_cell = line_cell,
}

return tables
