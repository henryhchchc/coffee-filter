use std::{
    ffi::{c_char, c_int, CStr, OsStr},
    os::unix::prelude::OsStrExt,
};

use crate::jvm::{general::JvmTIVersion, Jvm, JvmPointer};

/// Defines the `Agent_OnLoad` function. This macro should be used at most once in a lib.
/// # Example
/// ```rust
/// use std::ffi::OsStr;
/// use coffee_filter::{agent_on_load, jvm::general::JvmTIVersion, jvm::Jvm};
///
/// agent_on_load!(agent_onload, JvmTIVersion::LATEST);
///
/// fn agent_onload(jvm: Jvm, opts: Option<&OsStr>) -> Result<(), Box<dyn std::error::Error>> {
///     println!("Hello from coffee-filter");
///     println!("options: {:?}", opts);
///     let version = jvm.get_version()?;
///     println!(
///         "JVM TI version: {}.{}.{}",
///         version.major(),
///         version.minor(),
///         version.micro()
///     );
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! agent_on_load {
    ($callback:ident, $version:expr) => {
        #[no_mangle]
        unsafe extern "C" fn Agent_OnLoad(
            vm: $crate::jvm::JvmPointer,
            options: *const ::std::ffi::c_char,
            _reserved: *const ::std::ffi::c_void,
        ) -> ::std::ffi::c_int {
            $crate::agent_callback::on_agent_startup($callback, $version, vm, options)
        }
    };
}

/// Defines the `Agent_OnAttach` function. This macro should be used at most once in a lib.
/// See [`agent_on_load!`] for an example.
#[macro_export]
macro_rules! agent_on_attach {
    ($callback:ident, $version:expr) => {
        #[no_mangle]
        unsafe extern "C" fn Agent_OnAttach(
            vm: $crate::jvm::JvmPointer,
            options: *const ::std::ffi::c_char,
            _reserved: *const ::std::ffi::c_void,
        ) -> ::std::ffi::c_int {
            $crate::agent_callback::on_agent_startup($callback, $version, vm, options)
        }
    };
}

/// Invokes the callback function passed to [`agent_on_load!`] or [`agent_on_attach!`] by wrapping
/// around the unsafe stuff.
#[doc(hidden)]
pub unsafe fn on_agent_startup<E>(
    callback: impl FnOnce(&mut Jvm, Option<&OsStr>) -> Result<(), E>,
    version: JvmTIVersion,
    vm: JvmPointer,
    options: *const c_char,
) -> c_int {
    let options = if options.is_null() {
        None
    } else {
        // SAFETY: We already checked that `options` is not null.
        // And JVMTI guarantees that the string is null-terminated.
        let c_options = unsafe { CStr::from_ptr(options) };
        Some(OsStr::from_bytes(c_options.to_bytes()))
    };
    let jvmti = Jvm::from_jvm_ptr(vm, version).expect("Fail to initialize JVM TI");
    if callback(jvmti, options).is_ok() {
        0
    } else {
        1
    }
}
