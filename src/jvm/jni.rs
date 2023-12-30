use crate::sys;

#[derive(Debug)]
pub struct JNI {
    jni_ptr: *mut sys::JNIEnv,
}

impl JNI {
    pub unsafe fn from_ptr(jni_ptr: *mut sys::JNIEnv) -> Self {
        assert!(!jni_ptr.is_null(), "The JNI pointer is null");
        Self { jni_ptr }
    }
}
