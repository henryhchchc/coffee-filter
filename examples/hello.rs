use std::ffi::{c_char, c_void, CStr};

use coffee_filter::{
    jni::{self, objects::JThread, JavaVM, RawJavaVM},
    jvmti::{
        self,
        event::{callbacks::JVMTICallbacks, Event},
        Version,
    },
};

#[no_mangle]
unsafe extern "C" fn Agent_OnLoad(
    vm: *mut RawJavaVM,
    options: *mut c_char,
    _reserved: *mut c_void,
) {
    let jvm = JavaVM::from_raw(vm);
    let jvmti = dbg!(jvm
        .get_jvmti_env(Version::CURRENT)
        .expect("Fail to get JVMTI"));
    let options = Some(options).filter(|it| !it.is_null()).map(|it| {
        CStr::from_ptr(it)
            .to_str()
            .expect("The options is not in valid UTF-8")
    });
    if let Some(options) = options {
        println!("Options: {options}");
    }
    let memory = jvmti.allocate(1024).expect("Fail to allocate memory");
    println!("Allocated: {memory:?}");
    let phase = jvmti.get_phase().expect("Fail to get phase");
    println!("Phase: {phase:?}");
    let properties = jvmti
        .get_system_property_keys()
        .expect("Fail to get system property keys");
    for key in &properties {
        let value = jvmti
            .get_system_property(key)
            .expect("Fail to get system property");
        println!("Prop: {key:?}={value:?}");
    }
    let cap = jvmti
        .get_potential_capabilities()
        .expect("Fail to get capabilities");
    println!("Potential Capabilities: {cap:?}");

    jvmti
        .set_event_callbacks(JVMTICallbacks {
            vm_init: Some(dummy),
            thread_start: Some(dummy),
            thread_end: Some(dummy),
            ..Default::default()
        })
        .expect("Failt to set callbacks");

    for event in [Event::VMInit, Event::ThreadStart, Event::ThreadEnd] {
        jvmti
            .set_event_notification_mode(true, event, None)
            .expect("Fail to enable event");
    }
}

fn dummy(jvmti: &jvmti::Env, jni: jni::Env, thread: JThread) {
    println!("2333333")
}
