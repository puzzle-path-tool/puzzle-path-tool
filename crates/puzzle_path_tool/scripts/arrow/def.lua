---@meta

field = {}

---@return IntField
function field.int() end

---@return IdField
function field.id() end

---@return CharField
function field.char() end

---@return FloatField
function field.float() end

---@return BoolField
function field.bool() end

---@param id IdField
---@return RefField
function field.ref(id) end

---@param id IdField
---@return RootEntry
function field.root(id) end

---@class Field
---@class IntField: Field
---@class IdField: Field
---@class CharField: Field
---@class FloatField: Field
---@class BoolField: Field
---@class RefField: Field

---@class RootEntry
local root_entry = {}

---@return RootEntryInstance
function root_entry:any() end

---@param n integer
---@return RootEntryInstance
function root_entry:at_least(n) end

---@class RootEntryInstance
local root_entry_instance = {}

---@class IntFieldInstance

---@param field IntField
---@return IntFieldInstance
function root_entry_instance:field(field) end

---@param field RefField
---@return SubEntry
function root_entry_instance:sub(field) end

---@class SubEntry
local sub_entry = {}

---@return SubEntryInstance
function sub_entry:any() end

---@return SubEntryInstance
function sub_entry:all() end

---@param n integer
---@return SubEntryInstance
function sub_entry:exactly(n) end

---@param n integer
---@return SubEntryInstance
function sub_entry:at_least(n) end

---@param n integer
---@return SubEntryInstance
function sub_entry:at_most(n) end

---@param min integer
---@param max integer
---@return SubEntryInstance
function sub_entry:in_range(min, max) end

---@class SubEntryInstance
local sub_entry_instance = {}
