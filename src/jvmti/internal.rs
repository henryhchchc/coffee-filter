use super::event::callbacks::JVMTICallbacks;

#[derive(Debug, Default)]
pub(crate) struct EnvMetaData {
    pub(crate) callbacks: Option<JVMTICallbacks>,
}
