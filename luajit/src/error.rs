use std::ffi::c_char;

use crate::ffi::{lua_error, lua_pushlstring, LuaState};

pub unsafe fn handle(state: *mut LuaState, e: &dyn std::error::Error) -> ! {
    let m = e.to_string();
    lua_pushlstring(state, m.as_ptr() as *const c_char, m.len());
    lua_error(state)
}
