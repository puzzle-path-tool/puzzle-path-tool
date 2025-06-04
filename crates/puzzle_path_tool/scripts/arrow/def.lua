---@meta

---@class Field

---@class IntField: Field

IntField = {}

---@return IntField
function IntField.new() end


---@class IdField: Field

IdField = {}

---@return IdField
function IdField.new() end


---@class CharField: Field

CharField = {}

---@return CharField
function CharField.new() end


---@class FloatField: Field

FloatField = {}

---@return FloatField
function FloatField.new() end


---@class BoolField: Field

BoolField = {}

---@return BoolField
function BoolField.new() end


---@class RefField: Field

RefField = {}

---@param id IdField
---@return RefField
function RefField.new(id) end

---@class RootEntry
local root_entry = {}

RootEntry = {}

---@param id IdField
---@return RootEntry
function RootEntry.new(id) end

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
