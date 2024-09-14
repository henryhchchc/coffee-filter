use crate::sys;

/// The version of the JVM Tool Interface (JVM TI).
#[derive(Debug)]
#[repr(C)]
pub struct Version(sys::jint);

impl Version {
    /// JVM TI version 1.0.
    pub const JVMTI_1_0: Self = Self(sys::JVMTI_VERSION_1_0);
    /// JVM TI version 1.1.
    pub const JVMTI_1_1: Self = Self(sys::JVMTI_VERSION_1_1);
    /// JVM TI version 1.2.
    pub const JVMTI_1_2: Self = Self(sys::JVMTI_VERSION_1_2);
    /// JVM TI version 9.
    pub const JVMTI_9: Self = Self(sys::JVMTI_VERSION_9);
    /// JVM TI version 11.
    pub const JVMTI_11: Self = Self(sys::JVMTI_VERSION_11);
    /// JVM TI version 19.
    pub const JVMTI_19: Self = Self(sys::JVMTI_VERSION_19);
    /// JVM TI version 21.
    pub const JVMTI_21: Self = Self(sys::JVMTI_VERSION_21);
    /// The current JVM TI version.
    pub const CURRENT: Self = Self(sys::JVMTI_VERSION);

    /// The interface type.
    #[must_use]
    pub const fn interface_type(&self) -> i32 {
        self.0 & sys::JVMTI_VERSION_MASK_INTERFACE_TYPE
    }

    /// The major version number.
    #[must_use]
    pub const fn major(&self) -> i16 {
        ((self.0 & sys::JVMTI_VERSION_MASK_MAJOR) >> sys::JVMTI_VERSION_SHIFT_MAJOR) as i16
    }

    /// The minor version number.
    #[must_use]
    pub const fn minor(&self) -> i8 {
        ((self.0 & sys::JVMTI_VERSION_MASK_MINOR) >> sys::JVMTI_VERSION_SHIFT_MINOR) as i8
    }

    /// The micro version number.
    #[must_use]
    pub const fn micro(&self) -> i8 {
        ((self.0 & sys::JVMTI_VERSION_MASK_MICRO) >> sys::JVMTI_VERSION_SHIFT_MICRO) as i8
    }
}

#[allow(clippy::from_over_into)]
impl Into<sys::jint> for Version {
    fn into(self) -> sys::jint {
        self.0
    }
}
