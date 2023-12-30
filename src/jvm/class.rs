//! APIs for working with Java classes.

use std::mem::MaybeUninit;

use crate::{macros::call_jvmti, sys};

use super::{errors::JvmTIError, Jvm};

/// A Java class.
#[derive(Debug)]
pub struct Class<'j> {
    jvm: &'j Jvm,
    jclass: sys::jclass,
}

impl Class<'_> {
    pub(crate) unsafe fn from_ptr<'j>(jvm: &'j Jvm, jclass: sys::jclass) -> Class<'j> {
        assert!(!jclass.is_null(), "The class pointer must not be null");
        Class { jvm, jclass }
    }
}

impl Jvm {
    /// Gets all the loaded classes.
    /// See [`GetLoadedClasses`](https://docs.oracle.com/javase/8/docs/platform/jvmti/jvmti.html#GetLoadedClasses).
    /// # Errors
    /// See [`JvmTIError`] for more information.
    pub fn get_loaded_classes(&self) -> Result<Vec<Class<'_>>, JvmTIError> {
        let mut class_count: MaybeUninit<sys::jint> = MaybeUninit::uninit();
        let mut classes: MaybeUninit<*mut sys::jclass> = MaybeUninit::uninit();
        // SAFETY: `self.jvmti_ptr` is a valid `sys::jvmtiEnv` because of the API restrictions.
        unsafe {
            call_jvmti!(
                self.jvmti_ptr,
                GetLoadedClasses,
                class_count.as_mut_ptr(),
                classes.as_mut_ptr()
            )
        }
        .map(|_| {
            // SAFETY: `GetLoadedClasses` will initialize `class_count` and `classes` when successful.
            let class_count = unsafe { class_count.assume_init() } as usize;
            let classes = unsafe { classes.assume_init() };
            // SAFETY: `classes` is a pointer to an array of `jclass` and `class_count` is the length of the array.
            let classes = unsafe { std::slice::from_raw_parts(classes, class_count) };
            classes
                .iter()
                .map(|&jclass| Class { jvm: self, jclass })
                .collect()
        })
    }
}
