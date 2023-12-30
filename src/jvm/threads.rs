//! APIs for working with JVM threads.

use std::{
    ffi::{c_uint, CStr, OsStr, OsString},
    mem::MaybeUninit,
    os::unix::prelude::OsStrExt,
};

use crate::{macros::call_jvmti, sys};

use super::{errors::JvmTIError, objects::Object, Jvm};

#[derive(Debug)]
pub struct ThreadGroup<'j> {
    jvm: &'j Jvm,
    jthread_group: sys::jthreadGroup,
}

impl ThreadGroup<'_> {
    pub(crate) unsafe fn from_ptr<'j>(
        jvm: &'j Jvm,
        jthread_group: sys::jthreadGroup,
    ) -> ThreadGroup<'j> {
        assert!(
            !jthread_group.is_null(),
            "The thread group pointer must not be null"
        );
        ThreadGroup { jvm, jthread_group }
    }
}

#[derive(Debug)]
pub struct ThreadInfo<'g, 'l> {
    pub name: OsString,
    pub priority: i32,
    pub is_daemon: bool,
    pub group: ThreadGroup<'g>,
    pub context_class_loader: Object<'l>,
}

#[derive(Debug)]
pub struct Thread<'j> {
    jvm: &'j Jvm,
    jthread: sys::jthread,
}

impl Thread<'_> {
    pub fn into_raw(self) -> sys::jthread {
        self.jthread
    }

    pub fn info(&self) -> Result<ThreadInfo<'_, '_>, JvmTIError> {
        let native_thread_info = unsafe { self.jvm.get_thread_info(self.jthread) }?;
        let name_c = unsafe { CStr::from_ptr(native_thread_info.name) };
        let name = OsStr::from_bytes(name_c.to_bytes()).to_owned();
        let priority = native_thread_info.priority;
        let is_daemon = native_thread_info.is_daemon != 0;
        // SAFETY: `native_thread_info.thread_group` is a valid `sys::jthreadGroup` as garanteed by the JVM TI API.
        let group = unsafe { ThreadGroup::from_ptr(self.jvm, native_thread_info.thread_group) };
        let context_class_loader =
            unsafe { Object::from_ptr(self.jvm, native_thread_info.context_class_loader) };
        Ok(ThreadInfo {
            name,
            priority,
            is_daemon,
            group,
            context_class_loader,
        })
    }

    pub(crate) unsafe fn from_ptr<'j>(jvm: &'j Jvm, jthread: sys::jthread) -> Thread<'j> {
        assert!(!jthread.is_null(), "The thread pointer must not be null");
        Thread { jvm, jthread }
    }
}

impl Jvm {
    unsafe fn get_thread_info(
        &self,
        jthread: sys::jthread,
    ) -> Result<sys::jvmtiThreadInfo, JvmTIError> {
        let mut thread_info = MaybeUninit::uninit();
        call_jvmti!(
            self.jvmti_ptr,
            GetThreadInfo,
            jthread,
            thread_info.as_mut_ptr()
        )
        .map(|_| thread_info.assume_init())
    }
}
