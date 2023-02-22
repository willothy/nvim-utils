use nvim_utils::prelude::*;

/// Functions exported to Lua take two arguments: a reference to the Lua state and the arguments passed to the function (tuple, iterator or expr).
/// See https://docs.rs/mlua/ for more information on how to use mlua
fn hello(lua: &Lua, _args: ()) -> LuaResult<()> {
    log::info(lua, "Hello from Rust!")?;
    Ok(())
}

fn get_plugin_info(lua: &Lua, _args: ()) -> LuaResult<LuaTable> {
    ModuleBuilder::new(lua)
        .with_string("name", "example")?
        .with_string("version", "0.1.0")?
        .with_string("author", "Example Author")?
        .with_string("description", "Example plugin for nvim-utils")?
        .with_string("license", "MIT")?
        .with_table_from("dependencies", [("nvim_utils", "0.1.0"), ("mlua", "0.8.7")])?
        .build()
}

/// The #[lua_module] attribute generates an entry point for the plugin.
#[mlua::lua_module]
pub fn example(lua: &Lua) -> LuaResult<LuaTable> {
    // Create a new module builder
    ModuleBuilder::new(lua)
        // Add the hello function to the module
        .with_fn("hello", hello)?
        // Add the get_plugin_info function to the module
        .with_fn("get_plugin_info", get_plugin_info)?
        // Build the module
        .build()
}
