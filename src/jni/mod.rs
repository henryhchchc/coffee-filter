//! Functionalities for JNI
use std::mem::MaybeUninit;

use crate::{
    jvmti,
    macros::unsafe_jvmti,
    sys::{self},
};

pub mod objects;

pub type RawJavaVM = sys::JavaVM;

/// A Java Virtual Machine in JNI
#[derive(Debug)]
pub struct JavaVM {
    ptr: *mut sys::JavaVM,
}

impl JavaVM {
    /// Create a new instance of [`JavaVM`] from a raw pointer
    pub fn from_raw(ptr: *mut RawJavaVM) -> Self {
        Self { ptr }
    }

    /// Get the JVMTI environment.
    /// # Errors
    /// See [`GetEnvError`] for more information.
    pub fn get_jvmti_env(&self, version: jvmti::Version) -> Result<jvmti::Env, GetJVMTIEnvError> {
        let mut env_ptr: MaybeUninit<*mut sys::jvmtiEnv> = MaybeUninit::uninit();
        let result = unsafe_jvmti!(
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
            sys::JNI_EDETACHED => Err(GetJVMTIEnvError::Detached),
            sys::JNI_EVERSION => Err(GetJVMTIEnvError::VersionNotSupported),
            _ => unreachable!("By the document of JNI API."),
        }
    }
}

#[derive(Debug, thiserror::Error)]
/// Errors that can occur when getting the JVMTI environment
pub enum GetJVMTIEnvError {
    /// The current thread is not attached to the VM
    #[error("The current thread is not attached to the VM")]
    Detached,
    /// The specified version is not supported
    #[error("The specified version is not supported")]
    VersionNotSupported,
}

#[derive(Debug)]
pub struct Env {
    ptr: *mut sys::JNIEnv,
}

impl Env {
    pub(crate) fn from_raw(ptr: *mut sys::JNIEnv) -> Self {
        Self { ptr }
    }
}
