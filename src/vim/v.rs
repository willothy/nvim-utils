use crate::prelude::*;

pub fn get(lua: &Lua) -> LuaResult<LuaTable> {
    vim::get(lua)?.get::<_, LuaTable>("v")
}

pub fn count(lua: &Lua) -> LuaResult<u64> {
    self::get(lua)?.get::<_, u64>("count")
}
