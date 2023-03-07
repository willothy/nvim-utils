macro_rules! count {
    () => {0i32};
    ($x:tt $($xs:tt)*) => {1i32 + count!($($xs)*)};
}

use std::ffi::c_char;

pub(crate) use count;

use crate::error::Error;
use crate::ffi::{lua_gettop, lua_settop, LuaState};

pub(crate) unsafe fn check_stack(state: *mut LuaState, ctx: &'static str) -> Result<(), Error> {
    if lua_gettop(state) == 0 {
        Err(Error::StackUnderflow(ctx))
    } else {
        Ok(())
    }
}

pub(crate) unsafe fn set_ssize(state: *mut LuaState, s: i32) {
    if lua_gettop(state) < s {
        lua_settop(state, s);
    }
}
