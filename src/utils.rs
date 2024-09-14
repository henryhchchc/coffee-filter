use crate::{jvmti, sys};

pub(crate) fn jvmti_result<T>(
    errno: sys::jvmtiError,
    on_success: impl FnOnce() -> T,
) -> Result<T, jvmti::Error> {
    if errno == sys::JVMTI_ERROR_NONE {
        Ok(on_success())
    } else {
        Err(errno.try_into().expect("Granteed by the if"))
    }
}
