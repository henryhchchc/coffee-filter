use crate::{jvm::errors::JvmTIError, sys};

pub type JvmTINaiveCallResult = Result<(), JvmTIError>;

#[inline]
pub(crate) fn native_call_result(err_code: sys::jvmtiError) -> JvmTINaiveCallResult {
    match err_code {
        sys::JVMTI_ERROR_NONE => Ok(()),
        code => {
            // SAFETY: `return_code` is a valid `sys::jvmtiError` value.
            let error = unsafe { JvmTIError::from_error_code_unchecked(code) };
            Err(error)
        }
    }
}
