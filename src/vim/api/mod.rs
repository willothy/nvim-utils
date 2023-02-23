use crate::prelude::*;

mod global;
pub use global::*;

mod buffer;
pub use buffer::*;

mod window;
pub use window::*;

pub fn get(lua: &Lua) -> LuaResult<LuaTable> {
    vim::get(lua)?.get::<_, LuaTable>("api")
}
