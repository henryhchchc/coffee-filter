use std::ffi::OsStr;

use coffee_filter::{
    agent_on_load,
    jvm::general::JvmTIVersion,
    jvm::{events::JvmTIEvent, Jvm},
};

agent_on_load!(agent_onload, JvmTIVersion::LATEST);

fn agent_onload(jvm: &mut Jvm, opts: Option<&OsStr>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello from coffee-filter");
    println!("options: {:?}", opts);
    let version = jvm.get_version()?;
    println!(
        "JVM TI version: {}.{}.{}",
        version.major(),
        version.minor(),
        version.micro()
    );
    jvm.update_callbacks(|it| {
        it.thread_start = Some(Box::new(|jvm, jni, thread| {
            println!("thread.info(): {:?}", thread.info());
            println!("loaded classes: {:?}", jvm.get_loaded_classes());
        }));
    })?;
    jvm.enable_event(JvmTIEvent::ThreadStart, None)?;
    Ok(())
}
