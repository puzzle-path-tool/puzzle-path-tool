local function dump(o)
    if type(o) == 'table' then
        local s = '{ '
        for k, v in pairs(o) do
            if type(k) ~= 'number' then
                k = '"' .. k .. '"'
                s = s .. '[' .. k .. '] = ' .. dump(v) .. ','
            else
                s = s .. dump(v) .. ','
            end
        end
        return s .. '} '
    else
        return tostring(o)
    end
end

---@class Never
--- @operator add(any): Never
--- @operator sub(any): Never
--- @operator mul(any): Never
--- @operator div(any): Never

--- @class Part
--- @operator add(integer | Part): Part
--- @operator add(any): Never
--- @operator sub(integer | Part): Part
--- @operator sub(any): Never
--- @operator mul(integer | Part): Part
--- @operator mul(any): Never
--- @operator div(integer | Part): Part
--- @operator div(any): Never

local part = {}
local meta = {}

---@param o any
---@return Part
function part:new(o)
    if type(o) ~= "table" then
        o = { o }
    end
    setmetatable(o, meta)
    return o
end

---@param name any
---@param a any
---@param b any
---@return Part
local function combine(name, a, b)
    return part:new({ a, name, b })
end

function meta.__add(a, b)
    return combine("+", a, b)
end

function meta.__sub(a, b)
    return combine("-", a, b)
end

function meta.__mul(a, b)
    return combine("*", a, b)
end

function meta.__div(a, b)
    return combine("/", a, b)
end

---@diagnostic disable-next-line: name-style-check
function meta.__tostring(o)
    if #o > 1 then
        local o2 = {}
        for i = 1, #o do
            o2[i] = tostring(o[i])
        end
        return "(" .. table.concat(o2, " ") .. ")"
    else
        return tostring(o[1])
    end
end

local a = part:new("a")
local b = part:new("b")

---@type table
local t = { 1 }

---@type Part
local y = t + a

---@type Part
local x = a + b * a + 44 * b - a / (5 + 5)

print("hey")
print(y)
print(x)
