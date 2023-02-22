use nvim_utils::prelude::*;

fn hello(lua: &Lua, _args: ()) -> LuaResult<()> {
    log::info(lua, "Hello from Rust and NeoVim!\n")?;
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
pub fn test_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    // Create a new module builder
    ModuleBuilder::new(lua)
        // Add the hello function to the module
        .with_fn("hello", hello)?
        // Add the get_plugin_info function to the module
        .with_fn("get_plugin_info", get_plugin_info)?
        // Build the module
        .build()
}
