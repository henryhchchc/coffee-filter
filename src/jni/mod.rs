use std::mem::MaybeUninit;

use crate::{
    jvmti,
    macros::jvmti,
    sys::{self},
};

pub type RawJavaVM = sys::JavaVM;

#[derive(Debug)]
pub struct JavaVM {
    ptr: *mut sys::JavaVM,
}

impl JavaVM {
    pub fn from_raw(ptr: *mut RawJavaVM) -> Self {
        Self { ptr }
    }

    pub fn get_env(&self, version: jvmti::Version) -> Result<jvmti::Env, GetEnvError> {
        let mut env_ptr: MaybeUninit<*mut sys::jvmtiEnv> = MaybeUninit::uninit();
        let result = jvmti!(
            self.ptr,
            GetEnv,
            env_ptr.as_mut_ptr().cast(),
            version.into()
        );
        match result {
            sys::JNI_OK => {
                let ptr = unsafe { env_ptr.assume_init() };
                Ok(jvmti::Env::from_raw(ptr))
            }
            sys::JNI_EDETACHED => Err(GetEnvError::Detached),
            sys::JNI_EVERSION => Err(GetEnvError::VersionNotSupported),
            _ => unreachable!("By the document of JNI API."),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetEnvError {
    #[error("The current thread is not attached to the VM")]
    Detached,
    #[error("The specified version is not supported")]
    VersionNotSupported,
}
