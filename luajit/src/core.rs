use std::ffi::c_char;
use std::panic::PanicInfo;

use crate::ffi::{lua_error, lua_pushlstring};

fn panic_handler<'a, 'b>(info: &'a PanicInfo<'b>) {
    unsafe {
        crate::state::with_state(|state| {
            let msg = format!("{:?}", info.payload());
            lua_pushlstring(state, msg.as_ptr() as *const c_char, msg.len());
            lua_error(state);
        });
    }
}

pub(crate) fn setup_panic_handler() {
    std::panic::set_hook(Box::new(panic_handler));
}
