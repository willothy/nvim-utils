use std::ffi::c_char;

pub trait AsCStr
where
    Self: AsRef<str> + Sized,
{
    #[inline(always)]
    fn as_cstr(&self) -> *const c_char;
    #[inline(always)]
    fn null_terminate(self) -> String {
        format!("{}\0", self.as_ref())
    }
}

impl AsCStr for String {
    fn as_cstr(&self) -> *const c_char {
        self.as_ptr() as *const c_char
    }
}

impl<'a> AsCStr for &'a str {
    fn as_cstr(&self) -> *const c_char {
        self.as_ptr() as *const c_char
    }
}

#[test]
fn cstr_test() {
    let s = "test";
    let c = s.as_cstr();
    unsafe {
        println!("{}", *c);
        assert_eq!(*((c as usize + 3) as *const u8) as char, 't')
    }
}
