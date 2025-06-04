local arrow_rule = require("arrow.arrow_rule")
local ded1_ded = require("arrow.ded1.ded1_ded")

local res = {}

local arr = RootEntry.new(arrow_rule.arrow.id):any()
local x = arr:field(arrow_rule.arrow.x)
local y = arr:field(arrow_rule.arrow.y)

local cell = arr:sub(arrow_rule.line_cell.arrow):all()
local 



local x = Var.new(arrow_rule.arrow.x)
local y = Var.new(arrow_rule.arrow.y)

res.push(Op.eq(x, y + 1))

local result = {
    n = IntField()
}
