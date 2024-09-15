use crate::sys;

#[derive(Debug)]
pub struct JThread {
    inner: sys::jthread,
}

impl JThread {
    pub(crate) fn from_inner(inner: sys::jthread) -> Self {
        Self { inner }
    }

    pub(crate) fn into_inner(self) -> sys::jthread {
        self.inner
    }
}
