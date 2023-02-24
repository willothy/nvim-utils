use crate::prelude::*;

/// Corresponds to `vim.api.nvim_buf_attach`
// TODO: Change opts into a struct that implements `ToLua`
pub fn nvim_buf_attach(lua: &Lua, buffer: u64, send_buffer: bool, opts: LuaTable) -> LuaResult<()> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_buf_attach")?
        .call((buffer, send_buffer, opts))
}

/// Corresponds to `vim.api.nvim_buf_detach`
pub fn nvim_buf_detach(lua: &Lua, buffer: u64) -> LuaResult<()> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_buf_detach")?
        .call(buffer)
}

/// Corresponds to `vim.api.nvim_buf_set_lines`
pub fn nvim_buf_set_lines(
    lua: &Lua,
    buffer: u64,
    start: u64,
    end: u64,
    strict_indexing: bool,
    lines: Vec<&str>,
) -> LuaResult<()> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_buf_set_lines")?
        .call((buffer, start, end, strict_indexing, lines))
}
