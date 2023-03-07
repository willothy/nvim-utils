use crate::ffi::{lua_error, lua_pushlstring, LuaState};
use std::ffi::c_char;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Error popping {0}: found {1}")]
    PopUnexpectedType(&'static str, String),
    #[error("Underflowed stack while attempting to pop {0}")]
    StackUnderflow(&'static str),
    #[error("{0}")]
    External(anyhow::Error),
}

pub unsafe fn handle(state: *mut LuaState, e: &dyn std::error::Error) -> ! {
    let m = e.to_string();
    lua_pushlstring(state, m.as_ptr() as *const c_char, m.len());
    lua_error(state)
}
