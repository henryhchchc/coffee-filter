use crate::{
    jni::{self, objects::JThread},
    macros::unsafe_jvmti,
    sys,
    utils::jvmti_result,
};

use super::{Env, Error};

pub mod callbacks;

impl Env {
    pub fn set_event_callbacks(&self, callbacks: callbacks::JVMTICallbacks) -> Result<(), Error> {
        let sys_callbacks = callbacks.clone().into_sys();
        self.metadata()?
            .write()
            .expect("The metadata is poisioned")
            .callbacks = Some(callbacks);
        let errno = unsafe_jvmti!(
            self.ptr,
            SetEventCallbacks,
            &sys_callbacks,
            size_of_val(&sys_callbacks) as sys::jint
        );
        jvmti_result(errno, || ())
    }

    pub fn set_event_notification_mode(
        &self,
        enable: bool,
        event: Event,
        thread: Option<jni::objects::JThread>,
    ) -> Result<(), Error> {
        let mode = if enable {
            sys::JVMTI_ENABLE
        } else {
            sys::JVMTI_DISABLE
        };
        let event = event as sys::jvmtiEvent;
        let thread = thread
            .map(JThread::into_inner)
            .unwrap_or(std::ptr::null_mut());
        let errno = unsafe_jvmti!(self.ptr, SetEventNotificationMode, mode, event, thread);
        jvmti_result(errno, || ())
    }
}

/// JVMTI event.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    /// VM initialization event.
    /// This event is sent when the VM has initialized.
    VMInit = sys::JVMTI_EVENT_VM_INIT,
    /// VM death event.
    /// This event is sent when the VM is about to terminate.
    VMDeath = sys::JVMTI_EVENT_VM_DEATH,
    /// Thread start event.
    /// This event is sent when a thread is started.
    ThreadStart = sys::JVMTI_EVENT_THREAD_START,
    /// Thread end event.
    /// This event is sent when a thread is about to end.
    ThreadEnd = sys::JVMTI_EVENT_THREAD_END,
    /// Class file load hook event.
    /// This event is sent when a class file is being loaded.
    ClassFileLoadHook = sys::JVMTI_EVENT_CLASS_FILE_LOAD_HOOK,
    /// Class load event.
    /// This event is sent when a class is loaded.
    ClassLoad = sys::JVMTI_EVENT_CLASS_LOAD,
    /// Class prepare event.
    /// This event is sent when a class is prepared.
    ClassPrepare = sys::JVMTI_EVENT_CLASS_PREPARE,
    /// VM start event.
    /// This event is sent when the VM starts.
    VMStart = sys::JVMTI_EVENT_VM_START,
    /// Exception event.
    /// This event is sent when an exception is thrown.
    Exception = sys::JVMTI_EVENT_EXCEPTION,
    /// Exception catch event.
    /// This event is sent when an exception is caught.
    ExceptionCatch = sys::JVMTI_EVENT_EXCEPTION_CATCH,
    /// Single step event.
    /// This event is sent when a single step is completed.
    SingleStep = sys::JVMTI_EVENT_SINGLE_STEP,
    /// Frame pop event.
    /// This event is sent when a frame is popped.
    FramePop = sys::JVMTI_EVENT_FRAME_POP,
    /// Breakpoint event.
    /// This event is sent when a breakpoint is hit.
    Breakpoint = sys::JVMTI_EVENT_BREAKPOINT,
    /// Field access event.
    /// This event is sent when a field is accessed.
    FieldAccess = sys::JVMTI_EVENT_FIELD_ACCESS,
    /// Field modification event.
    /// This event is sent when a field is modified.
    FieldModification = sys::JVMTI_EVENT_FIELD_MODIFICATION,
    /// Method entry event.
    /// This event is sent when a method is entered.
    MethodEntry = sys::JVMTI_EVENT_METHOD_ENTRY,
    /// Method exit event.
    /// This event is sent when a method is exited.
    MethodExit = sys::JVMTI_EVENT_METHOD_EXIT,
    /// Native method bind event.
    /// This event is sent when a native method is bound.
    NativeMethodBind = sys::JVMTI_EVENT_NATIVE_METHOD_BIND,
    /// Compiled method load event.
    /// This event is sent when a compiled method is loaded.
    CompiledMethodLoad = sys::JVMTI_EVENT_COMPILED_METHOD_LOAD,
    /// Compiled method unload event.
    /// This event is sent when a compiled method is unloaded.
    CompiledMethodUnload = sys::JVMTI_EVENT_COMPILED_METHOD_UNLOAD,
    /// Dynamic code generated event.
    /// This event is sent when dynamic code is generated.
    DynamicCodeGenerated = sys::JVMTI_EVENT_DYNAMIC_CODE_GENERATED,
    /// Data dump request event.
    /// This event is sent when a data dump is requested.
    DataDumpRequest = sys::JVMTI_EVENT_DATA_DUMP_REQUEST,
    /// Monitor wait event.
    /// This event is sent when a thread is waiting on a monitor.
    MonitorWait = sys::JVMTI_EVENT_MONITOR_WAIT,
    /// Monitor waited event.
    /// This event is sent when a thread has finished waiting on a monitor.
    MonitorWaited = sys::JVMTI_EVENT_MONITOR_WAITED,
    /// Monitor contended enter event.
    /// This event is sent when a thread is about to enter a contended monitor.
    MonitorContendedEnter = sys::JVMTI_EVENT_MONITOR_CONTENDED_ENTER,
    /// Monitor contended entered event.
    /// This event is sent when a thread has entered a contended monitor.
    MonitorContendedEntered = sys::JVMTI_EVENT_MONITOR_CONTENDED_ENTERED,
    /// Resource exhausted event.
    /// This event is sent when the VM is about to throw an OutOfMemoryError.
    ResourceExhausted = sys::JVMTI_EVENT_RESOURCE_EXHAUSTED,
    /// Garbage collection start event.
    /// This event is sent when garbage collection starts.
    GarbageCollectionStart = sys::JVMTI_EVENT_GARBAGE_COLLECTION_START,
    /// Garbage collection finish event.
    /// This event is sent when garbage collection finishes.
    GarbageCollectionFinish = sys::JVMTI_EVENT_GARBAGE_COLLECTION_FINISH,
    /// Object free event.
    /// This event is sent when an object is freed.
    ObjectFree = sys::JVMTI_EVENT_OBJECT_FREE,
    /// VM object allocation event.
    /// This event is sent when an object is allocated.
    VMObjectAlloc = sys::JVMTI_EVENT_VM_OBJECT_ALLOC,
}
