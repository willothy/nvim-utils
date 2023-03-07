//! Corresponds to `vim.fn`
use luajit::{cstr::AsCStr, pop::Pop, push::Push};
use nvim_utils_luajit as luajit;
use std::path::PathBuf;

use crate::prelude::*;

/// Gets the `vim.fn` table
pub fn get(lua: &Lua) -> LuaResult<LuaTable> {
    vim::get(lua)?.get::<_, LuaTable>("fn")
}

/// Corresponds to `vim.fn.stdpath`
pub fn stdpath(lua: &Lua, path: &str) -> LuaResult<String> {
    self::get(lua)?.get::<_, LuaFunction>("stdpath")?.call(path)
}

/// Corresponds to `vim.fn.tmpname`
pub fn tmpname(lua: &Lua) -> LuaResult<String> {
    self::get(lua)?.get::<_, LuaFunction>("tempname")?.call(())
}

/// Corresponds to `vim.fn.line`
pub fn line(line: &str) -> Result<usize, anyhow::Error> {
    unsafe {
        luajit::state::with_state(|state| -> Result<usize, anyhow::Error> {
            line.push(state)?;
            luajit::ffi::lua_getglobal(state, "line\0".as_cstr());
            luajit::ffi::lua_getfield(state, -1, "fn\0".as_cstr());
            luajit::ffi::lua_getfield(state, -1, "line\0".as_cstr());
            luajit::ffi::lua_call(state, 1, 1);
            let res = usize::pop(state)?;
            luajit::ffi::lua_pop(state, 2);
            Ok(res)
        })
    }
}

/// Corresponds to `vim.fn.foldclosedend`
pub fn foldclosedend(lua: &Lua, line: u64) -> LuaResult<i64> {
    self::get(lua)?
        .get::<_, LuaFunction>("foldclosedend")?
        .call(line)
}

/// Corresponds to `vim.fn.foldclosed`
pub fn foldclosed(lua: &Lua, line: u64) -> LuaResult<i64> {
    vim::func::get(lua)?.call_function("foldclosed", line)
}

/// Corresponds to `vim.fn.indent`
pub fn indent(lua: &Lua, line: u64) -> LuaResult<u64> {
    self::get(lua)?.get::<_, LuaFunction>("indent")?.call(line)
}

/// Corresponds to `vim.fn.shiftwidth`
pub fn shiftwidth(lua: &Lua) -> LuaResult<u64> {
    self::get(lua)?
        .get::<_, LuaFunction>("shiftwidth")?
        .call(())
}

/// The result of a GetLine call, either a string or a table (array) of strings
/// Can be used with match, or if you know the type you can use `into_string` or `into_table`
#[derive(Debug, Clone)]
pub enum GetLineResult {
    String(String),
    Table(Vec<String>),
}

impl<'a> FromLua<'a> for GetLineResult {
    fn from_lua(val: LuaValue<'a>, lua: &'a Lua) -> LuaResult<Self> {
        match &val {
            LuaValue::String(_) => Ok(GetLineResult::String(lua.unpack(val)?)),
            LuaValue::Table(_) => Ok(GetLineResult::Table(lua.unpack(val)?)),
            _ => Err(LuaError::FromLuaConversionError {
                from: "LuaValue",
                to: "GetLineResult",
                message: Some("Invalid type".to_string()),
            }),
        }
    }
}

impl GetLineResult {
    pub fn into_string(self) -> LuaResult<String> {
        match self {
            GetLineResult::String(s) => Ok(s),
            _ => Err(LuaError::FromLuaConversionError {
                from: "LuaValue",
                to: "GetLineResult",
                message: Some("Invalid type".to_string()),
            }),
        }
    }

    pub fn into_table(self) -> LuaResult<Vec<String>> {
        match self {
            GetLineResult::Table(t) => Ok(t),
            _ => Err(LuaError::FromLuaConversionError {
                from: "LuaValue",
                to: "GetLineResult",
                message: Some("Invalid type".to_string()),
            }),
        }
    }
}

/// Corresponds to `vim.fn.getline`
pub fn getline(lua: &Lua, line: u64, end: Option<u64>) -> LuaResult<GetLineResult> {
    let val = if let Some(end) = end {
        self::get(lua)?.call_function("getline", (line, end))?
    } else {
        self::get(lua)?
            .get::<_, LuaFunction>("getline")?
            .call(line)?
    };
    match &val {
        LuaValue::String(_) => Ok(GetLineResult::String(String::from_lua(val, lua)?)),
        LuaValue::Table(_) => Ok(GetLineResult::Table(lua.unpack(val)?)),
        _ => Err(LuaError::FromLuaConversionError {
            from: "LuaValue",
            to: "GetLineResult",
            message: Some("Invalid type".to_string()),
        }),
    }
}

/// Corresponds to `vim.fn.setline`
pub fn setline(lua: &Lua, line: u64, text: &str) -> LuaResult<bool> {
    self::get(lua)?
        .get::<_, LuaFunction>("setline")?
        .call((line, text))
}

/// Corresponds to `vim.fn.getcwd`
pub fn getcwd(lua: &Lua, win_num: Option<u64>, tab_num: Option<u64>) -> LuaResult<PathBuf> {
    let cwd = self::get(lua)?
        .call_function::<_, _, String>("getcwd", (win_num.unwrap_or(0), tab_num.unwrap_or(0)))?;
    Ok(PathBuf::from(cwd))
}

/// Corresponds to `vim.fn.mode`
pub fn mode(lua: &Lua) -> LuaResult<String> {
    vim::func::get(lua)?.call_function("mode", ())
}
