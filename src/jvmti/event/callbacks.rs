use crate::{
    jni::{self, objects::JThread},
    jvmti, sys,
};

pub type VMInit = fn(jvmti: &jvmti::Env, jni: jni::Env, thread: JThread);
pub type VMDeath = fn(jvmti: &jvmti::Env, jni: jni::Env);
pub type ThreadStart = fn(jvmti: &jvmti::Env, jni: jni::Env, thread: JThread);
pub type ThreadEnd = fn(jvmti: &jvmti::Env, jni: jni::Env, thread: JThread);

#[derive(Debug, Default, Clone)]
pub struct JVMTICallbacks {
    pub vm_init: Option<VMInit>,
    pub vm_death: Option<VMDeath>,
    pub thread_start: Option<ThreadStart>,
    pub thread_end: Option<ThreadEnd>,
}
macro_rules! invoke_callback {
    ($name: ident; $jvmti: expr, $jni: expr $(, $arg: expr)*) => {
        let metadata = $jvmti
            .metadata()
            .expect("The metadata is not set")
            .read()
            .expect("The metadata is poisioned");
        let callbacks = metadata
            .callbacks
            .as_ref()
            .expect("The callbacks are not set");
        if let Some(callback) = callbacks.$name {
            callback($jvmti, $jni $(,$arg)*);
        }
    };
}

impl JVMTICallbacks {
    unsafe extern "C" fn vm_init(
        jvmti: *mut sys::jvmtiEnv,
        jni: *mut sys::JNIEnv,
        thread: sys::jthread,
    ) {
        let jvmti_env = jvmti::Env::from_raw(jvmti);
        let jni_env = jni::Env::from_raw(jni);
        let thread = JThread::from_inner(thread);
        invoke_callback!(vm_init; &jvmti_env, jni_env, thread);
    }

    unsafe extern "C" fn vm_death(jvmti: *mut sys::jvmtiEnv, jni: *mut sys::JNIEnv) {
        let jvmti_env = jvmti::Env::from_raw(jvmti);
        let jni_env = jni::Env::from_raw(jni);
        invoke_callback!(vm_death; &jvmti_env, jni_env);
    }

    unsafe extern "C" fn thread_start(
        jvmti: *mut sys::jvmtiEnv,
        jni: *mut sys::JNIEnv,
        thread: sys::jthread,
    ) {
        let jvmti_env = jvmti::Env::from_raw(jvmti);
        let jni_env = jni::Env::from_raw(jni);
        let thread = JThread::from_inner(thread);
        invoke_callback!(thread_start; &jvmti_env, jni_env, thread);
    }

    unsafe extern "C" fn thread_end(
        jvmti: *mut sys::jvmtiEnv,
        jni: *mut sys::JNIEnv,
        thread: sys::jthread,
    ) {
        let jvmti_env = jvmti::Env::from_raw(jvmti);
        let jni_env = jni::Env::from_raw(jni);
        let thread = JThread::from_inner(thread);
        invoke_callback!(thread_end; &jvmti_env, jni_env, thread);
    }
}

impl JVMTICallbacks {
    pub(crate) fn into_sys(self) -> sys::jvmtiEventCallbacks {
        sys::jvmtiEventCallbacks {
            VMInit: self.vm_init.map(|_| Self::vm_init as _),
            VMDeath: self.vm_death.map(|_| Self::vm_death as _),
            ThreadStart: self.thread_start.map(|_| Self::thread_start as _),
            ThreadEnd: self.thread_end.map(|_| Self::thread_end as _),
            ClassFileLoadHook: None,
            ClassLoad: None,
            ClassPrepare: None,
            VMStart: None,
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
