use std::ffi::c_int;

use crate::ffi::{lua_gettop, lua_settop, LuaState};

pub struct StackGuard {
    state: *mut LuaState,
    start: c_int,
}

impl StackGuard {
    #[inline(always)]
    pub fn new(state: *mut LuaState) -> Self {
        Self {
            state,
            start: unsafe { lua_gettop(state) },
        }
    }

    #[inline(always)]
    pub fn pop(state: *mut LuaState) -> Self {
        let ssize = unsafe { lua_gettop(state) };

        if ssize == 0 {
            panic!("Rust underflowed the Lua stack!");
        }

        Self {
            state,
            start: ssize,
        }
    }
}

impl Drop for StackGuard {
    fn drop(&mut self) {
        unsafe {
            let ssize = lua_gettop(self.state) - self.start;
            if ssize > self.start {
                lua_settop(self.state, self.start);
            } else if ssize < self.start {
                panic!(
                    "Popped too many values from stack: Expected size {}, found {}",
                    self.start, ssize
                );
            }
        }
    }
}

macro_rules! stackguard {
    ($state:expr) => {
        let _guard = StackGuard::new($state);
    };
}

macro_rules! popguard {
    ($state:expr) => {
        let _guard = StackGuard::pop($state);
    };
}

pub(crate) use popguard;
pub(crate) use stackguard;
