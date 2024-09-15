use std::ffi::{c_char, CStr};

use crate::{macros::unsafe_jvmti, sys};

use super::Env;

pub struct JString<'j> {
    env: &'j Env,
    ptr: *mut c_char,
}

impl std::fmt::Debug for JString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cstr = unsafe { CStr::from_ptr(self.ptr) };
        cstr.fmt(f)
    }
}

impl<'a> JString<'a> {
    pub(crate) const fn from_raw_parts(env: &'a Env, ptr: *mut c_char) -> Self {
        Self { env, ptr }
    }

    pub(crate) const fn as_ptr(&self) -> *const c_char {
        self.ptr
    }

    pub const fn as_cstr(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.ptr) }
    }
}

impl Drop for JString<'_> {
    fn drop(&mut self) {
        let errno = unsafe_jvmti!(self.env.ptr, Deallocate, self.ptr.cast());
        assert_eq!(errno, sys::JVMTI_ERROR_NONE, "Failed to deallocate string");
    }
}

impl Clone for JString<'_> {
    fn clone(&self) -> Self {
        let strlen = unsafe { CStr::from_ptr(self.ptr) }.to_bytes().len();
        let new_mem = self
            .env
            .allocate(strlen)
            .expect("Fail to allocate memory for copying")
            .into_raw()
            .cast();
        unsafe {
            std::ptr::copy_nonoverlapping(self.ptr.cast(), new_mem, strlen + 1);
        }
        Self {
            env: self.env,
            ptr: new_mem,
        }
    }
}
