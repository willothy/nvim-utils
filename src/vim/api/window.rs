use crate::prelude::*;

pub fn nvim_win_get_cursor(lua: &Lua, window: u64) -> LuaResult<LuaTable> {
    vim::api::get(lua)?.call_function("nvim_win_get_cursor", window)
}
