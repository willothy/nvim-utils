use std::ffi::c_int;

use crate::ffi::LuaState;
use crate::push::Push;
use crate::{error, state};

#[doc(hidden)]
pub unsafe fn entry<R, E>(state: *mut LuaState, plugin: fn() -> Result<R, E>) -> c_int
where
    // TODO: R should have Push as bounds, not Error
    R: Push,
    E: std::error::Error,
{
    state::init(state);

    match plugin() {
        Ok(module) => module.push(state).unwrap(),
        Err(e) => error::handle(state, &e),
    }
}
