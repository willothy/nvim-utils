use std::collections::HashMap;
use std::ffi::{c_char, c_int};

use crate::ffi::{self, lua_createtable, lua_rawset, lua_rawseti, lua_setfield, LuaState as State};
use crate::types::*;
use crate::util::count;

type PushRes = Result<c_int, anyhow::Error>;

pub trait Push {
    unsafe fn push(self, state: *mut State) -> PushRes;
}

impl Push for () {
    unsafe fn push(self, state: *mut State) -> PushRes {
        ffi::lua_pushnil(state);
        Ok(1)
    }
}

impl Push for bool {
    unsafe fn push(self, state: *mut State) -> PushRes {
        ffi::lua_pushboolean(state, self as i32);
        Ok(1)
    }
}

impl Push for LuaInteger {
    unsafe fn push(self, state: *mut State) -> PushRes {
        ffi::lua_pushinteger(state, self);
        Ok(1)
    }
}

impl Push for i8 {
    unsafe fn push(self, state: *mut State) -> PushRes {
        let n: LuaInteger = self.into();
        n.push(state)
    }
}

impl Push for u8 {
    unsafe fn push(self, state: *mut State) -> PushRes {
        (self as LuaInteger).push(state)
    }
}

impl Push for i16 {
    unsafe fn push(self, state: *mut State) -> PushRes {
        (self as LuaInteger).push(state)
    }
}

impl Push for u16 {
    unsafe fn push(self, state: *mut State) -> PushRes {
        (self as LuaInteger).push(state)
    }
}

impl Push for u32 {
    unsafe fn push(self, state: *mut State) -> PushRes {
        (self as LuaInteger).push(state)
    }
}

impl Push for i64 {
    unsafe fn push(self, state: *mut State) -> PushRes {
        (self as LuaInteger).push(state)
    }
}

impl Push for u64 {
    unsafe fn push(self, state: *mut State) -> PushRes {
        (self as LuaInteger).push(state)
    }
}

impl Push for usize {
    unsafe fn push(self, state: *mut State) -> PushRes {
        (self as LuaInteger).push(state)
    }
}

impl Push for LuaNumber {
    unsafe fn push(self, state: *mut State) -> PushRes {
        ffi::lua_pushnumber(state, self);
        Ok(1)
    }
}

impl Push for f32 {
    unsafe fn push(self, state: *mut State) -> PushRes {
        (self as LuaNumber).push(state)
    }
}

impl Push for String {
    unsafe fn push(self, state: *mut State) -> PushRes {
        ffi::lua_pushlstring(state, self.as_ptr() as *const c_char, self.len());
        Ok(1)
    }
}

impl<'a> Push for &'a str {
    unsafe fn push(self, state: *mut State) -> PushRes {
        ffi::lua_pushlstring(state, self.as_ptr() as *const c_char, self.len());
        Ok(1)
    }
}

impl<T> Push for Option<T>
where
    T: Push,
{
    unsafe fn push(self, state: *mut State) -> PushRes {
        if let Some(t) = self {
            t.push(state)
        } else {
            ().push(state)
        }
    }
}

impl<T> Push for Vec<T>
where
    T: Push,
{
    unsafe fn push(self, state: *mut State) -> PushRes {
        ffi::lua_createtable(state, self.len() as _, 0);

        self.into_iter()
            .enumerate()
            .try_for_each(|(idx, el)| -> Result<(), anyhow::Error> {
                el.push(state)?;
                ffi::lua_rawseti(state, -2, idx as i32 + 1);
                Ok(())
            })?;
        Ok(1)
    }
}

impl<K, V> Push for HashMap<K, V>
where
    K: Push + Eq + std::hash::Hash,
    V: Push,
{
    unsafe fn push(self, state: *mut State) -> PushRes {
        lua_createtable(state, 0, self.len() as i32);

        self.into_iter()
            .try_for_each(|(k, v)| -> Result<(), anyhow::Error> {
                k.push(state)?;
                v.push(state)?;
                lua_rawset(state, -3);
                Ok(())
            })?;

        Ok(1)
    }
}

macro_rules! push_tuple {
	($($name:ident),*) => {
		impl<$($name,)*> Push for ($($name,)*)
		where
			$($name: Push,)*
		{
			#[allow(non_snake_case)]
			unsafe fn push(self, state: *mut crate::ffi::LuaState) -> Result<c_int, anyhow::Error> {
				let ($($name,)*) = self;
				$($name.push(state)?;)*
				Ok(count!($($name)*))
			}
		}
	}
}

push_tuple!(A);
push_tuple!(A, B);
push_tuple!(A, B, C);
push_tuple!(A, B, C, D);
push_tuple!(A, B, C, D, E);
push_tuple!(A, B, C, D, E, F);
push_tuple!(A, B, C, D, E, F, G);
push_tuple!(A, B, C, D, E, F, G, H);
push_tuple!(A, B, C, D, E, F, G, H, I);
push_tuple!(A, B, C, D, E, F, G, H, I, J);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
push_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
