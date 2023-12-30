//! General JVM TI functionalities.
//! See [the JVMTI documentation](https://docs.oracle.com/en/java/javase/21/docs/specs/jvmti.html#general) for more information.
use std::mem::MaybeUninit;

use crate::{macros::call_jvmti, sys};

use super::{errors::JvmTIError, Jvm};

/// The phase of VM execution.
#[derive(Debug)]
#[repr(u32)]
pub enum JvmTiPhase {
    /// While in the `Agent_OnLoad` or, for statically linked agents, the `Agent_OnLoad_<agent-lib-name>` function.
    OnLoad = sys::JVMTI_PHASE_ONLOAD,
    /// Between return from the `Agent_OnLoad` or `Agent_OnLoad_<agent-lib-name>` function and the `VMStart` event.
    Primordial = sys::JVMTI_PHASE_PRIMORDIAL,
    /// When the `VMStart` event is sent and until the `VMInit` event is sent.
    Start = sys::JVMTI_PHASE_START,
    /// After the `VMInit` event is sent and until the `VMDeath` event returns.
    Live = sys::JVMTI_PHASE_LIVE,
    /// After the `VMDeath` event returns or after start-up failure.
    Dead = sys::JVMTI_PHASE_DEAD,
}

/// The version of the JVM Tool Interface (JVM TI).
#[derive(Debug)]
#[repr(transparent)]
pub struct JvmTIVersion(u32);

impl JvmTIVersion {
    /// Crates a new [`JvmTIVersion`].
    pub const fn new(major: u16, minor: u8, micro: u8) -> Self {
        let major = major as u32;
        let minor = minor as u32;
        let micro = micro as u32;
        let version_number = sys::JVMTI_VERSION_INTERFACE_JVMTI
            | (major << sys::JVMTI_VERSION_SHIFT_MAJOR)
            | (minor << sys::JVMTI_VERSION_SHIFT_MINOR)
            | (micro << sys::JVMTI_VERSION_SHIFT_MICRO);
        Self(version_number)
    }

    /// JVM TI version 1.0.
    pub const JVMTI_1_0: Self = Self(0x30010000);
    /// JVM TI version 1.1.
    pub const JVMTI_1_1: Self = Self(0x30010100);
    /// JVM TI version 1.2.
    pub const JVMTI_1_2: Self = Self(0x30010200);
    /// JVM TI version 9.
    pub const JVMTI_9: Self = Self(0x30090000);
    /// JVM TI version 11.
    pub const JVMTI_11: Self = Self(0x300B0000);
    /// JVM TI version 19.
    pub const JVMTI_19: Self = Self(0x30130000);
    /// JVM TI version 21.
    pub const JVMTI_21: Self = Self(0x30150000);
    /// The latest JVM TI version.
    pub const LATEST: Self = Self::JVMTI_21;

    /// The interface type.
    pub const fn interface_type(&self) -> u32 {
        self.0 & sys::JVMTI_VERSION_MASK_INTERFACE_TYPE
    }

    /// The major version number.
    pub const fn major(&self) -> u16 {
        ((self.0 & sys::JVMTI_VERSION_MASK_MAJOR) >> sys::JVMTI_VERSION_SHIFT_MAJOR) as u16
    }

    /// The minor version number.
    pub const fn minor(&self) -> u8 {
        ((self.0 & sys::JVMTI_VERSION_MASK_MINOR) >> sys::JVMTI_VERSION_SHIFT_MINOR) as u8
    }

    /// The micro version number.
    pub const fn micro(&self) -> u8 {
        ((self.0 & sys::JVMTI_VERSION_MASK_MICRO) >> sys::JVMTI_VERSION_SHIFT_MICRO) as u8
    }
}

impl From<u32> for JvmTIVersion {
    fn from(version_number: u32) -> Self {
        Self(version_number)
    }
}

impl Into<sys::jint> for JvmTIVersion {
    fn into(self) -> sys::jint {
        self.0 as sys::jint
    }
}

/// The verbosity category.
/// See [the JVMTI documentation](https://docs.oracle.com/en/java/javase/21/docs/specs/jvmti.html#jvmtiVerboseFlag) for more information.
#[derive(Debug)]
#[repr(u32)]
pub enum JvmTIVerboseCategory {
    /// Verbose output other than the other categories.
    Other = sys::JVMTI_VERBOSE_OTHER,
    /// Verbose garbage collection output, like that specified with `-verbose:gc`.
    GarbageCollection = sys::JVMTI_VERBOSE_GC,
    /// Verbose class loading output, like that specified with `-verbose:class`.
    Class = sys::JVMTI_VERBOSE_CLASS,
    /// Verbose JNI output, like that specified with `-verbose:jni`.
    Jni = sys::JVMTI_VERBOSE_JNI,
}

/// The location format.
/// See [the JVMTI documentation](https://docs.oracle.com/en/java/javase/21/docs/specs/jvmti.html#jlocation) for more information.
#[derive(Debug)]
#[repr(u32)]
pub enum JvmTiJLocationFormat {
    /// Virtual machine bytecode index.
    JvmBCI = sys::JVMTI_JLOCATION_JVMBCI,
    /// Native machine program counter.
    MachinePC = sys::JVMTI_JLOCATION_MACHINEPC,
    /// Other.
    Other = sys::JVMTI_JLOCATION_OTHER,
}

impl Jvm {
    /// Returns the current phase of VM execution.
    /// See [the JVMTI documentation](https://docs.oracle.com/en/java/javase/21/docs/specs/jvmti.html#GetPhase) for more information.
    pub fn get_phase(&self) -> Result<JvmTiPhase, JvmTIError> {
        let mut phase_ptr: MaybeUninit<sys::jvmtiPhase> = MaybeUninit::uninit();
        // SAFETY: `self.jvmti_ptr` is a valid `sys::jvmtiEnv` because of the API restrictions.
        unsafe { call_jvmti!(self.jvmti_ptr, GetPhase, phase_ptr.as_mut_ptr()) }.map(|_| {
            // SAFETY: A successful result indicates that the `phase_ptr` has been initialized.
            let phase = unsafe { phase_ptr.assume_init() };
            debug_assert!(phase > 0);
            match phase {
                sys::JVMTI_PHASE_ONLOAD => JvmTiPhase::OnLoad,
                sys::JVMTI_PHASE_PRIMORDIAL => JvmTiPhase::Primordial,
                sys::JVMTI_PHASE_START => JvmTiPhase::Start,
                sys::JVMTI_PHASE_LIVE => JvmTiPhase::Live,
                sys::JVMTI_PHASE_DEAD => JvmTiPhase::Dead,
                _ => unreachable!("unexpected result from GetPhase"),
            }
        })
    }

    /// Gets the version of the JVM TI of the attached JVM.
    /// # Errors
    /// See [`JvmTIError`] for possible errors.
    pub fn get_version(&self) -> Result<JvmTIVersion, JvmTIError> {
        let mut version: MaybeUninit<sys::jint> = MaybeUninit::uninit();
        // SAFETY: `version` is a valid allocated `sys::jint` and GetVersionNumber never panics.
        unsafe { call_jvmti!(self.jvmti_ptr, GetVersionNumber, version.as_mut_ptr()) }.map(|_| {
            // SAFETY: A successful result indicates that the `version` has been initialized.
            let version_number = unsafe { version.assume_init() };
            debug_assert!(version_number > 0);
            (version_number as u32).into()
        })
    }

    /// Controls the verbosity of the output.
    /// See [the JVMTI documentation](https://docs.oracle.com/en/java/javase/21/docs/specs/jvmti.html#SetVerboseFlag) for more information.
    /// # Errors
    /// See [`JvmTIError`] for more information.
    pub fn set_verbose_flag(
        &self,
        category: JvmTIVerboseCategory,
        verbose: bool,
    ) -> Result<(), JvmTIError> {
        // SAFETY: The `category` is a valid `sys::jvmtiVerboseFlag` because of the API restrictions.
        unsafe {
            call_jvmti!(
                self.jvmti_ptr,
                SetVerboseFlag,
                category as u32,
                verbose as sys::jboolean
            )
        }
    }

    /// Gets the location format.
    /// See [the JVMTI documentation](https://docs.oracle.com/en/java/javase/21/docs/specs/jvmti.html#GetJLocationFormat) for more information.
    pub fn get_jlocation_format(&self) -> Result<JvmTiJLocationFormat, JvmTIError> {
        let mut format_ptr: MaybeUninit<sys::jvmtiJlocationFormat> = MaybeUninit::uninit();
        unsafe { call_jvmti!(self.jvmti_ptr, GetJLocationFormat, format_ptr.as_mut_ptr()) }.map(
            |_| {
                // SAFETY: A successful result indicates that the `format_ptr` has been initialized.
                let format = unsafe { format_ptr.assume_init() };
                match format {
                    sys::JVMTI_JLOCATION_JVMBCI => JvmTiJLocationFormat::JvmBCI,
                    sys::JVMTI_JLOCATION_MACHINEPC => JvmTiJLocationFormat::MachinePC,
                    sys::JVMTI_JLOCATION_OTHER => JvmTiJLocationFormat::Other,
                    _ => unreachable!("unexpected result from GetLocationFormat"),
                }
            },
        )
    }
}
