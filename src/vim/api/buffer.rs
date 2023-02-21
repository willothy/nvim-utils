use crate::prelude::*;

pub fn nvim_buf_attach(lua: &Lua, buffer: u64, send_buffer: bool, opts: LuaTable) -> LuaResult<()> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_buf_attach")?
        .call((buffer, send_buffer, opts))
}

pub fn nvim_buf_detach(lua: &Lua, buffer: u64) -> LuaResult<()> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_buf_detach")?
        .call(buffer)
}

pub fn nvim_buf_set_lines(
    lua: &Lua,
    buffer: LuaInteger,
    start: LuaInteger,
    end: LuaInteger,
    strict_indexing: bool,
    lines: Vec<String>,
) -> LuaResult<()> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_buf_set_lines")?
        .call((buffer, start, end, strict_indexing, lines))
}
