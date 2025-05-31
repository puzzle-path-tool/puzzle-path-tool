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

--- @class Part
--- @operator add(any): Part
--- @operator sub(any): Part
--- @operator mul(any): Part
--- @operator div(any): Part

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

local y = 1 + a

local x = a + b * a * b - a / (5 + 5)

print("hey")
print(y)
print(x)
