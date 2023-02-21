//! Corresponds to `vim.fn`
use crate::prelude::*;

pub fn get(lua: &Lua) -> LuaResult<LuaTable> {
    vim::get(lua)?.get::<_, LuaTable>("fn")
}

pub fn stdpath(lua: &Lua, path: &str) -> LuaResult<String> {
    self::get(lua)?.get::<_, LuaFunction>("stdpath")?.call(path)
}

pub fn tmpname(lua: &Lua) -> LuaResult<String> {
    self::get(lua)?.get::<_, LuaFunction>("tempname")?.call(())
}
