//! # Semi-direct mapping of the `vim` module in Neovim's lua api
//!
//! The goal for this module is to provide an idiomatic way to call Neovim's lua api from Rust, without needing to repeat the boilerplate for loading specific functions.
//!
//! Notable differences from the lua api:
//! - `vim.fn` has been renamed to `vim::func`, since fn is a keyword in Rust
//! - Added `vim::ext` for functions that don't directly map to the Neovim api but make use of it or extend it
//! - Not all functions are implemented yet

pub mod ext;
pub mod func;
pub mod keymap;
pub mod log;

use crate::prelude::*;

/// Get global `vim`
///
/// ## Example
/// ```rust
/// use nvim_utils::prelude::*;
/// fn my_module(lua: &mlua::prelude::Lua) -> mlua::prelude::LuaResult<()> {
///     let global_vim = vim::get(lua)?;
///     let vim_version: LuaTable = global_vim.call_function("version", ())?;
///     println!("Vim version: {}", vim::inspect(lua, vim_version)?);
///     Ok(())
/// }
/// ```
pub fn get(lua: &Lua) -> LuaResult<LuaTable> {
    lua.globals().get("vim")
}

/// Corresponds to `vim.cmd()`
///
/// ## Example
/// ```rust
/// use nvim_utils::prelude::*;
/// fn my_module(lua: &mlua::prelude::Lua) -> mlua::prelude::LuaResult<()> {
///     vim::cmd(lua, "echo 'Hello, world!'")?;
///     vim::cmd(lua, "terminal")
/// }
/// ```
pub fn cmd(lua: &Lua, cmd: &str) -> LuaResult<()> {
    self::get(lua)?.call_function("cmd", cmd)
}

/// Corresponds to `vim.inspect()`
///
/// ## Example
/// ```rust
/// use nvim_utils::prelude::*;
/// fn my_module(lua: &Lua) -> LuaResult<()> {
///     let table = lua.create_table()?;
///     table.set("foo", "bar")?;
///     let inspect = vim::inspect(lua, table)?;
///     Ok(())
/// }
/// ```
pub fn inspect<'a>(lua: &'a Lua, value: impl ToLua<'a>) -> LuaResult<String> {
    self::get(lua)?.call_function("inspect", value)
}

/// Corresponds to `vim.notify()`
///
/// ## Example
/// ```rust
/// use nvim_utils::prelude::*;
/// fn my_module(lua: &mlua::prelude::Lua) -> mlua::prelude::LuaResult<()> {
///     vim::notify(lua, "Loaded module!", vim::log::LogLevel::Info)
/// }
/// ```
pub fn notify(lua: &Lua, msg: &str, log_level: log::LogLevel) -> LuaResult<()> {
    self::get(lua)?.call_function("notify", (msg, log_level as u8))
}
