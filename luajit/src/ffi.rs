use crate::types::*;
use std::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LuaState {
    _unused: [u8; 0],
}

// Indices
pub mod index {
    pub const REGISTRY: i32 = -10000;
    pub const ENVIRON: i32 = -10001;
    pub const GLOBALS: i32 = -10002;

    #[repr(i32)]
    #[derive(Debug, Clone, Copy)]
    pub enum Index {
        Globals = -10002,
        Environ = -10001,
        Registry = -10000,
    }
}

pub mod status {
    pub const OK: u32 = 0;
    pub const YIELD: u32 = 1;
    pub const ERRRUN: u32 = 2;
    pub const ERRSYNTAX: u32 = 3;
    pub const ERRMEM: u32 = 4;
    pub const ERRERR: u32 = 5;

    #[repr(u32)]
    #[derive(Debug, Clone, Copy)]
    pub enum Status {
        Ok = 0,
        Yield = 1,
        ErrRun = 2,
        ErrSyntax = 3,
        ErrMem = 4,
        ErrErr = 5,
    }
}

pub mod types {
    pub const NONE: i32 = -1;
    pub const NIL: i32 = 0;
    pub const BOOLEAN: i32 = 1;
    pub const LIGHTUSERDATA: i32 = 2;
    pub const NUMBER: i32 = 3;
    pub const STRING: i32 = 4;
    pub const TABLE: i32 = 5;
    pub const FUNCTION: i32 = 6;
    pub const USERDATA: i32 = 7;
    pub const THREAD: i32 = 8;

    #[repr(i32)]
    #[derive(Debug, Clone, Copy)]
    pub enum TypeId {
        None = -1,
        Nil = 0,
        Boolean = 1,
        LightUserData = 2,
        Number = 3,
        String = 4,
        Table = 5,
        Function = 6,
        UserData = 7,
        Thread = 8,
    }
}

extern "C" {
    pub fn lua_newstate(f: LuaAlloc, ud: *mut c_void) -> *mut LuaState;

    pub fn lua_close(L: *mut LuaState);

    pub fn lua_newthread(L: *mut LuaState) -> *mut LuaState;

    pub fn lua_atpanic(L: *mut LuaState, panicf: LuaCFunction) -> LuaCFunction;

    pub fn lua_gettop(L: *mut LuaState) -> c_int;

    pub fn lua_settop(L: *mut LuaState, idx: c_int);

    pub fn lua_pushvalue(L: *mut LuaState, idx: c_int);

    pub fn lua_remove(L: *mut LuaState, idx: c_int);

    pub fn lua_insert(L: *mut LuaState, idx: c_int);

    pub fn lua_replace(L: *mut LuaState, idx: c_int);

    pub fn lua_checkstack(L: *mut LuaState, sz: c_int) -> c_int;

    pub fn lua_xmove(from: *mut LuaState, to: *mut LuaState, n: c_int);

    pub fn lua_isnumber(L: *mut LuaState, idx: c_int) -> c_int;

    pub fn lua_isstring(L: *mut LuaState, idx: c_int) -> c_int;

    pub fn lua_iscfunction(L: *mut LuaState, idx: c_int) -> c_int;

    pub fn lua_isuserdata(L: *mut LuaState, idx: c_int) -> c_int;

    pub fn lua_type(L: *mut LuaState, idx: c_int) -> c_int;

    pub fn lua_typename(L: *mut LuaState, tp: c_int) -> *const c_char;

    pub fn lua_equal(L: *mut LuaState, idx1: c_int, idx2: c_int) -> c_int;

    pub fn lua_rawequal(L: *mut LuaState, idx1: c_int, idx2: c_int) -> c_int;

    pub fn lua_lessthan(L: *mut LuaState, idx1: c_int, idx2: c_int) -> c_int;

    pub fn lua_tonumber(L: *mut LuaState, idx: c_int) -> LuaNumber;

    pub fn lua_tointeger(L: *mut LuaState, idx: c_int) -> LuaInteger;

    pub fn lua_toboolean(L: *mut LuaState, idx: c_int) -> c_int;

    pub fn lua_tolstring(L: *mut LuaState, idx: c_int, len: *mut usize) -> *const c_char;

    pub fn lua_objlen(L: *mut LuaState, idx: c_int) -> usize;

    pub fn lua_tocfunction(L: *mut LuaState, idx: c_int) -> LuaCFunction;

    pub fn lua_touserdata(L: *mut LuaState, idx: c_int) -> *mut c_void;

    pub fn lua_tothread(L: *mut LuaState, idx: c_int) -> *mut LuaState;

    pub fn lua_topointer(L: *mut LuaState, idx: c_int) -> *const c_void;

    pub fn lua_pop(L: *mut LuaState, n: c_int);

    pub fn lua_pushnil(L: *mut LuaState);

    pub fn lua_pushnumber(L: *mut LuaState, n: LuaNumber);

    pub fn lua_pushinteger(L: *mut LuaState, n: LuaInteger);

    pub fn lua_pushlstring(L: *mut LuaState, s: *const c_char, l: usize);

    pub fn lua_pushstring(L: *mut LuaState, s: *const c_char);

    pub fn lua_pushvfstring(
        L: *mut LuaState,
        fmt: *const c_char,
        argp: *mut __va_list_tag,
    ) -> *const c_char;

    pub fn lua_pushfstring(L: *mut LuaState, fmt: *const c_char, ...) -> *const c_char;

    pub fn lua_pushcclosure(L: *mut LuaState, fn_: LuaCFunction, n: c_int);

    pub fn lua_pushboolean(L: *mut LuaState, b: c_int);

    pub fn lua_pushlightuserdata(L: *mut LuaState, p: *mut c_void);

    pub fn lua_pushthread(L: *mut LuaState) -> c_int;

    pub fn lua_gettable(L: *mut LuaState, idx: c_int);

    pub fn lua_getfield(L: *mut LuaState, idx: c_int, k: *const c_char);

    pub fn lua_rawget(L: *mut LuaState, idx: c_int);

    pub fn lua_rawgeti(L: *mut LuaState, idx: c_int, n: c_int);

    pub fn lua_createtable(L: *mut LuaState, narr: c_int, nrec: c_int);

    pub fn lua_newuserdata(L: *mut LuaState, sz: usize) -> *mut c_void;

    pub fn lua_getmetatable(L: *mut LuaState, objindex: c_int) -> c_int;

    pub fn lua_getfenv(L: *mut LuaState, idx: c_int);

    pub fn lua_settable(L: *mut LuaState, idx: c_int);

    pub fn lua_setfield(L: *mut LuaState, idx: c_int, k: *const c_char);

    pub fn lua_rawset(L: *mut LuaState, idx: c_int);

    pub fn lua_rawseti(L: *mut LuaState, idx: c_int, n: c_int);

    pub fn lua_setmetatable(L: *mut LuaState, objindex: c_int) -> c_int;

    pub fn lua_setfenv(L: *mut LuaState, idx: c_int) -> c_int;

    pub fn lua_call(L: *mut LuaState, nargs: c_int, nresults: c_int);

    pub fn lua_pcall(L: *mut LuaState, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int;

    pub fn lua_cpcall(L: *mut LuaState, func: LuaCFunction, ud: *mut c_void) -> c_int;

    pub fn lua_load(
        L: *mut LuaState,
        reader: LuaReader,
        dt: *mut c_void,
        chunkname: *const c_char,
    ) -> c_int;

    pub fn lua_dump(L: *mut LuaState, writer: LuaWriter, data: *mut c_void) -> c_int;

    pub fn lua_yield(L: *mut LuaState, nresults: c_int) -> c_int;

    pub fn lua_resume(L: *mut LuaState, narg: c_int) -> c_int;

    pub fn lua_status(L: *mut LuaState) -> c_int;

    pub fn lua_gc(L: *mut LuaState, what: c_int, data: c_int) -> c_int;

    pub fn lua_error(L: *mut LuaState) -> !;

    pub fn lua_next(L: *mut LuaState, idx: c_int) -> c_int;

    pub fn lua_concat(L: *mut LuaState, n: c_int);

    pub fn lua_getallocf(L: *mut LuaState, ud: *mut *mut c_void) -> LuaAlloc;

    pub fn lua_setallocf(L: *mut LuaState, f: LuaAlloc, ud: *mut c_void);

    pub fn lua_setlevel(from: *mut LuaState, to: *mut LuaState);

    pub fn lua_getstack(L: *mut LuaState, level: c_int, ar: *mut LuaDebug) -> c_int;

    pub fn lua_getinfo(L: *mut LuaState, what: *const c_char, ar: *mut LuaDebug) -> c_int;

    pub fn lua_getlocal(L: *mut LuaState, ar: *const LuaDebug, n: c_int) -> *const c_char;

    pub fn lua_setlocal(L: *mut LuaState, ar: *const LuaDebug, n: c_int) -> *const c_char;

    pub fn lua_getupvalue(L: *mut LuaState, funcindex: c_int, n: c_int) -> *const c_char;

    pub fn lua_setupvalue(L: *mut LuaState, funcindex: c_int, n: c_int) -> *const c_char;

    pub fn lua_sethook(L: *mut LuaState, func: LuaHook, mask: c_int, count: c_int) -> c_int;

    pub fn lua_gethook(L: *mut LuaState) -> LuaHook;

    pub fn lua_gethookmask(L: *mut LuaState) -> c_int;

    pub fn lua_gethookcount(L: *mut LuaState) -> c_int;

    pub fn lua_upvalueid(L: *mut LuaState, idx: c_int, n: c_int) -> *mut c_void;

    pub fn lua_upvaluejoin(L: *mut LuaState, idx1: c_int, n1: c_int, idx2: c_int, n2: c_int);

    pub fn lua_loadx(
        L: *mut LuaState,
        reader: LuaReader,
        dt: *mut c_void,
        chunkname: *const c_char,
        mode: *const c_char,
    ) -> c_int;

    pub fn lua_version(L: *mut LuaState) -> *const LuaNumber;

    pub fn lua_copy(L: *mut LuaState, fromidx: c_int, toidx: c_int);

    pub fn lua_tonumberx(L: *mut LuaState, idx: c_int, isnum: *mut c_int) -> LuaNumber;

    pub fn lua_tointegerx(L: *mut LuaState, idx: c_int, isnum: *mut c_int) -> LuaInteger;

    pub fn lua_isyieldable(L: *mut LuaState) -> c_int;
}

pub unsafe fn lua_getglobal(L: *mut LuaState, name: *const c_char) {
    lua_getfield(L, index::GLOBALS, name);
}

// Misc stuff
pub type va_list = [__va_list_tag; 1usize];
pub type wchar_t = c_int;

pub type LuaHook = ::std::option::Option<unsafe extern "C" fn(L: *mut LuaState, ar: *mut LuaDebug)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LuaDebug {
    pub event: c_int,
    pub name: *const c_char,
    pub namewhat: *const c_char,
    pub what: *const c_char,
    pub source: *const c_char,
    pub currentline: c_int,
    pub nups: c_int,
    pub linedefined: c_int,
    pub lastlinedefined: c_int,
    pub short_src: [c_char; 60usize],
    pub i_ci: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut c_void,
    pub reg_save_area: *mut c_void,
}

#[allow(unused)]
pub type LuaReader = ::std::option::Option<
    unsafe extern "C" fn(L: *mut LuaState, ud: *mut c_void, sz: *mut usize) -> *const c_char,
>;

#[allow(unused)]
pub type LuaWriter = ::std::option::Option<
    unsafe extern "C" fn(L: *mut LuaState, p: *const c_void, sz: usize, ud: *mut c_void) -> c_int,
>;
#[allow(unused)]
pub type LuaAlloc = ::std::option::Option<
    unsafe extern "C" fn(
        ud: *mut c_void,
        ptr: *mut c_void,
        osize: usize,
        nsize: usize,
    ) -> *mut c_void,
>;

// Things that were generated but probably won't be used
#[allow(unused)]
pub const LUA_MULTRET: i32 = -1;
#[allow(unused)]
pub const LUA_MINSTACK: u32 = 20;
#[allow(unused)]
pub const LUA_GCSTOP: u32 = 0;
#[allow(unused)]
pub const LUA_GCRESTART: u32 = 1;
#[allow(unused)]
pub const LUA_GCCOLLECT: u32 = 2;
#[allow(unused)]
pub const LUA_GCCOUNT: u32 = 3;
#[allow(unused)]
pub const LUA_GCCOUNTB: u32 = 4;
#[allow(unused)]
pub const LUA_GCSTEP: u32 = 5;
#[allow(unused)]
pub const LUA_GCSETPAUSE: u32 = 6;
#[allow(unused)]
pub const LUA_GCSETSTEPMUL: u32 = 7;
#[allow(unused)]
pub const LUA_GCISRUNNING: u32 = 9;
#[allow(unused)]
pub const LUA_HOOKCALL: u32 = 0;
#[allow(unused)]
pub const LUA_HOOKRET: u32 = 1;
#[allow(unused)]
pub const LUA_HOOKLINE: u32 = 2;
#[allow(unused)]
pub const LUA_HOOKCOUNT: u32 = 3;
#[allow(unused)]
pub const LUA_HOOKTAILRET: u32 = 4;
#[allow(unused)]
pub const LUA_MASKCALL: u32 = 1;
#[allow(unused)]
pub const LUA_MASKRET: u32 = 2;
#[allow(unused)]
pub const LUA_MASKLINE: u32 = 4;
#[allow(unused)]
pub const LUA_MASKCOUNT: u32 = 8;
