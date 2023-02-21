//! Corresponds to `vim.keymap`

use crate::prelude::*;

pub fn get(lua: &Lua) -> LuaResult<LuaTable> {
    vim::get(lua)?.get::<_, LuaTable>("keymap")
}

pub fn set(lua: &Lua, mode: LuaValue, lhs: LuaValue, rhs: LuaValue, opts: Option<LuaValue>) -> LuaResult<()> {
    self::get(lua)?.call_function("set", (mode, lhs, rhs, opts))
}