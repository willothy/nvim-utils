//! Builder pattern for creating lua modules in a readable, declarative way

use std::collections::HashMap;

use crate::prelude::*;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
use std::future::Future;

/// Wraps some of the boilerplate for building a lua module using `mlua` in a nice builder pattern.<br>
/// Includes functions for operating on a reference to the builder, as well as for consuming the builder.
///
/// # Examples
/// ```rust
/// use nvim_utils::prelude::*; // imports mlua::prelude::* and the `vim` module
/// use nvim_utils::builder::ModuleBuilder;
///
/// fn my_plugin(lua: &Lua) -> LuaResult<LuaTable> {
///     // A method that adds two numbers and pushes the result to a table
///     let add = |lua: &Lua, (this, a, b): (LuaTable, i32, i32)| -> LuaResult<()> {
///         let results = this.get::<_, LuaTable>("results")?;
///         results.push(a + b)?;
///         Ok(())
///     };
///
///     // Consuming the builder
///     let module = ModuleBuilder::new(lua)
///         .with_table_empty("results")?
///         .with_fn("add", add)?
///         .build()?;
///
///     // Using a mutable reference to the builder
///     let mut builder = ModuleBuilder::new(lua);
///     builder.add_table_empty("results")?;
///     builder.add_fn("add", add)?;
///     let module = builder.build()?;
///
///     // If you need to return a LuaValue instead of a LuaTable, you can use mlua's `to_lua` method instead of `build`
///     // let value = builder.to_lua(lua)?;
///
///     Ok(module)
/// }
/// ```
#[derive(Debug)]
pub struct ModuleBuilder<'a> {
    fields: HashMap<String, LuaValue<'a>>,
    lua: &'a Lua,
}

impl<'a> ModuleBuilder<'a> {
    /// Creates a new module builder
    pub fn new(lua: &'a Lua) -> Self {
        Self {
            fields: HashMap::new(),
            lua,
        }
    }

    fn check_collision(&self, name: &str) -> LuaResult<()> {
        if self.fields.contains_key(name) {
            Err(LuaError::RuntimeError(format!(
                "Module already contains a field named {}",
                name
            )))
        } else {
            Ok(())
        }
    }

    /// Produces an iterator over the fields in the builder
    pub fn fields(&'a self) -> impl Iterator<Item = (&String, &LuaValue)> {
        self.fields.iter()
    }

    /// Produces a mutable iterator over the fields in the builder
    pub fn fields_mut(&'a mut self) -> impl Iterator<Item = (&String, &mut LuaValue)> {
        self.fields.iter_mut()
    }

    /// Adds a function to the module
    pub fn add_fn<A, R, F>(&mut self, name: &str, func: F) -> LuaResult<&mut Self>
    where
        F: 'static + Send + Fn(&'a Lua, A) -> LuaResult<R>,
        A: FromLuaMulti<'a>,
        R: ToLuaMulti<'a>,
    {
        self.check_collision(name)?;
        let func = self.lua.create_function(func)?;
        self.fields.insert(name.to_owned(), self.lua.pack(func)?);
        Ok(self)
    }

    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    /// Adds an async function to the module
    pub fn add_fn_async<A, R, F, FR>(&mut self, name: &str, func: F) -> LuaResult<&mut Self>
    where
        F: 'static + Send + Fn(&'a Lua, A) -> FR,
        A: FromLuaMulti<'a>,
        R: ToLuaMulti<'a>,
        FR: 'a + Send + Future<Output = LuaResult<R>>,
    {
        self.check_collision(name)?;
        let func = self.lua.create_async_function(func)?;
        self.fields.insert(name.to_owned(), self.lua.pack(func)?);
        Ok(self)
    }

    /// Adds a C function to the module
    pub fn add_c_fn(&mut self, name: &str, func: mlua::lua_CFunction) -> LuaResult<&mut Self> {
        self.check_collision(name)?;
        unsafe {
            let func = self.lua.create_c_function(func)?;
            self.fields.insert(name.to_owned(), self.lua.pack(func)?);
        }
        Ok(self)
    }

    /// Adds a table to the module
    pub fn add_table(&mut self, name: &str, table: LuaTable<'a>) -> LuaResult<&mut Self> {
        self.check_collision(name)?;
        self.fields.insert(name.to_owned(), self.lua.pack(table)?);
        Ok(self)
    }

    /// Adds a table to the module from an iterator
    pub fn add_table_from<K, V, I>(&mut self, name: &str, iterator: I) -> LuaResult<&mut Self>
    where
        K: ToLua<'a>,
        V: ToLua<'a>,
        I: IntoIterator<Item = (K, V)>,
    {
        self.check_collision(name)?;
        self.fields.insert(
            name.to_owned(),
            self.lua.pack(self.lua.create_table_from(iterator)?)?,
        );
        Ok(self)
    }

    /// Adds an empty table to the module
    pub fn add_table_empty(&mut self, name: &str) -> LuaResult<&mut Self> {
        self.check_collision(name)?;
        let table = self.lua.create_table()?;
        self.fields.insert(name.to_owned(), self.lua.pack(table)?);
        Ok(self)
    }

    /// Adds a lua value (any) to the module
    pub fn add_value(&mut self, name: &str, value: impl ToLua<'a>) -> LuaResult<&mut Self> {
        self.check_collision(name)?;
        self.fields.insert(name.to_owned(), value.to_lua(self.lua)?);
        Ok(self)
    }

    /// Adds a string to the module
    pub fn add_string(&mut self, name: &str, value: &str) -> LuaResult<&mut Self> {
        self.check_collision(name)?;
        self.fields.insert(name.to_owned(), value.to_lua(self.lua)?);
        Ok(self)
    }

    /// Adds an integer to the module
    pub fn add_int(&mut self, name: &str, value: i64) -> LuaResult<&mut Self> {
        self.check_collision(name)?;
        self.fields.insert(name.to_owned(), value.to_lua(self.lua)?);
        Ok(self)
    }

    /// Adds a number to the module
    pub fn add_float(&mut self, name: &str, value: f64) -> LuaResult<&mut Self> {
        self.check_collision(name)?;
        self.fields.insert(name.to_owned(), value.to_lua(self.lua)?);
        Ok(self)
    }

    /// Adds a boolean to the module
    pub fn add_bool(&mut self, name: &str, value: bool) -> LuaResult<&mut Self> {
        self.check_collision(name)?;
        self.fields.insert(name.to_owned(), value.to_lua(self.lua)?);
        Ok(self)
    }

    /// Adds a function to the module, consuming and returning the builder
    pub fn with_fn<A, R, F>(mut self, name: &str, func: F) -> LuaResult<Self>
    where
        F: 'static + Send + Fn(&'a Lua, A) -> LuaResult<R>,
        A: FromLuaMulti<'a>,
        R: ToLuaMulti<'a>,
    {
        self.check_collision(name)?;
        let func = self.lua.create_function(func)?;
        self.fields.insert(name.to_owned(), self.lua.pack(func)?);
        Ok(self)
    }

    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    /// Adds an async function to the module, consuming and returning the builder
    pub fn with_fn_async<A, R, F, FR>(mut self, name: &str, func: F) -> LuaResult<Self>
    where
        F: 'static + Send + Fn(&'a Lua, A) -> FR,
        A: FromLuaMulti<'a>,
        R: ToLuaMulti<'a>,
        FR: 'a + Send + Future<Output = LuaResult<R>>,
    {
        self.check_collision(name)?;
        let func = self.lua.create_async_function(func)?;
        self.fields.insert(name.to_owned(), self.lua.pack(func)?);
        Ok(self)
    }

    /// Adds a C function to the module, consuming and returning the builder
    pub fn with_c_fn(mut self, name: &str, func: mlua::lua_CFunction) -> LuaResult<Self> {
        self.check_collision(name)?;
        unsafe {
            let func = self.lua.create_c_function(func)?;
            self.fields.insert(name.to_owned(), self.lua.pack(func)?);
        }
        Ok(self)
    }

    /// Adds a table to the module, consuming and returning the builder
    pub fn with_table(mut self, name: &str, table: LuaTable<'a>) -> LuaResult<Self> {
        self.check_collision(name)?;
        self.fields.insert(name.to_owned(), self.lua.pack(table)?);
        Ok(self)
    }

    /// Adds a table to the module from an iterator, consuming and returning the builder
    pub fn with_table_from<K, V, I>(mut self, name: &str, iterator: I) -> LuaResult<Self>
    where
        K: ToLua<'a>,
        V: ToLua<'a>,
        I: IntoIterator<Item = (K, V)>,
    {
        self.check_collision(name)?;
        self.fields.insert(
            name.to_owned(),
            self.lua.pack(self.lua.create_table_from(iterator)?)?,
        );
        Ok(self)
    }

    /// Adds an empty table to the module, consuming and returning the builder
    pub fn with_table_empty(mut self, name: &str) -> LuaResult<Self> {
        self.check_collision(name)?;
        let table = self.lua.create_table()?;
        self.fields.insert(name.to_owned(), self.lua.pack(table)?);
        Ok(self)
    }

    /// Adds a lua value (any) to the module, consuming and returning the builder
    pub fn with_value(mut self, name: &str, value: impl ToLua<'a>) -> LuaResult<Self> {
        self.check_collision(name)?;
        self.fields.insert(name.to_owned(), value.to_lua(self.lua)?);
        Ok(self)
    }

    /// Adds a string to the module, consuming and returning the builder
    pub fn with_string(mut self, name: &str, value: &str) -> LuaResult<Self> {
        self.check_collision(name)?;
        self.fields.insert(name.to_owned(), value.to_lua(self.lua)?);
        Ok(self)
    }

    /// Adds an integer to the module, consuming and returning the builder
    pub fn with_int(mut self, name: &str, value: i64) -> LuaResult<Self> {
        self.check_collision(name)?;
        self.fields.insert(name.to_owned(), value.to_lua(self.lua)?);
        Ok(self)
    }

    /// Adds a number to the module, consuming and returning the builder
    pub fn with_float(mut self, name: &str, value: f64) -> LuaResult<Self> {
        self.check_collision(name)?;
        self.fields.insert(name.to_owned(), value.to_lua(self.lua)?);
        Ok(self)
    }

    /// Adds a boolean to the module, consuming and returning the builder
    pub fn with_bool(&mut self, name: &str, value: bool) -> LuaResult<&mut Self> {
        self.check_collision(name)?;
        self.fields.insert(name.to_owned(), value.to_lua(self.lua)?);
        Ok(self)
    }

    /// Consumes the builder and returns the module as a table
    pub fn build(self) -> LuaResult<LuaTable<'a>> {
        let module = self.lua.create_table()?;
        for (name, value) in self.fields {
            module.set(name, value)?;
        }
        Ok(module)
    }
}

impl<'a> ToLua<'a> for ModuleBuilder<'a> {
    fn to_lua(self, lua: &'a Lua) -> LuaResult<LuaValue<'a>> {
        Ok(lua.pack(self.build()?)?)
    }
}
