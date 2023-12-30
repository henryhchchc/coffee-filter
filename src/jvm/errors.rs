//! APIs for errors returned by the JVM Tool Interface (JVM TI).
use crate::sys;

/// The error occurrec when calling a JVM Tool Interface (JVM TI) function.
/// See [the JVMTI documentation](https://docs.oracle.com/en/java/javase/21/docs/specs/jvmti.html#jvmtierror) for more information.
#[derive(Debug, thiserror::Error)]
#[repr(u32)]
#[allow(missing_docs)]
pub enum JvmTIError {
    // # Universal Errors
    #[error("JVMTI_ERROR_OUT_OF_MEMORY")]
    OutOfMemory = sys::JVMTI_ERROR_OUT_OF_MEMORY,

    #[error("JVMTI_ERROR_ACCESS_DENIED")]
    AccessDenied = sys::JVMTI_ERROR_ACCESS_DENIED,

    #[error("JVMTI_ERROR_UNATTACHED_THREAD")]
    UnAttachedThread = sys::JVMTI_ERROR_UNATTACHED_THREAD,

    #[error("JVMTI_ERROR_INVALID_ENVIRONMENT")]
    InvalidEnvironment = sys::JVMTI_ERROR_INVALID_ENVIRONMENT,

    #[error("JVMTI_ERROR_WRONG_PHASE")]
    WrongPhase = sys::JVMTI_ERROR_WRONG_PHASE,

    #[error("JVMTI_ERROR_INTERNAL")]
    Internal = sys::JVMTI_ERROR_INTERNAL,

    // # Function Specific Required Errors
    #[error("JVMTI_ERROR_INVALID_PRIORITY")]
    InvalidPriority = sys::JVMTI_ERROR_INVALID_PRIORITY,

    #[error("JVMTI_ERROR_THREAD_NOT_SUSPENDED")]
    ThreadNotSuspended = sys::JVMTI_ERROR_THREAD_NOT_SUSPENDED,

    #[error("JVMTI_ERROR_THREAD_SUSPENDED")]
    ThreadSuspended = sys::JVMTI_ERROR_THREAD_SUSPENDED,

    #[error("JVMTI_ERROR_THREAD_NOT_ALIVE")]
    ThreadNotAlive = sys::JVMTI_ERROR_THREAD_NOT_ALIVE,

    #[error("JVMTI_ERROR_CLASS_NOT_PREPARED")]
    ClassNotPrepared = sys::JVMTI_ERROR_CLASS_NOT_PREPARED,

    #[error("JVMTI_ERROR_NO_MORE_FRAMES")]
    NoMoreFrames = sys::JVMTI_ERROR_NO_MORE_FRAMES,

    #[error("JVMTI_ERROR_OPAQUE_FRAME")]
    OpaqueFrame = sys::JVMTI_ERROR_OPAQUE_FRAME,

    #[error("JVMTI_ERROR_DUPLICATE")]
    Duplicate = sys::JVMTI_ERROR_DUPLICATE,

    #[error("JVMTI_ERROR_NOT_FOUND")]
    NotFound = sys::JVMTI_ERROR_NOT_FOUND,

    #[error("JVMTI_ERROR_NOT_MONITOR_OWNER")]
    NotMonitorOwner = sys::JVMTI_ERROR_NOT_MONITOR_OWNER,

    #[error("JVMTI_ERROR_INTERRUPT")]
    Interrupt = sys::JVMTI_ERROR_INTERRUPT,

    #[error("JVMTI_ERROR_UNMODIFIABLE_CLASS")]
    UnmodifiableClass = sys::JVMTI_ERROR_UNMODIFIABLE_CLASS,

    #[error("JVMTI_ERROR_UNMODIFIABLE_MODULE")]
    UnmodifiableModule = sys::JVMTI_ERROR_UNMODIFIABLE_MODULE,

    #[error("JVMTI_ERROR_NOT_AVAILABLE")]
    NotAvailable = sys::JVMTI_ERROR_NOT_AVAILABLE,

    #[error("JVMTI_ERROR_ABSENT_INFORMATION")]
    AbsentInformation = sys::JVMTI_ERROR_ABSENT_INFORMATION,

    #[error("JVMTI_ERROR_INVALID_EVENT_TYPE")]
    InvalidEventType = sys::JVMTI_ERROR_INVALID_EVENT_TYPE,

    #[error("JVMTI_ERROR_NATIVE_METHOD")]
    NativeMethod = sys::JVMTI_ERROR_NATIVE_METHOD,

    #[error("JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED")]
    ClassLoaderUnsupported = sys::JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED,

    // # Function Specific Agent Errors
    #[error("JVMTI_ERROR_INVALID_THREAD")]
    InvalidThread = sys::JVMTI_ERROR_INVALID_THREAD,

    #[error("JVMTI_ERROR_INVALID_FIELDID")]
    InvalidFieldId = sys::JVMTI_ERROR_INVALID_FIELDID,

    #[error("JVMTI_ERROR_INVALID_MODULE")]
    InvalidModule = sys::JVMTI_ERROR_INVALID_MODULE,

    #[error("JVMTI_ERROR_INVALID_METHODID")]
    InvalidMethodId = sys::JVMTI_ERROR_INVALID_METHODID,

    #[error("JVMTI_ERROR_INVALID_LOCATION")]
    InvalidLocation = sys::JVMTI_ERROR_INVALID_LOCATION,

    #[error("JVMTI_ERROR_INVALID_OBJECT")]
    InvalidObject = sys::JVMTI_ERROR_INVALID_OBJECT,

    #[error("JVMTI_ERROR_INVALID_CLASS")]
    InvalidClass = sys::JVMTI_ERROR_INVALID_CLASS,

    #[error("JVMTI_ERROR_TYPE_MISMATCH")]
    TypeMismatch = sys::JVMTI_ERROR_TYPE_MISMATCH,

    #[error("JVMTI_ERROR_INVALID_SLOT")]
    InvalidSlot = sys::JVMTI_ERROR_INVALID_SLOT,

    #[error("JVMTI_ERROR_MUST_POSSESS_CAPABILITY")]
    MustPossessCapability = sys::JVMTI_ERROR_MUST_POSSESS_CAPABILITY,

    #[error("JVMTI_ERROR_INVALID_THREAD_GROUP")]
    InvalidThreadGroup = sys::JVMTI_ERROR_INVALID_THREAD_GROUP,

    #[error("JVMTI_ERROR_INVALID_MONITOR")]
    InvalidMonitor = sys::JVMTI_ERROR_INVALID_MONITOR,

    #[error("JVMTI_ERROR_ILLEGAL_ARGUMENT")]
    IllegalArgument = sys::JVMTI_ERROR_ILLEGAL_ARGUMENT,

    #[error("JVMTI_ERROR_INVALID_TYPESTATE")]
    InvalidTypeState = sys::JVMTI_ERROR_INVALID_TYPESTATE,

    #[error("JVMTI_ERROR_UNSUPPORTED_VERSION")]
    UnsupportedVersion = sys::JVMTI_ERROR_UNSUPPORTED_VERSION,

    #[error("JVMTI_ERROR_INVALID_CLASS_FORMAT")]
    InvalidClassFormat = sys::JVMTI_ERROR_INVALID_CLASS_FORMAT,

    #[error("JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION")]
    CircularClassDefinition = sys::JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION,

    #[error("JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED")]
    UnsupportedRedefinitionMethodAdded = sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED,

    #[error("JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED")]
    UnsupportedRedefinitionSchemaChanged = sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED,

    #[error("JVMTI_ERROR_FAILS_VERIFICATION")]
    FailsVerification = sys::JVMTI_ERROR_FAILS_VERIFICATION,

    #[error("JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED")]
    UnsupportedRedefinitionHierarchyChanged =
        sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED,

    #[error("JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED")]
    UnsupportedRedefinitionMethodDeleted = sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED,

    #[error("JVMTI_ERROR_NAMES_DONT_MATCH")]
    NamesDontMatch = sys::JVMTI_ERROR_NAMES_DONT_MATCH,

    #[error("JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED")]
    UnsupportedRedefinitionClassModifiersChanged =
        sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED,

    #[error("JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED")]
    UnsupportedRedefinitionMethodModifiersChanged =
        sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED,

    #[error("JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_ATTRIBUTE_CHANGED")]
    UnsupportedRedefinitionClassAttributeChanged =
        sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_ATTRIBUTE_CHANGED,

    #[error("JVMTI_ERROR_UNSUPPORTED_OPERATION")]
    UnsupportedOperation = sys::JVMTI_ERROR_UNSUPPORTED_OPERATION,
}

impl JvmTIError {
    pub(crate) unsafe fn from_error_code_unchecked(code: sys::jvmtiError) -> Self {
        std::mem::transmute(code)
    }
}
