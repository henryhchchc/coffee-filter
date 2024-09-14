use std::mem::MaybeUninit;

use super::{Env, Error};
use crate::{macros::jvmti, sys, utils::jvmti_result};

/// A chunk of memory allocated by the JVM
#[derive(Debug)]
pub struct JvmMemoryChunk<'jvmti> {
    env: &'jvmti Env,
    ptr: *mut u8,
    size: usize,
}

impl<'a> JvmMemoryChunk<'a> {
    pub(crate) fn from_raw_parts(env: &'a Env, ptr: *mut u8, size: usize) -> Self {
        Self { env, ptr, size }
    }

    /// Extracts a slice containing the entire chunk
    #[must_use]
    pub const fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.size) }
    }

    /// Extracts a mutable slice containing the entire chunk
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.size) }
    }

    /// Consumes and leaks the chunk of memory.
    /// It is nolonger possible to deallocate the memory.
    /// Dropping the returned slice will lead to a memory leak.
    #[must_use]
    pub fn leak(self) -> &'a mut [u8] {
        let ptr = self.ptr;
        let size = self.size;
        std::mem::forget(self);
        unsafe { std::slice::from_raw_parts_mut(ptr, size) }
    }
}

impl Drop for JvmMemoryChunk<'_> {
    fn drop(&mut self) {
        let result = jvmti!(self.env.ptr, Deallocate, self.ptr);
        assert_eq!(
            result,
            sys::JVMTI_ERROR_NONE,
            "Failed to deallocate memory: {result:?}",
        );
    }
}

impl Env {
    /// Allocates a chunk of memory of the given size.
    /// # Errors
    /// - [`Error::OutOfMemory`] if the memory request cannot be honored.
    pub fn allocate(&self, size: usize) -> Result<JvmMemoryChunk<'_>, Error> {
        let mut bytes: MaybeUninit<*mut u8> = MaybeUninit::uninit();
        let size_in_jvm = size.try_into().map_err(|_| Error::IllegalArgument)?;
        let errno = jvmti!(self.ptr, Allocate, size_in_jvm, bytes.as_mut_ptr());
        jvmti_result(errno, || {
            let ptr = unsafe { bytes.assume_init() };
            JvmMemoryChunk::from_raw_parts(self, ptr, size)
        })
    }
}
