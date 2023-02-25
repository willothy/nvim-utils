#![cfg_attr(docsrs, feature(doc_cfg))]
//! # Utilities for building Neovim plugins in Rust<br>
//! The intention of `nvim-utils` is to allow Rust plugin authors to interact with Neovim in Rust just as easily as in Lua,<br>
//! by removing as much of the required boilerplate as possible.
//!
//! It provides utilities for:
//! - Declaratively building lua modules using `mlua`
//! - Interacting with Neovim's lua api
//! - Logging using `vim.notify`
//! - Accessing common lua builtin functions like `require` and `print`
//! - And more to come!
//!
//! #### Features
//! - `builder` enables the [`builder`] module, containing [`ModuleBuilder`](struct@builder) (enabled by default)
//! - `vim` enables the [`vim`] module (enabled by default)
//! - `async` enables async functions in [`builder::ModuleBuilder`], and the `async` feature in mlua (disabled by default)
//! - `send` enables the `send` feature for [`mlua`], which enables `Send` for lua types (disabled by default)
//! - `unstable` includes unstable / untested API features (disabled by default)

/// Includes [`mlua::prelude`], [`vim`], [`vim::ext::log`], and [`builder::ModuleBuilder`] if the corresponding features are enabled
pub mod prelude {
    #[cfg(feature = "vim")]
    #[cfg_attr(docsrs, doc(cfg(feature = "vim")))]
    pub use crate::vim;

    #[cfg(feature = "vim")]
    #[cfg_attr(docsrs, doc(cfg(feature = "vim")))]
    pub use crate::vim::ext::log;

    #[cfg(feature = "builder")]
    #[cfg_attr(docsrs, doc(cfg(feature = "builder")))]
    pub use crate::builder::ModuleBuilder;

    pub use mlua::serde::{Deserializer, LuaSerdeExt, Serializer};

    pub use mlua::lua_module;
    pub use mlua::prelude::*;
}

#[allow(unused_imports)]
#[macro_use]
extern crate nvim_utils_macros;
pub use nvim_utils_macros::module;

#[cfg(feature = "builder")]
#[cfg_attr(docsrs, doc(cfg(feature = "builder")))]
pub mod builder;
#[cfg(feature = "vim")]
#[cfg_attr(docsrs, doc(cfg(feature = "vim")))]
pub mod vim;

use prelude::*;

// TODO: figure out how to cache the result of gets for commonly used functions like require so they don't need to be repeatedly fetched

/// Gets the global `require` function and calls it with the given name as an argument
pub fn require<'a, T: FromLua<'a> + Clone>(lua: &'a Lua, name: &'a str) -> LuaResult<T> {
    lua.globals()
        .get::<_, LuaFunction<'a>>("require")?
        .call(name)
}

/// Gets the global `print` function and calls it with the given message as an argument
pub fn print(lua: &Lua, msg: String) -> LuaResult<()> {
    lua.globals()
        .get::<_, LuaFunction>("print")?
        .call::<_, ()>(msg)
}
