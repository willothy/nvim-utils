use std::ffi::c_int;

use crate::ffi::LuaState;

pub type LuaCFunction = unsafe extern "C" fn(L: *mut LuaState) -> c_int;
pub type LuaNumber = f64;
pub type LuaInteger = isize;

pub struct LuaRef {
    index: c_int,
    drop: bool,
}
