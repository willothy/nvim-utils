use crate::prelude::*;

pub fn nvim_win_get_cursor(lua: &Lua, window: u64) -> LuaResult<LuaTable> {
    vim::api::get(lua)?.call_function("nvim_win_get_cursor", window)
}

pub fn nvim_win_set_cursor(lua: &Lua, window: u64, pos: (u64, u64)) -> LuaResult<()> {
    vim::api::get(lua)?.call_function("nvim_win_set_cursor", (window, [pos.0, pos.1]))
}
