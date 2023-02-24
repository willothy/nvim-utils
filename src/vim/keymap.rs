//! Corresponds to `vim.keymap`

use crate::prelude::*;

/// Gets the `vim.keymap` table
pub fn get(lua: &Lua) -> LuaResult<LuaTable> {
    vim::get(lua)?.get::<_, LuaTable>("keymap")
}

/// Corresponds to `vim.keymap.set`
pub fn set(
    lua: &Lua,
    mode: LuaValue,
    lhs: LuaValue,
    rhs: LuaValue,
    opts: Option<LuaValue>,
) -> LuaResult<()> {
    self::get(lua)?.call_function("set", (mode, lhs, rhs, opts))
}

