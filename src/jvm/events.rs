//! APIs for working with event callbacks.

use std::{
    ffi::{c_char, c_uchar, CStr, OsStr, OsString},
    mem::size_of,
    os::unix::prelude::OsStrExt,
};

use crate::{
    jvm::{class::Class, objects::Object},
    macros::call_jvmti,
    sys,
};

use super::{errors::JvmTIError, jni::JNI, threads::Thread, Jvm};

#[derive(Debug)]
#[repr(u32)]
pub enum JvmTIEvent {
    VMInit = sys::JVMTI_EVENT_VM_INIT,
    VMDeath = sys::JVMTI_EVENT_VM_DEATH,
    VMStart = sys::JVMTI_EVENT_VM_START,
    ThreadStart = sys::JVMTI_EVENT_THREAD_START,
    ThreadEnd = sys::JVMTI_EVENT_THREAD_END,
    ClassFileLoadHook = sys::JVMTI_EVENT_CLASS_FILE_LOAD_HOOK,
    ClassLoad = sys::JVMTI_EVENT_CLASS_LOAD,
    ClassPrepare = sys::JVMTI_EVENT_CLASS_PREPARE,
    Exception = sys::JVMTI_EVENT_EXCEPTION,
    ExceptionCatch = sys::JVMTI_EVENT_EXCEPTION_CATCH,
    SingleStep = sys::JVMTI_EVENT_SINGLE_STEP,
    FramePop = sys::JVMTI_EVENT_FRAME_POP,
    Breakpoint = sys::JVMTI_EVENT_BREAKPOINT,
    FieldAccess = sys::JVMTI_EVENT_FIELD_ACCESS,
    FieldModification = sys::JVMTI_EVENT_FIELD_MODIFICATION,
    MethodEntry = sys::JVMTI_EVENT_METHOD_ENTRY,
    MethodExit = sys::JVMTI_EVENT_METHOD_EXIT,
    NativeMethodBind = sys::JVMTI_EVENT_NATIVE_METHOD_BIND,
    CompiledMethodLoad = sys::JVMTI_EVENT_COMPILED_METHOD_LOAD,
    CompiledMethodUnload = sys::JVMTI_EVENT_COMPILED_METHOD_UNLOAD,
    DynamicCodeGenerated = sys::JVMTI_EVENT_DYNAMIC_CODE_GENERATED,
    DataDumpRequest = sys::JVMTI_EVENT_DATA_DUMP_REQUEST,
    MonitorWait = sys::JVMTI_EVENT_MONITOR_WAIT,
    MonitorWaited = sys::JVMTI_EVENT_MONITOR_WAITED,
    MonitorContendedEnter = sys::JVMTI_EVENT_MONITOR_CONTENDED_ENTER,
    MonitorContendedEntered = sys::JVMTI_EVENT_MONITOR_CONTENDED_ENTERED,
    ResourceExhausted = sys::JVMTI_EVENT_RESOURCE_EXHAUSTED,
    GarbageCollectionStart = sys::JVMTI_EVENT_GARBAGE_COLLECTION_START,
    GarbageCollectionFinish = sys::JVMTI_EVENT_GARBAGE_COLLECTION_FINISH,
    ObjectFree = sys::JVMTI_EVENT_OBJECT_FREE,
    VMObjectAlloc = sys::JVMTI_EVENT_VM_OBJECT_ALLOC,
    SampledObjectAlloc = sys::JVMTI_EVENT_SAMPLED_OBJECT_ALLOC,
    VirtualThreadStart = sys::JVMTI_EVENT_VIRTUAL_THREAD_START,
    VirtualThreadEnd = sys::JVMTI_EVENT_VIRTUAL_THREAD_END,
}

impl Jvm {
    /// Enables the given event.
    /// See [`SetEventNotificationMode`](https://docs.oracle.com/javase/8/docs/platform/jvmti/jvmti.html#SetEventNotificationMode).
    /// # Errors
    /// See [`JvmTIError`] for more information.
    pub fn enable_event(
        &mut self,
        event_type: JvmTIEvent,
        thread: Option<Thread<'_>>,
    ) -> Result<(), JvmTIError> {
        let thread_ptr = thread.map_or(std::ptr::null_mut(), Thread::into_raw);
        // SAFETY: `self.jvmti_ptr` is a valid `sys::jvmtiEnv` because of the API restrictions.
        unsafe {
            call_jvmti!(
                self.jvmti_ptr,
                SetEventNotificationMode,
                sys::JVMTI_ENABLE,
                event_type as sys::jvmtiEvent,
                thread_ptr
            )
        }?;
        Ok(())
    }

    pub(super) fn update_native_callback(&self) -> Result<(), JvmTIError> {
        let callbacks = Box::new(self.callbacks.c_callbacks());
        // SAFETY: `self.jvmti_ptr` is a valid `sys::jvmtiEnv` because of the API restrictions.
        unsafe {
            call_jvmti!(
                self.jvmti_ptr,
                SetEventCallbacks,
                Box::into_raw(callbacks),
                size_of::<sys::jvmtiEventCallbacks>() as sys::jint
            )
        }?;
        Ok(())
    }
}

impl EventCallbacks {
    unsafe extern "C" fn vm_init_callback(
        jvmti_env: *mut sys::jvmtiEnv,
        jni_env: *mut sys::JNIEnv,
        thread: sys::jthread,
    ) {
        let jvm = Jvm::from_ptr(jvmti_env);
        let jni = JNI::from_ptr(jni_env);
        let thread = Thread::from_ptr(jvm, thread);
        if let Some(ref callback) = jvm.callbacks.vm_init {
            callback(jvm, &jni, &thread);
        }
    }

    unsafe extern "C" fn vm_death_callback(
        jvmti_env: *mut sys::jvmtiEnv,
        jni_env: *mut sys::JNIEnv,
    ) {
        let jvm = Jvm::from_ptr(jvmti_env);
        let jni = JNI::from_ptr(jni_env);
        if let Some(ref callback) = jvm.callbacks.vm_death {
            callback(jvm, &jni);
        }
    }

    unsafe extern "C" fn vm_start_callback(
        jvmti_env: *mut sys::jvmtiEnv,
        jni_env: *mut sys::JNIEnv,
    ) {
        let jvm = Jvm::from_ptr(jvmti_env);
        let jni = JNI::from_ptr(jni_env);
        if let Some(ref callback) = jvm.callbacks.vm_start {
            callback(jvm, &jni);
        }
    }

    unsafe extern "C" fn thread_start_callback(
        jvmti_env: *mut sys::jvmtiEnv,
        jni_env: *mut sys::JNIEnv,
        thread: sys::jthread,
    ) {
        let jvm = Jvm::from_ptr(jvmti_env);
        let jni = JNI::from_ptr(jni_env);
        let thread = Thread::from_ptr(jvm, thread);
        if let Some(ref callback) = jvm.callbacks.thread_start {
            callback(jvm, &jni, &thread);
        }
    }
    unsafe extern "C" fn thread_end_callback(
        jvmti_env: *mut sys::jvmtiEnv,
        jni_env: *mut sys::JNIEnv,
        thread: sys::jthread,
    ) {
        let jvm = Jvm::from_ptr(jvmti_env);
        let jni = JNI::from_ptr(jni_env);
        let thread = Thread::from_ptr(jvm, thread);
        if let Some(ref callback) = jvm.callbacks.thread_end {
            callback(jvm, &jni, &thread);
        }
    }

    unsafe extern "C" fn class_file_load_hook_callback(
        jvmti_env: *mut sys::jvmtiEnv,
        jni_env: *mut sys::JNIEnv,
        class_being_redefined: sys::jclass,
        loader: sys::jobject,
        name: *const c_char,
        protection_domain: sys::jobject,
        class_data_len: sys::jint,
        class_data: *const c_uchar,
        new_class_data_len: *mut sys::jint,
        new_class_data: *mut *mut c_uchar,
    ) {
        let jvm = Jvm::from_ptr(jvmti_env);
        let jni = JNI::from_ptr(jni_env);
        let class_being_redefined = Class::from_ptr(jvm, class_being_redefined);
        let name = if name.is_null() {
            None
        } else {
            Some(OsStr::from_bytes(CStr::from_ptr(name).to_bytes()))
        };
        let class_loader = Object::from_ptr(jvm, loader);
        let protection_domain = Object::from_ptr(jvm, protection_domain);
        let class_data = Vec::from(std::slice::from_raw_parts(
            class_data,
            class_data_len as usize,
        ));
        if let Some(bytes) = jvm.callbacks.class_file_load_hook.as_ref().and_then(|it| {
            it(
                jvm,
                &jni,
                &class_being_redefined,
                name,
                &class_loader,
                &protection_domain,
                class_data,
            )
        }) {
            *new_class_data_len = bytes.len() as sys::jint;
            *new_class_data = bytes.leak().as_mut_ptr();
        } else {
            *new_class_data_len = 0;
            *new_class_data = std::ptr::null_mut();
        }
    }

    unsafe extern "C" fn class_load_callback(
        jvmti_env: *mut sys::jvmtiEnv,
        jni_env: *mut sys::JNIEnv,
        thread: sys::jthread,
        klass: sys::jclass,
    ) {
        let jvm = Jvm::from_ptr(jvmti_env);
        let jni = JNI::from_ptr(jni_env);
        let thread = Thread::from_ptr(jvm, thread);
        let class = Class::from_ptr(jvm, klass);
        if let Some(ref callback) = jvm.callbacks.class_load {
            callback(jvm, &jni, &thread, &class);
        }
    }

    unsafe extern "C" fn class_prepare_callback(
        jvmti_env: *mut sys::jvmtiEnv,
        jni_env: *mut sys::JNIEnv,
        thread: sys::jthread,
        klass: sys::jclass,
    ) {
        let jvm = Jvm::from_ptr(jvmti_env);
        let jni = JNI::from_ptr(jni_env);
        let thread = Thread::from_ptr(jvm, thread);
        let class = Class::from_ptr(jvm, klass);
        if let Some(ref callback) = jvm.callbacks.class_prepare {
            callback(jvm, &jni, &thread, &class);
        }
    }
}

#[derive(Default)]
#[non_exhaustive]
pub struct EventCallbacks {
    // pub vm_init: Option<Box<dyn Fn(&Jvm, &JNI, &Thread<'_>) + Send + Sync>>,
    pub vm_init: Option<Box<dyn Fn(&Jvm, &JNI, &Thread<'_>)>>,
    pub vm_death: Option<Box<dyn Fn(&Jvm, &JNI)>>,
    pub vm_start: Option<Box<dyn Fn(&Jvm, &JNI)>>,
    pub thread_start: Option<Box<dyn Fn(&Jvm, &JNI, &Thread<'_>)>>,
    pub thread_end: Option<Box<dyn Fn(&Jvm, &JNI, &Thread<'_>)>>,
    pub class_file_load_hook: Option<
        Box<
            dyn Fn(
                &Jvm,
                &JNI,
                &Class<'_>,
                Option<&OsStr>,
                &Object<'_>,
                &Object<'_>,
                Vec<u8>,
            ) -> Option<Vec<u8>>,
        >,
    >,
    pub class_load: Option<Box<dyn Fn(&Jvm, &JNI, &Thread<'_>, &Class<'_>)>>,
    pub class_prepare: Option<Box<dyn Fn(&Jvm, &JNI, &Thread<'_>, &Class<'_>)>>,
}

impl EventCallbacks {
    pub(crate) fn c_callbacks(&self) -> sys::jvmtiEventCallbacks {
        sys::jvmtiEventCallbacks {
            VMInit: Some(Self::vm_init_callback),
            VMDeath: Some(Self::vm_death_callback),
            VMStart: Some(Self::vm_start_callback),
            ThreadStart: Some(Self::thread_start_callback),
            ThreadEnd: Some(Self::thread_end_callback),
            ClassFileLoadHook: Some(Self::class_file_load_hook_callback),
            ClassLoad: Some(Self::class_load_callback),
            ClassPrepare: Some(Self::class_prepare_callback),
            Exception: None,
            ExceptionCatch: None,
            SingleStep: None,
            FramePop: None,
            Breakpoint: None,
            FieldAccess: None,
            FieldModification: None,
            MethodEntry: None,
            MethodExit: None,
            NativeMethodBind: None,
            CompiledMethodLoad: None,
            CompiledMethodUnload: None,
            DynamicCodeGenerated: None,
            DataDumpRequest: None,
            reserved72: None,
            MonitorWait: None,
            MonitorWaited: None,
            MonitorContendedEnter: None,
            MonitorContendedEntered: None,
            reserved77: None,
            reserved78: None,
            reserved79: None,
            ResourceExhausted: None,
            GarbageCollectionStart: None,
            GarbageCollectionFinish: None,
            ObjectFree: None,
            VMObjectAlloc: None,
            reserved85: None,
            SampledObjectAlloc: None,
            VirtualThreadStart: None,
            VirtualThreadEnd: None,
        }
    }
}
