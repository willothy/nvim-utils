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

fn get_mode(lua: &Lua) -> Result<(), String> {
    Ok(())
}

tests! {
    "Get Mode": get_mode
}

const TEST_SEP: &str = "\n--------------------------\n";

fn run_tests(lua: &Lua) -> Result<(), Vec<(&'static str, String)>> {
    let results = tests
        .iter()
        .filter_map(|test| test.run(lua).err())
        .collect::<Vec<_>>();
    if results.is_empty() {
        Ok(())
    } else {
        Err(results)
    }
}

/// The #[lua_module] attribute generates an entry point for the plugin.
#[mlua::lua_module]
pub fn test_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    if let Err(e) = run_tests(lua) {
        let msg = e
            .iter()
            .map(|(n, e)| format!("{}: {}", n, e))
            .collect::<Vec<_>>()
            .join(TEST_SEP);
        assert!(false, "API Tests failed:\n{}", msg)
    }

    // Create a new module builder
    ModuleBuilder::new(lua)
        // Add the hello function to the module
        .with_fn("hello", hello)?
        // Add the get_plugin_info function to the module
        .with_fn("get_plugin_info", get_plugin_info)?
        // Build the module
        .build()
}

struct Test {
    runner: fn(&Lua) -> Result<(), String>,
    name: &'static str,
}

impl Test {
    fn run(&self, lua: &Lua) -> Result<(), (&'static str, String)> {
        if let Err(e) = (self.runner)(lua) {
            return Err((self.name, e));
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! tests {
    ($($name:literal: $r:expr)*) => {
		static tests: [Test; count!($($name)*)] = [
			$(Test {
				name: $name,
				runner: $r
			}),*
		];
	};
}
