use std::{ffi::c_char, mem::MaybeUninit};

use crate::{macros::unsafe_jvmti, sys, utils::jvmti_result};

use super::{Env, Error, JStr};

impl Env {
    /// Gets a list of VM system property keys which may be used with [`get_system_property`].
    /// # Errors
    /// See [`Error`] for more information.
    pub fn get_system_property_keys(&self) -> Result<Vec<JStr<'_>>, Error> {
        let mut count: MaybeUninit<sys::jint> = MaybeUninit::uninit();
        let mut ptr: MaybeUninit<*mut *mut c_char> = MaybeUninit::uninit();
        let errno = unsafe_jvmti!(
            self.ptr,
            GetSystemProperties,
            count.as_mut_ptr(),
            ptr.as_mut_ptr()
        );
        jvmti_result(errno, || {
            let count = unsafe { count.assume_init() };
            let strs_ptr = unsafe { ptr.assume_init() };
            let keys = (0..count)
                .map(|i| {
                    let ptr = unsafe { *strs_ptr.add(i as usize) };
                    JStr::from_raw_parts(self, ptr)
                })
                .collect();
            let errno = unsafe_jvmti!(self.ptr, Deallocate, strs_ptr.cast());
            assert_eq!(
                errno,
                sys::JVMTI_ERROR_NONE,
                "Failed to deallocate system property keys"
            );
            keys
        })
    }

    /// Gets the value of a VM system property.
    /// # Errors
    /// - [`Error::NotAvailable`] if the property is not available.
    ///
    /// See [`Error`] for more information.
    pub fn get_system_property(&self, key: &JStr<'_>) -> Result<JStr<'_>, Error> {
        let mut value_ptr: MaybeUninit<*mut c_char> = MaybeUninit::uninit();
        let errno = unsafe_jvmti!(
            self.ptr,
            GetSystemProperty,
            key.as_ptr(),
            value_ptr.as_mut_ptr()
        );
        jvmti_result(errno, || {
            let value_ptr = unsafe { value_ptr.assume_init() };
            JStr::from_raw_parts(self, value_ptr)
        })
    }

    /// Sets the value of a VM system property.
    /// # Errors
    /// - [`Error::NotAvailable`] if the property is not available or writable.
    pub fn set_system_property(&self, key: &JStr<'_>, value: &JStr<'_>) -> Result<(), Error> {
        let errno = unsafe_jvmti!(self.ptr, SetSystemProperty, key.as_ptr(), value.as_ptr());
        jvmti_result(errno, || ())
    }
}
