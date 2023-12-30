use crate::sys;

use super::Jvm;

#[derive(Debug)]
pub struct Object<'j> {
    jvm: &'j Jvm,
    jobject: sys::jobject,
}

impl Object<'_> {
    pub(crate) unsafe fn from_ptr<'j>(jvm: &'j Jvm, jobject: sys::jobject) -> Object<'j> {
        assert!(!jobject.is_null(), "The object pointer must not be null");
        Object { jvm, jobject }
    }
}
