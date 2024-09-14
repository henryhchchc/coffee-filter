mod memory;
mod version;

pub mod general;
pub mod system_properties;

use std::{ffi::c_char, path::Display};

pub use {memory::JvmMemoryChunk, version::Version};

use crate::{macros::jvmti, sys};

/// A JVM Tool Interface (JVM TI) environment.
#[derive(Debug)]
pub struct Env {
    pub(crate) ptr: *mut sys::jvmtiEnv,
}

impl Env {
    pub(crate) fn from_raw(ptr: *mut sys::jvmtiEnv) -> Self {
        Self { ptr }
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        let result = jvmti!(self.ptr, DisposeEnvironment);
        assert_eq!(
            result,
            sys::JVMTI_ERROR_NONE,
            "Failed to dispose JVMTI environment: {result:?}",
        );
    }
}

pub struct JStr<'j> {
    env: &'j Env,
    ptr: *mut c_char,
}

impl std::fmt::Debug for JStr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cstr = unsafe { std::ffi::CStr::from_ptr(self.ptr) };
        write!(f, "{cstr:?}")
    }
}

impl<'a> JStr<'a> {
    pub(crate) fn from_raw_parts(env: &'a Env, ptr: *mut c_char) -> Self {
        Self { env, ptr }
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.ptr
    }
}

impl Drop for JStr<'_> {
    fn drop(&mut self) {
        let errno = jvmti!(self.env.ptr, Deallocate, self.ptr.cast());
        assert_eq!(errno, sys::JVMTI_ERROR_NONE, "Failed to deallocate string");
    }
}

/// The error that can occur when using the JVM Tool Interface (JVM TI).
#[derive(Debug, thiserror::Error, derive_more::TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum Error {
    /// Pointer is unexpectedly `NULL`.
    #[error("Pointer is unexpectedly NULL")]
    NullPointer = sys::JVMTI_ERROR_NULL_POINTER,

    /// The function attempted to allocate memory and no more memory was available for allocation.
    #[error("No more memory was available for allocation")]
    OutOfMemory = sys::JVMTI_ERROR_OUT_OF_MEMORY,

    /// The desired functionality has not been enabled in this virtual machine.
    #[error("The desired functionality has not been enabled in this virtual machine")]
    AccessDenied = sys::JVMTI_ERROR_ACCESS_DENIED,

    /// The thread being used to call this function is not attached to the virtual machine.
    /// Calls must be made from attached threads.
    #[error("The thread being used to call this function is not attached to the virtual machine")]
    UnattachedThread = sys::JVMTI_ERROR_UNATTACHED_THREAD,

    /// The JVM TI environment provided is no longer connected or is not an environment.
    #[error("The JVM TI environment provided is no longer connected or is not an environment")]
    InvalidEnvironment = sys::JVMTI_ERROR_INVALID_ENVIRONMENT,

    /// The operation is in the wrong phase.
    #[error("The operation is in the wrong phase")]
    WrongPhase = sys::JVMTI_ERROR_WRONG_PHASE,

    /// An internal error has occurred.
    #[error("An internal error has occurred")]
    Internal = sys::JVMTI_ERROR_INTERNAL,

    /// The requested information is not available.
    #[error("The requested information is not available")]
    AbsentInformation = sys::JVMTI_ERROR_ABSENT_INFORMATION,

    /// A circularity has been detected in the class definition.
    #[error("A circularity has been detected in the class definition")]
    CircularClassDefinition = sys::JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION,

    /// The class loader is not supported.
    #[error("The class loader is not supported")]
    ClassLoaderUnsupported = sys::JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED,

    /// The class has not been prepared.
    #[error("The class has not been prepared")]
    ClassNotPrepared = sys::JVMTI_ERROR_CLASS_NOT_PREPARED,

    /// The item already exists.
    #[error("The item already exists")]
    Duplicate = sys::JVMTI_ERROR_DUPLICATE,

    /// The class fails verification.
    #[error("The class fails verification")]
    FailsVerification = sys::JVMTI_ERROR_FAILS_VERIFICATION,

    /// An illegal argument has been provided.
    #[error("An illegal argument has been provided")]
    IllegalArgument = sys::JVMTI_ERROR_ILLEGAL_ARGUMENT,

    /// The operation was interrupted.
    #[error("The operation was interrupted")]
    Interrupt = sys::JVMTI_ERROR_INTERRUPT,

    /// The class is invalid.
    #[error("The class is invalid")]
    InvalidClass = sys::JVMTI_ERROR_INVALID_CLASS,

    /// The class format is invalid.
    #[error("The class format is invalid")]
    InvalidClassFormat = sys::JVMTI_ERROR_INVALID_CLASS_FORMAT,

    /// The event type is invalid.
    #[error("The event type is invalid")]
    InvalidEventType = sys::JVMTI_ERROR_INVALID_EVENT_TYPE,

    /// The field ID is invalid.
    #[error("The field ID is invalid")]
    InvalidFieldId = sys::JVMTI_ERROR_INVALID_FIELDID,

    /// The location is invalid.
    #[error("The location is invalid")]
    InvalidLocation = sys::JVMTI_ERROR_INVALID_LOCATION,

    /// The method ID is invalid.
    #[error("The method ID is invalid")]
    InvalidMethodId = sys::JVMTI_ERROR_INVALID_METHODID,

    /// The module is invalid.
    #[error("The module is invalid")]
    InvalidModule = sys::JVMTI_ERROR_INVALID_MODULE,

    /// The monitor is invalid.
    #[error("The monitor is invalid")]
    InvalidMonitor = sys::JVMTI_ERROR_INVALID_MONITOR,

    /// The object is invalid.
    #[error("The object is invalid")]
    InvalidObject = sys::JVMTI_ERROR_INVALID_OBJECT,

    /// The priority is invalid.
    #[error("The priority is invalid")]
    InvalidPriority = sys::JVMTI_ERROR_INVALID_PRIORITY,

    /// The slot is invalid.
    #[error("The slot is invalid")]
    InvalidSlot = sys::JVMTI_ERROR_INVALID_SLOT,

    /// The thread is invalid.
    #[error("The thread is invalid")]
    InvalidThread = sys::JVMTI_ERROR_INVALID_THREAD,

    /// The thread group is invalid.
    #[error("The thread group is invalid")]
    InvalidThreadGroup = sys::JVMTI_ERROR_INVALID_THREAD_GROUP,

    /// The type state is invalid.
    #[error("The type state is invalid")]
    InvalidTypeState = sys::JVMTI_ERROR_INVALID_TYPESTATE,

    /// The required capability is not possessed.
    #[error("The required capability is not possessed")]
    MustPossessCapability = sys::JVMTI_ERROR_MUST_POSSESS_CAPABILITY,

    /// The names do not match.
    #[error("The names do not match")]
    NamesDontMatch = sys::JVMTI_ERROR_NAMES_DONT_MATCH,

    /// The method is native.
    #[error("The method is native")]
    NativeMethod = sys::JVMTI_ERROR_NATIVE_METHOD,

    /// There are no more frames.
    #[error("There are no more frames")]
    NoMoreFrames = sys::JVMTI_ERROR_NO_MORE_FRAMES,

    /// The requested item is not available.
    #[error("The requested item is not available")]
    NotAvailable = sys::JVMTI_ERROR_NOT_AVAILABLE,

    /// The requested item was not found.
    #[error("The requested item was not found")]
    NotFound = sys::JVMTI_ERROR_NOT_FOUND,

    /// The current thread is not the owner of the monitor.
    #[error("The current thread is not the owner of the monitor")]
    NotMonitorOwner = sys::JVMTI_ERROR_NOT_MONITOR_OWNER,

    /// The frame is opaque.
    #[error("The frame is opaque")]
    OpaqueFrame = sys::JVMTI_ERROR_OPAQUE_FRAME,

    /// The thread is not alive.
    #[error("The thread is not alive")]
    ThreadNotAlive = sys::JVMTI_ERROR_THREAD_NOT_ALIVE,

    /// The thread is not suspended.
    #[error("The thread is not suspended")]
    ThreadNotSuspended = sys::JVMTI_ERROR_THREAD_NOT_SUSPENDED,

    /// The thread is already suspended.
    #[error("The thread is already suspended")]
    ThreadSuspended = sys::JVMTI_ERROR_THREAD_SUSPENDED,

    /// The type does not match.
    #[error("The type does not match")]
    TypeMismatch = sys::JVMTI_ERROR_TYPE_MISMATCH,

    /// The class is unmodifiable.
    #[error("The class is unmodifiable")]
    UnmodifiableClass = sys::JVMTI_ERROR_UNMODIFIABLE_CLASS,

    /// The module is unmodifiable.
    #[error("The module is unmodifiable")]
    UnmodifiableModule = sys::JVMTI_ERROR_UNMODIFIABLE_MODULE,

    /// The operation is unsupported.
    #[error("The operation is unsupported")]
    UnsupportedOperation = sys::JVMTI_ERROR_UNSUPPORTED_OPERATION,

    /// The class attribute change is unsupported during redefinition.
    #[error("The class attribute change is unsupported during redefinition")]
    UnsupportedRedefinitionClassAttributeChanged =
        sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_ATTRIBUTE_CHANGED,

    /// The class modifiers change is unsupported during redefinition.
    #[error("The class modifiers change is unsupported during redefinition")]
    UnsupportedRedefinitionClassModifiersChanged =
        sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED,

    /// The hierarchy change is unsupported during redefinition.
    #[error("The hierarchy change is unsupported during redefinition")]
    UnsupportedRedefinitionHierarchyChanged =
        sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED,

    /// Adding a method is unsupported during redefinition.
    #[error("Adding a method is unsupported during redefinition")]
    UnsupportedRedefinitionMethodAdded = sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED,

    /// Deleting a method is unsupported during redefinition.
    #[error("Deleting a method is unsupported during redefinition")]
    UnsupportedRedefinitionMethodDeleted = sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED,

    /// The method modifiers change is unsupported during redefinition.
    #[error("The method modifiers change is unsupported during redefinition")]
    UnsupportedRedefinitionMethodModifiersChanged =
        sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED,

    /// The schema change is unsupported during redefinition.
    #[error("The schema change is unsupported during redefinition")]
    UnsupportedRedefinitionSchemaChanged = sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED,

    /// The version is unsupported.
    #[error("The version is unsupported")]
    UnsupportedVersion = sys::JVMTI_ERROR_UNSUPPORTED_VERSION,
}
