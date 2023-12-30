//! APIs for interacting with the JVM Tool Interface (JVM TI).
use std::{
    fmt::Debug,
    mem::{size_of, MaybeUninit},
};

pub mod class;
pub mod errors;
pub mod events;
pub mod general;
pub mod jni;
pub mod objects;
pub mod threads;

use crate::{macros::call_jvmti, sys};

use self::{errors::JvmTIError, general::JvmTIVersion};

/// A raw JVM pointer.
pub type JvmPointer = *mut sys::JavaVM;

/// An JVM Tool Interface (JVM TI) environment.
pub struct Jvm {
    jvmti_ptr: *mut sys::jvmtiEnv,
    callbacks: events::EventCallbacks,
}

impl Debug for Jvm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Jvm@{:p}", self.jvmti_ptr))
    }
}

/// An error returned when creating a [`Jvm`] from a raw JVM pointer.
#[derive(thiserror::Error, Debug)]
pub enum JvmTICreationError {
    /// When the JVM pointer is null.
    #[error("The JVM pointer is null")]
    NullJVMPointer,
    /// When the JVM TI version is not supported.
    #[error("JNI version mismatch")]
    WrongVersion,
    /// When the JVM is not attached.
    #[error("The VM is not attached")]
    Detached,
    /// When the JVM TI environment fails to initialize.
    #[error("Fail to initialize the JVM TI environment: {0}")]
    JvmTIInitialization(#[from] JvmTIError),
}

impl Jvm {
    /// Creates a new [`Jvm`] from a raw JVM pointer.
    pub fn from_jvm_ptr<'j>(
        vm_ptr: JvmPointer,
        version: JvmTIVersion,
    ) -> Result<&'j mut Self, JvmTICreationError> {
        if vm_ptr.is_null() {
            return Err(JvmTICreationError::NullJVMPointer);
        }
        let mut jvmti_ptr: MaybeUninit<*mut sys::jvmtiEnv> = MaybeUninit::uninit();

        // SAFETY: GetEnv will return an error code if it fails instead of panicking.
        let return_code = unsafe {
            let jni_get_env = (**vm_ptr).GetEnv.expect("GetEnv is not available");
            jni_get_env(vm_ptr, jvmti_ptr.as_mut_ptr().cast(), version.into())
        };
        match return_code {
            it if it == (sys::JNI_OK as i32) => {
                // SAFEFY: A `sys::JNI_OK` indicates that the `jvmti_ptr` has been initialized.
                let jvmti_ptr = unsafe { jvmti_ptr.assume_init() };
                let result = Self {
                    jvmti_ptr,
                    callbacks: Default::default(),
                };
                let result = Box::leak(Box::new(result));
                unsafe {
                    call_jvmti!(
                        jvmti_ptr,
                        SetEnvironmentLocalStorage,
                        result as *mut _ as *const _
                    )
                }?;
                Ok(result)
            }
            sys::JNI_EDETACHED => Err(JvmTICreationError::Detached),
            sys::JNI_EVERSION => Err(JvmTICreationError::WrongVersion),
            _ => unreachable!("unexpected result from GetEnv"),
        }
    }

    /// Gets a reference to the [`Jvm`] from the JVM TI environment pointer.
    /// # Safety
    /// It is safe to call this function if there is a [`Jvm`] created with [`from_jvm_ptr`].
    /// # Panics
    /// This works only if the [`Jvm`] has been set as the local storage of the JVM TI environment.
    /// It will panic if:
    /// - The JVM TI environment pointer is null.
    /// - There is no [`Jvm`] set as the local storage of the JVM TI environment.
    pub(crate) unsafe fn from_ptr<'j>(jvmti_ptr: *mut sys::jvmtiEnv) -> &'j Self {
        assert!(!jvmti_ptr.is_null(), "The jvmti_ptr is null.");
        let mut jvm_ptr: MaybeUninit<*mut Self> = MaybeUninit::uninit();
        call_jvmti!(
            jvmti_ptr,
            GetEnvironmentLocalStorage,
            jvm_ptr.as_mut_ptr().cast()
        )
        .map(|_| {
            jvm_ptr
                .assume_init()
                .as_ref()
                .expect("JvmTIEnv is not initialized")
        })
        .expect("Fail to get the jvm pointer from local storage.")
    }

    pub fn update_callbacks<U>(&mut self, modifier: U) -> Result<(), JvmTIError>
    where
        U: FnOnce(&mut events::EventCallbacks),
    {
        modifier(&mut self.callbacks);
        self.update_native_callback()?;
        Ok(())
    }
}

impl Drop for Jvm {
    fn drop(&mut self) {
        // SAFETY: `self.jvmti_ptr` is a valid `sys::jvmtiEnv` because of the API restrictions.
        unsafe { call_jvmti!(self.jvmti_ptr, DisposeEnvironment) }
            .expect("Fail to dispose the jvmTiEnv.");
    }
}
