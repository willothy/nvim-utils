use std::collections::HashMap;

use crate::error::Error;
use crate::ffi::{
    self, lua_gettop, lua_next, lua_objlen, lua_pop, lua_pushnil, lua_pushvalue, lua_toboolean,
    lua_tointeger, lua_tonumber, lua_type, types::TypeId, LuaState as State,
};
use crate::guard::{popguard, StackGuard};
use crate::types::{LuaInteger, LuaNumber};
use crate::util::{count, set_ssize};

type Result<T> = std::result::Result<T, Error>;

pub trait Pop: Sized {
    unsafe fn pop(state: *mut State) -> Result<Self>;
}

impl Pop for () {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        if lua_gettop(state) == 0 {
            return Ok(());
        }

        match lua_type(state, -1) {
            i if i == TypeId::Nil as i32 => {
                lua_pop(state, 1);
                Ok(())
            }
            other => Err(Error::PopUnexpectedType("nil", other.to_string())),
        }
    }
}

impl Pop for bool {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        popguard!(state);

        match lua_type(state, -1) {
            i if i == TypeId::Boolean as i32 => {
                let val = lua_toboolean(state, -1) == 1;
                lua_pop(state, 1);
                Ok(val)
            }
            other => Err(Error::PopUnexpectedType("bool", other.to_string())),
        }
    }
}

impl Pop for LuaInteger {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        popguard!(state);

        match lua_type(state, -1) {
            i if i == TypeId::Number as i32 => {
                let val = lua_tointeger(state, -1);
                lua_pop(state, 1);
                Ok(val)
            }
            other => Err(Error::PopUnexpectedType("int", other.to_string())),
        }
    }
}

impl Pop for i8 {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        let n: LuaInteger = LuaInteger::pop(state)?;
        Ok(n as i8)
    }
}

impl Pop for u8 {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        let n: LuaInteger = LuaInteger::pop(state)?;
        Ok(n as u8)
    }
}

impl Pop for i16 {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        let n: LuaInteger = LuaInteger::pop(state)?;
        Ok(n as i16)
    }
}

impl Pop for u16 {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        let n: LuaInteger = LuaInteger::pop(state)?;
        Ok(n as u16)
    }
}

impl Pop for i32 {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        let n: LuaInteger = LuaInteger::pop(state)?;
        Ok(n as i32)
    }
}

impl Pop for u32 {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        let n: LuaInteger = LuaInteger::pop(state)?;
        Ok(n as u32)
    }
}

impl Pop for i64 {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        let n: LuaInteger = LuaInteger::pop(state)?;
        Ok(n as i64)
    }
}

impl Pop for u64 {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        let n: LuaInteger = LuaInteger::pop(state)?;
        Ok(n as u64)
    }
}

impl Pop for usize {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        let n: LuaInteger = LuaInteger::pop(state)?;
        Ok(n as usize)
    }
}

impl Pop for LuaNumber {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        popguard!(state);

        match lua_type(state, -1) {
            t if t == TypeId::Number as i32 => {
                let val = lua_tonumber(state, -1);
                lua_pop(state, 1);
                Ok(val)
            }
            other => Err(Error::PopUnexpectedType("float", other.to_string())),
        }
    }
}

impl Pop for f32 {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        let n: LuaNumber = LuaNumber::pop(state)?;
        Ok(n as f32)
    }
}

impl Pop for String {
    unsafe fn pop(state: *mut State) -> Result<Self> {
        popguard!(state);

        match lua_type(state, -1) {
            t if t == TypeId::String as i32 => {
                let mut len: usize = 0;
                let ptr = ffi::lua_tolstring(state, -1, &mut len as *mut usize);
                let s = std::slice::from_raw_parts(ptr as *const u8, len as usize);
                let s = String::from_utf8_lossy(s).into_owned();
                lua_pop(state, 1);
                Ok(s)
            }
            other => Err(Error::PopUnexpectedType("string", other.to_string())),
        }
    }
}

impl<T> Pop for Option<T>
where
    T: Pop,
{
    unsafe fn pop(state: *mut State) -> Result<Self> {
        popguard!(state);

        match lua_type(state, -1) {
            t if t == TypeId::Nil as i32 => {
                lua_pop(state, 1);
                Ok(None)
            }
            _ => T::pop(state).map(Some),
        }
    }
}

impl<T> Pop for Vec<T>
where
    T: Pop,
{
    unsafe fn pop(state: *mut State) -> Result<Self> {
        popguard!(state);

        match lua_type(state, -1) {
            t if t == TypeId::Table as i32 => {
                let mut vec = Vec::with_capacity(lua_objlen(state, -1));
                lua_pushnil(state);
                while lua_next(state, -2) != 0 {
                    vec.push(T::pop(state)?);
                }

                lua_pop(state, 1);

                Ok(vec)
            }
            other => Err(Error::PopUnexpectedType("Vec<T>", other.to_string())),
        }
    }
}

impl<K, V> Pop for HashMap<K, V>
where
    K: Pop + Eq + std::hash::Hash,
    V: Pop,
{
    unsafe fn pop(state: *mut State) -> Result<Self> {
        popguard!(state);

        match lua_type(state, -1) {
            t if t == TypeId::Table as i32 => {
                let mut res = HashMap::with_capacity(lua_objlen(state, -1));
                lua_pushnil(state);

                while lua_next(state, -2) != 0 {
                    let value = V::pop(state)?;

                    lua_pushvalue(state, -1);

                    let key = K::pop(state)?;

                    res.insert(key, value);
                }

                lua_pop(state, 1);

                Ok(res)
            }
            other => Err(Error::PopUnexpectedType("HashMap<K, V>", other.to_string())),
        }
    }
}

macro_rules! pop_tuple {
	($($name:ident),*) => {
		impl<$($name,)*> Pop for ($($name,)*)
		where
			$($name: Pop,)*
		{
			#[allow(non_snake_case)]
			unsafe fn pop(state: *mut State) -> Result<Self> {
				set_ssize(state, count!($($name)*));
				pop_rev!(state, $($name)*);
				Ok(($($name,)*))
			}
		}
	}
}

macro_rules! pop_rev {
    ($state:expr, $x:ident $($xs:ident)*) => {
        pop_rev!($state, $($xs)*);
		let $x = $x::pop($state)?;
    };
	($state:expr,) => ();
}

pop_tuple!(A);
pop_tuple!(A, B);
pop_tuple!(A, B, C);
pop_tuple!(A, B, C, D);
pop_tuple!(A, B, C, D, E);
pop_tuple!(A, B, C, D, E, F);
pop_tuple!(A, B, C, D, E, F, G);
pop_tuple!(A, B, C, D, E, F, G, H);
pop_tuple!(A, B, C, D, E, F, G, H, I);
pop_tuple!(A, B, C, D, E, F, G, H, I, J);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
pop_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
