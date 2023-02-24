//! Utilities that don't directly map to the Neovim API but make use of it or extend it

use crate::prelude::*;
use std::path::PathBuf;

/// Creats a session at the given path using `mksession!`
///
/// ## Example
/// ```rust
/// use nvim_utils::prelude::*;
/// use std::path::PathBuf;
///
/// fn my_module(lua: &Lua) -> LuaResult<()> {
///     vim::ext::mksession(lua, PathBuf::from("~/.sessions/session.vim"))?;
///     Ok(())
/// }
/// ```
pub fn mksession<'a>(lua: &Lua, path: PathBuf) -> LuaResult<()> {
    vim::cmd(
        lua,
        &format!("mksession! {}", String::from(path.to_string_lossy())),
    )
}

pub mod log {
    //! Utility functions for calling `vim.notify` with different log levels
    use crate::prelude::*;

    /// Calls notify with log level `Info`
    pub fn info(lua: &Lua, msg: &str) -> LuaResult<()> {
        vim::notify(lua, msg, vim::log::LogLevel::Info)
    }

    /// Calls notify with log level `Warn`
    pub fn warn(lua: &Lua, msg: &str) -> LuaResult<()> {
        vim::notify(lua, msg, vim::log::LogLevel::Warn)
    }

    /// Calls notify with log level `Error`
    pub fn error(lua: &Lua, msg: &str) -> LuaResult<()> {
        vim::notify(lua, msg, vim::log::LogLevel::Error)
    }

    /// Calls notify with log level `Trace`
    pub fn trace(lua: &Lua, msg: &str) -> LuaResult<()> {
        vim::notify(lua, msg, vim::log::LogLevel::Trace)
    }

    /// Calls notify with log level `Debug`
    pub fn debug(lua: &Lua, msg: &str) -> LuaResult<()> {
        vim::notify(lua, msg, vim::log::LogLevel::Debug)
    }
}
