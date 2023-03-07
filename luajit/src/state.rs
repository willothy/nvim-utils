use once_cell::unsync::OnceCell;

use crate::ffi::LuaState;

thread_local! {
    static LUA: OnceCell<*mut LuaState> = OnceCell::new();
}

pub unsafe fn init(state: *mut LuaState) {
    LUA.with(|lua| lua.set(state).unwrap_unchecked());
}

pub unsafe fn with_state<F, R>(f: F) -> R
where
    F: FnOnce(*mut LuaState) -> R,
{
    LUA.with(|lua| f(*lua.get().unwrap()))
}
