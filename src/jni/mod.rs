use std::mem::MaybeUninit;

use crate::{
    jvmti,
    sys::{self},
};

#[derive(Debug)]
pub struct JavaVM {
    ptr: *mut sys::JavaVM,
}

impl JavaVM {
    pub fn get_env(&self, version: jvmti::Version) -> Result<jvmti::Env, i32> {
        let mut env_ptr: MaybeUninit<*mut sys::jvmtiEnv> = MaybeUninit::uninit();
        let result = unsafe {
            let get_env_func = (**self.ptr).GetEnv.expect("GetEnv function should exist");
            get_env_func(self.ptr, env_ptr.as_mut_ptr().cast(), version.into())
        };
        if result == sys::JNI_OK {
            let env_ptr = unsafe { env_ptr.assume_init() };
            Ok(jvmti::Env::from_ptr(env_ptr))
        } else {
            Err(result)
        }
    }
}
