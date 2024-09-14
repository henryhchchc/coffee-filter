use std::mem::MaybeUninit;

use crate::{macros::unsafe_jvmti, sys, utils::jvmti_result};

use super::{Env, Error};

#[derive(Debug, derive_more::TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum Phase {
    OnLoad = sys::JVMTI_PHASE_ONLOAD,
    Primordial = sys::JVMTI_PHASE_PRIMORDIAL,
    Start = sys::JVMTI_PHASE_START,
    Live = sys::JVMTI_PHASE_LIVE,
    Dead = sys::JVMTI_PHASE_DEAD,
}

#[derive(Debug)]
#[repr(i32)]
pub enum VeroseFlag {
    OTHER = sys::JVMTI_VERBOSE_OTHER,
    CLASS = sys::JVMTI_VERBOSE_CLASS,
    GC = sys::JVMTI_VERBOSE_GC,
    JNI = sys::JVMTI_VERBOSE_JNI,
}

impl Env {
    /// Get the current phase of the JVM TI environment.
    /// # Errors
    /// See [`Error`] for more information.
    #[expect(clippy::missing_panics_doc, reason = "Garenteed by JVMTI API")]
    pub fn get_phase(&self) -> Result<Phase, Error> {
        let mut phase: MaybeUninit<sys::jvmtiPhase> = MaybeUninit::uninit();
        let errno = unsafe_jvmti!(self.ptr, GetPhase, phase.as_mut_ptr());
        jvmti_result(errno, || {
            let phase_repr = unsafe { phase.assume_init() };
            phase_repr.try_into().unwrap()
        })
    }

    /// Sets the verbose flag of the JVM.
    /// # Errors
    /// See [`Error`] for more information.
    pub fn set_verbose_flag(&self, flag: VeroseFlag, value: bool) -> Result<(), Error> {
        let errno = unsafe_jvmti!(self.ptr, SetVerboseFlag, flag as i32, value.into());
        jvmti_result(errno, || ())
    }
}
