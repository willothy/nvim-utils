#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Corresponds to `vim.api.nvim_get_current_buf`
pub fn nvim_get_current_buf(lua: &Lua) -> LuaResult<i64> {
    vim::api::get(lua)?.call_function("nvim_get_current_buf", ())
}

/// Corresponds to `vim.api.nvim_get_current_line`
pub fn nvim_get_current_line(lua: &Lua) -> LuaResult<String> {
    vim::api::get(lua)?.call_function("nvim_get_current_line", ())
}

/// Corresponds to `vim.api.nvim_get_current_tabpage`
pub fn nvim_get_current_tabpage(lua: &Lua) -> LuaResult<i64> {
    vim::api::get(lua)?.call_function("nvim_get_current_tabpage", ())
}

/// Corresponds to `vim.api.nvim_get_current_win`
pub fn nvim_get_current_win(lua: &Lua) -> LuaResult<i64> {
    vim::api::get(lua)?.call_function("nvim_get_current_win", ())
}

/// Corresponds to `vim.api.nvim_list_bufs`
pub fn nvim_list_bufs(lua: &Lua) -> LuaResult<Vec<LuaInteger>> {
    vim::api::get(lua)?.call_function("nvim_list_bufs", ())
}

/// Corresponds to `vim.api.nvim_exec`
pub fn nvim_exec<'a>(lua: &'a Lua, cmd: &str, output: bool) -> LuaResult<LuaValue<'a>> {
    vim::api::get(lua)?.call_function("nvim_exec", (cmd, output))
}

/// Corresponds to `vim.api.nvim_feedkeys`
pub fn nvim_feedkeys(lua: &Lua, keys: &str, mode: &str, escape_ks: bool) -> LuaResult<()> {
    vim::api::get(lua)?.call_function("nvim_feedkeys", (keys, mode, escape_ks))
}

/// Result struct for `vim.api.nvim_get_mode`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
#[derive(Clone, Deserialize)]
pub struct GetModeRes {
    pub mode: String,
    pub blocking: bool,
}

/// Corresponds to `vim.api.nvim_get_mode`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_get_mode(lua: &Lua) -> LuaResult<GetModeRes> {
    lua.from_value(vim::api::get(lua)?.call_function("nvim_get_mode", ())?)
}

/// Corresponds to `vim.api.nvim_stats`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_stats(lua: &Lua) -> LuaResult<LuaTable> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_stats")?
        .call(())
}

/// Corresponds to `vim.api.nvim_chan_send`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_chan_send(lua: &Lua, chan: i32, data: &str) -> LuaResult<()> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_chan_send")?
        .call((chan, data))
}

/// Corresponds to `vim.api.nvim_create_buf`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_create_buf(lua: &Lua, listed: bool, scratch: bool) -> LuaResult<i64> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_create_buf")?
        .call((listed, scratch))
}

/// Corresponds to `vim.api.nvim_del_current_line`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_del_current_line(lua: &Lua) -> LuaResult<()> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_del_current_line")?
        .call(())
}

/// Corresponds to `vim.api.nvim_del_keymap`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_del_keymap(lua: &Lua, mode: &str, lhs: &str) -> LuaResult<()> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_del_keymap")?
        .call((mode, lhs))
}

/// Corresponds to `vim.api.nvim_del_mark`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_del_mark(lua: &Lua, name: &str) -> LuaResult<bool> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_del_mark")?
        .call(name)
}

/// Corresponds to `vim.api.nvim_del_var`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_del_var(lua: &Lua, name: &str) -> LuaResult<()> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_del_var")?
        .call(name)
}

/// Corresponds to `vim.api.nvim_echo`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_echo(
    lua: &Lua,
    chunks: LuaTable,
    history: bool,
    opts: Option<LuaTable>,
) -> LuaResult<()> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_echo")?
        .call((chunks, history, opts))
}

/// Corresponds to `vim.api.nvim_err_write`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_err_write(lua: &Lua, msg: &str) -> LuaResult<()> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_err_write")?
        .call(msg)
}

/// Corresponds to `vim.api.nvim_err_writeln`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_err_writeln(lua: &Lua, msg: &str) -> LuaResult<()> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_err_writeln")?
        .call(msg)
}

/// Options for `nvim_eval_statusline`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
#[derive(Debug)]
pub struct EvalStatuslineOpt {
    winid: LuaInteger,
    maxwidth: LuaInteger,
    fillchar: String,
    highlights: bool,
    use_winbar: bool,
    use_tabline: bool,
}

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
impl<'a> ToLua<'a> for EvalStatuslineOpt {
    fn to_lua(self, lua: &'a Lua) -> LuaResult<LuaValue<'a>> {
        let table = lua.create_table()?;
        table.set("winid", self.winid)?;
        table.set("maxwidth", self.maxwidth)?;
        table.set("fillchar", self.fillchar)?;
        table.set("highlights", self.highlights)?;
        table.set("use_winbar", self.use_winbar)?;
        table.set("use_tabline", self.use_tabline)?;
        lua.pack(table)
    }
}

/// Result of `nvim_eval_statusline`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
#[derive(Debug)]
pub struct EvalStatuslineRes {
    pub str: String,
    pub width: LuaInteger,
    pub highlights: Vec<HighlightInfo>,
}

/// Highlight info for `nvim_eval_statusline`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
#[derive(Debug)]
pub struct HighlightInfo {
    pub start: LuaInteger,
    pub group: String,
}

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
impl<'a> FromLua<'a> for HighlightInfo {
    fn from_lua(lua_value: LuaValue<'a>, _lua: &'a Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Table(table) => {
                let start = table.get::<_, LuaInteger>("start")?;
                let group = table.get::<_, String>("group")?;
                Ok(HighlightInfo { start, group })
            }
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: "LuaValue",
                    to: "HighlightInfo",
                    message: Some("Expected LuaValue::Table".to_string()),
                })
            }
        }
    }
}

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
impl<'a> FromLua<'a> for EvalStatuslineRes {
    fn from_lua(lua_value: LuaValue<'a>, _lua: &'a Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Table(table) => {
                let str = table.get::<_, String>("str")?;
                let width = table.get::<_, LuaInteger>("width")?;
                let highlights = table.get::<_, Vec<HighlightInfo>>("highlights")?;
                Ok(EvalStatuslineRes {
                    str,
                    width,
                    highlights,
                })
            }
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: "LuaValue",
                    to: "EvalStatuslineRes",
                    message: Some("Expected LuaValue::Table".to_string()),
                })
            }
        }
    }
}

/// Corresponds to `vim.api.nvim_eval_statusline`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_eval_statusline<'a>(
    lua: &'a Lua,
    expr: &str,
    opt: Option<EvalStatuslineOpt>,
) -> LuaResult<LuaTable<'a>> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_eval_statusline")?
        .call((expr, opt))
}

/// Corresponds to `vim.api.nvim_exec_lua`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_exec_lua<'a>(
    lua: &'a Lua,
    code: &str,
    args: Option<LuaTable<'a>>,
) -> LuaResult<LuaValue<'a>> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_exec_lua")?
        .call((code, args))
}

/// Corresponds to `vim.api.nvim_get_api_info`
// TODO: return type
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_get_api_info(lua: &Lua) -> LuaResult<(LuaInteger, LuaTable)> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_get_api_info")?
        .call(())
}

/// Info struct for `nvim_get_chan_info`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
#[derive(Debug)]
pub struct ChannelInfo<'a> {
    pub id: LuaInteger,
    pub stream: String,
    pub mode: String,
    pub pty: Option<String>,
    pub argv: Vec<String>,
    pub buffer: Option<String>,
    pub client: Option<LuaTable<'a>>,
}

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
impl<'a> FromLua<'a> for ChannelInfo<'a> {
    fn from_lua(lua_value: LuaValue<'a>, _lua: &'a Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Table(table) => {
                let id = table.get::<_, LuaInteger>("id")?;
                let stream = table.get::<_, String>("stream")?;
                let mode = table.get::<_, String>("mode")?;
                let pty = table.get::<_, Option<String>>("pty")?;
                let argv = table.get::<_, Vec<String>>("argv")?;
                let buffer = table.get::<_, Option<String>>("buffer")?;
                let client = table.get::<_, Option<LuaTable<'a>>>("client")?;
                Ok(ChannelInfo {
                    id,
                    stream,
                    mode,
                    pty,
                    argv,
                    buffer,
                    client,
                })
            }
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: "LuaValue",
                    to: "ChannelInfo",
                    message: Some("Expected LuaValue::Table".to_string()),
                })
            }
        }
    }
}

/// Corresponds to `vim.api.nvim_get_chan_info`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_get_chan_info<'a>(lua: &'a Lua, chan: LuaInteger) -> LuaResult<ChannelInfo<'a>> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_get_chan_info")?
        .call(chan)
}

/// Corresponds to `vim.api.nvim_get_color_by_name`
/// Returns 24-bit RGB value or -1 for invalid color.
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_get_color_by_name(lua: &Lua, name: &str) -> LuaResult<i64> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_get_color_by_name")?
        .call(name)
}

/// Corresponds to `vim.api.nvim_get_color_map`
// TODO: return type
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_get_color_map(lua: &Lua) -> LuaResult<LuaTable> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_get_color_map")?
        .call(())
}

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub type GetContextOpt = Vec<String>;

/// Corresponds to `vim.api.nvim_get_context`
// TODO: return type
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_get_context(lua: &Lua, opt: Option<GetContextOpt>) -> LuaResult<LuaTable> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_get_context")?
        .call(opt)
}

/// Corresponds to `vim.api.nvim_get_hl_by_id`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_get_hl_by_id<'a>(lua: &'a Lua, id: LuaInteger, rgb: bool) -> LuaResult<LuaTable<'a>> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_get_hl_by_id")?
        .call((id, rgb))
}

/// Corresponds to `vim.api.nvim_get_hl_by_name`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_get_hl_by_name<'a>(lua: &'a Lua, name: &str, rgb: bool) -> LuaResult<LuaTable<'a>> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_get_hl_by_name")?
        .call((name, rgb))
}

/// Corresponds to `vim.api.nvim_get_hl_id_by_name`
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_get_hl_id_by_name(lua: &Lua, name: &str) -> LuaResult<i64> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_get_hl_id_by_name")?
        .call(name)
}

#[derive(Debug)]
#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub struct Mapping<'a> {
    pub buffer: LuaInteger,
    pub expr: LuaValue<'a>,
    pub lhs: String,
    pub lhsraw: Vec<u8>,
    pub lnum: LuaInteger,
    pub mode: String,
    pub noremap: bool,
    pub nowait: bool,
    pub script: LuaInteger,
    pub sid: LuaInteger,
    pub silent: bool,
}

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
impl<'a> FromLua<'a> for Mapping<'a> {
    fn from_lua(lua_value: LuaValue<'a>, _lua: &'a Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Table(table) => {
                let buffer = table.get::<_, LuaInteger>("buffer")?;
                let expr = table.get::<_, LuaValue>("expr")?;
                let lhs = table.get::<_, String>("lhs")?;
                let lhsraw = table.get::<_, Vec<u8>>("lhsraw")?;
                let lnum = table.get::<_, LuaInteger>("lnum")?;
                let mode = table.get::<_, String>("mode")?;
                let noremap = table.get::<_, bool>("noremap")?;
                let nowait = table.get::<_, bool>("nowait")?;
                let script = table.get::<_, LuaInteger>("script")?;
                let sid = table.get::<_, LuaInteger>("sid")?;
                let silent = table.get::<_, bool>("silent")?;
                Ok(Mapping {
                    buffer,
                    expr,
                    lhs,
                    lhsraw,
                    lnum,
                    mode,
                    noremap,
                    nowait,
                    script,
                    sid,
                    silent,
                })
            }
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: "LuaValue",
                    to: "Mapping",
                    message: Some("Expected LuaValue::Table".to_string()),
                })
            }
        }
    }
}

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_get_keymap<'a>(lua: &'a Lua, mode: &str) -> LuaResult<Vec<Mapping<'a>>> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_get_keymap")?
        .call(mode)
}

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_get_mark(
    lua: &Lua,
    name: &str,
) -> LuaResult<(LuaInteger, LuaInteger, LuaInteger, String)> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_get_mark")?
        .call(name)
}

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
#[derive(Debug)]
pub struct GetModeRes {
    pub blocking: bool,
    pub mode: String,
}

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
impl<'a> FromLua<'a> for GetModeRes {
    fn from_lua(lua_value: LuaValue<'a>, _lua: &'a Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Table(table) => {
                let blocking = table.get::<_, bool>("blocking")?;
                let mode = table.get::<_, String>("mode")?;
                Ok(GetModeRes { blocking, mode })
            }
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: "LuaValue",
                    to: "GetModeRes",
                    message: Some("Expected LuaValue::Table".to_string()),
                })
            }
        }
    }
}

#[cfg(feature = "unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
pub fn nvim_get_proc<'a>(lua: &'a Lua, pid: LuaInteger) -> LuaResult<LuaValue<'a>> {
    vim::api::get(lua)?
        .get::<_, LuaFunction>("nvim_get_proc")?
        .call(pid)
}
