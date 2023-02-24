//! Corresponds to `vim.fn`
use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub fn get(lua: &Lua) -> LuaResult<LuaTable> {
    vim::get(lua)?.get::<_, LuaTable>("fn")
}

pub fn stdpath(lua: &Lua, path: &str) -> LuaResult<String> {
    self::get(lua)?.get::<_, LuaFunction>("stdpath")?.call(path)
}

pub fn tmpname(lua: &Lua) -> LuaResult<String> {
    self::get(lua)?.get::<_, LuaFunction>("tempname")?.call(())
}

pub fn line(lua: &Lua, line: &str) -> LuaResult<u64> {
    self::get(lua)?.get::<_, LuaFunction>("line")?.call(line)
}

pub fn foldclosedend(lua: &Lua, line: u64) -> LuaResult<i64> {
    self::get(lua)?
        .get::<_, LuaFunction>("foldclosedend")?
        .call(line)
}

pub fn foldclosed(lua: &Lua, line: u64) -> LuaResult<i64> {
    vim::func::get(lua)?.call_function("foldclosed", line)
}

pub fn indent(lua: &Lua, line: u64) -> LuaResult<u64> {
    self::get(lua)?.get::<_, LuaFunction>("indent")?.call(line)
}

pub fn shiftwidth(lua: &Lua) -> LuaResult<u64> {
    self::get(lua)?
        .get::<_, LuaFunction>("shiftwidth")?
        .call(())
}

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

pub fn getline(lua: &Lua, line: u64, end: Option<u64>) -> LuaResult<GetLineResult> {
    let val = self::get(lua)?.call_function("getline", line)?;
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

pub fn setline(lua: &Lua, line: u64, text: &str) -> LuaResult<bool> {
    self::get(lua)?
        .get::<_, LuaFunction>("setline")?
        .call((line, text))
}
