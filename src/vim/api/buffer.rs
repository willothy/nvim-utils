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
pub fn nvim_buf_set_lines<S: Into<String>>(
    lua: &Lua,
    buffer: u64,
    start: u64,
    end: u64,
    strict_indexing: bool,
    lines: Vec<S>,
) -> LuaResult<()> {
    vim::api::get(lua)?.call_function(
        "nvim_buf_set_lines",
        (
            buffer,
            start,
            end,
            strict_indexing,
            lines.into_iter().map(|s| s.into()).collect::<Vec<_>>(),
        ),
    )
}

/// Corresponds to `vim.api.nvim_buf_get_lines`
pub fn nvim_buf_get_lines(
    lua: &Lua,
    buffer: u64,
    start: u64,
    end: u64,
    strict_indexing: bool,
) -> LuaResult<Vec<String>> {
    vim::api::get(lua)?.call_function("nvim_buf_get_lines", (buffer, start, end, strict_indexing))
}
