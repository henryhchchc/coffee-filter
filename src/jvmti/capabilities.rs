use std::{collections::HashSet, mem::MaybeUninit};

use crate::{macros::unsafe_jvmti, sys, utils::jvmti_result};

use super::{Env, Error};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Capabilities {
    /// Capability to tag objects.
    TagObjects,
    /// Capability to generate field modification events.
    GenerateFieldModificationEvents,
    /// Capability to generate field access events.
    GenerateFieldAccessEvents,
    /// Capability to get bytecodes of methods.
    GetBytecodes,
    /// Capability to get synthetic attribute of classes, fields, and methods.
    GetSyntheticAttribute,
    /// Capability to get information about owned monitors.
    GetOwnedMonitorInfo,
    /// Capability to get the current contended monitor.
    GetCurrentContendedMonitor,
    /// Capability to get monitor information.
    GetMonitorInfo,
    /// Capability to pop a frame.
    PopFrame,
    /// Capability to redefine classes.
    RedefineClasses,
    /// Capability to signal a thread.
    SignalThread,
    /// Capability to get the source file name of a class.
    GetSourceFileName,
    /// Capability to get line numbers of a method.
    GetLineNumbers,
    /// Capability to get source debug extension.
    GetSourceDebugExtension,
    /// Capability to access local variables.
    AccessLocalVariables,
    /// Capability to maintain original method order.
    MaintainOriginalMethodOrder,
    /// Capability to generate single step events.
    GenerateSingleStepEvents,
    /// Capability to generate exception events.
    GenerateExceptionEvents,
    /// Capability to generate frame pop events.
    GenerateFramePopEvents,
    /// Capability to generate breakpoint events.
    GenerateBreakpointEvents,
    /// Capability to suspend threads.
    Suspend,
    /// Capability to redefine any class.
    RedefineAnyClass,
    /// Capability to get current thread CPU time.
    GetCurrentThreadCpuTime,
    /// Capability to get thread CPU time.
    GetThreadCpuTime,
    /// Capability to generate method entry events.
    GenerateMethodEntryEvents,
    /// Capability to generate method exit events.
    GenerateMethodExitEvents,
    /// Capability to generate all class hook events.
    GenerateAllClassHookEvents,
    /// Capability to generate compiled method load events.
    GenerateCompiledMethodLoadEvents,
    /// Capability to generate monitor events.
    GenerateMonitorEvents,
    /// Capability to generate VM object allocation events.
    GenerateVmObjectAllocEvents,
    /// Capability to generate native method bind events.
    GenerateNativeMethodBindEvents,
    /// Capability to generate garbage collection events.
    GenerateGarbageCollectionEvents,
    /// Capability to generate object free events.
    GenerateObjectFreeEvents,
    /// Capability to force early return from a method.
    ForceEarlyReturn,
    /// Capability to get owned monitor stack depth information.
    GetOwnedMonitorStackDepthInfo,
    /// Capability to get constant pool of a class.
    GetConstantPool,
    /// Capability to set native method prefix.
    SetNativeMethodPrefix,
    /// Capability to retransform classes.
    RetransformClasses,
    /// Capability to retransform any class.
    RetransformAnyClass,
    /// Capability to generate resource exhaustion heap events.
    GenerateResourceExhaustionHeapEvents,
    /// Capability to generate resource exhaustion threads events.
    GenerateResourceExhaustionThreadsEvents,
    /// Capability to generate early VM start events.
    GenerateEarlyVmstart,
    /// Capability to generate early class hook events.
    GenerateEarlyClassHookEvents,
    /// Capability to generate sampled object allocation events.
    GenerateSampledObjectAllocEvents,
    /// Capability to support virtual threads.
    SupportVirtualThreads,
}

impl Env {
    /// Retrieves the potential capabilities that the current environment can support.
    pub fn get_potential_capabilities(&self) -> Result<HashSet<Capabilities>, Error> {
        let mut sys_cap: MaybeUninit<sys::jvmtiCapabilities> = MaybeUninit::uninit();
        let errno = unsafe_jvmti!(self.ptr, GetPotentialCapabilities, sys_cap.as_mut_ptr());
        jvmti_result(errno, || {
            let sys_cap = unsafe { sys_cap.assume_init() };
            sys_cap.into()
        })
    }

    /// Retrieves the currently enabled capabilities of the current environment.
    pub fn get_capabilities(&self) -> Result<HashSet<Capabilities>, Error> {
        let mut sys_cap: MaybeUninit<sys::jvmtiCapabilities> = MaybeUninit::uninit();
        let errno = unsafe_jvmti!(self.ptr, GetCapabilities, sys_cap.as_mut_ptr());
        jvmti_result(errno, || {
            let sys_cap = unsafe { sys_cap.assume_init() };
            sys_cap.into()
        })
    }

    /// Adds the specified capabilities to the current environment.
    pub fn add_capabilities(&self, caps: HashSet<Capabilities>) -> Result<(), Error> {
        let sys_cap: sys::jvmtiCapabilities = caps.into();
        let errno = unsafe_jvmti!(self.ptr, AddCapabilities, &sys_cap);
        jvmti_result(errno, || ())
    }

    /// Relinquishes the specified capabilities from the current environment.
    pub fn relinquish_capabilities(&self, caps: HashSet<Capabilities>) -> Result<(), Error> {
        let sys_cap: sys::jvmtiCapabilities = caps.into();
        let errno = unsafe_jvmti!(self.ptr, RelinquishCapabilities, &sys_cap);
        jvmti_result(errno, || ())
    }
}

impl From<HashSet<Capabilities>> for sys::jvmtiCapabilities {
    fn from(caps: HashSet<Capabilities>) -> Self {
        let mut sys_cap = sys::jvmtiCapabilities {
            _bitfield_align_1: Default::default(),
            _bitfield_1: Default::default(),
        };
        for cap in caps {
            match cap {
                Capabilities::TagObjects => sys_cap.set_can_tag_objects(1),
                Capabilities::GenerateFieldModificationEvents => {
                    sys_cap.set_can_generate_field_modification_events(1)
                }
                Capabilities::GenerateFieldAccessEvents => {
                    sys_cap.set_can_generate_field_access_events(1)
                }
                Capabilities::GetBytecodes => sys_cap.set_can_get_bytecodes(1),
                Capabilities::GetSyntheticAttribute => sys_cap.set_can_get_synthetic_attribute(1),
                Capabilities::GetOwnedMonitorInfo => sys_cap.set_can_get_owned_monitor_info(1),
                Capabilities::GetCurrentContendedMonitor => {
                    sys_cap.set_can_get_current_contended_monitor(1)
                }
                Capabilities::GetMonitorInfo => sys_cap.set_can_get_monitor_info(1),
                Capabilities::PopFrame => sys_cap.set_can_pop_frame(1),
                Capabilities::RedefineClasses => sys_cap.set_can_redefine_classes(1),
                Capabilities::SignalThread => sys_cap.set_can_signal_thread(1),
                Capabilities::GetSourceFileName => sys_cap.set_can_get_source_file_name(1),
                Capabilities::GetLineNumbers => sys_cap.set_can_get_line_numbers(1),
                Capabilities::GetSourceDebugExtension => {
                    sys_cap.set_can_get_source_debug_extension(1)
                }
                Capabilities::AccessLocalVariables => sys_cap.set_can_access_local_variables(1),
                Capabilities::MaintainOriginalMethodOrder => {
                    sys_cap.set_can_maintain_original_method_order(1)
                }
                Capabilities::GenerateSingleStepEvents => {
                    sys_cap.set_can_generate_single_step_events(1)
                }
                Capabilities::GenerateExceptionEvents => {
                    sys_cap.set_can_generate_exception_events(1)
                }
                Capabilities::GenerateFramePopEvents => {
                    sys_cap.set_can_generate_frame_pop_events(1)
                }
                Capabilities::GenerateBreakpointEvents => {
                    sys_cap.set_can_generate_breakpoint_events(1)
                }
                Capabilities::Suspend => sys_cap.set_can_suspend(1),
                Capabilities::RedefineAnyClass => sys_cap.set_can_redefine_any_class(1),
                Capabilities::GetCurrentThreadCpuTime => {
                    sys_cap.set_can_get_current_thread_cpu_time(1)
                }
                Capabilities::GetThreadCpuTime => sys_cap.set_can_get_thread_cpu_time(1),
                Capabilities::GenerateMethodEntryEvents => {
                    sys_cap.set_can_generate_method_entry_events(1)
                }
                Capabilities::GenerateMethodExitEvents => {
                    sys_cap.set_can_generate_method_exit_events(1)
                }
                Capabilities::GenerateAllClassHookEvents => {
                    sys_cap.set_can_generate_all_class_hook_events(1)
                }
                Capabilities::GenerateCompiledMethodLoadEvents => {
                    sys_cap.set_can_generate_compiled_method_load_events(1)
                }
                Capabilities::GenerateMonitorEvents => sys_cap.set_can_generate_monitor_events(1),
                Capabilities::GenerateVmObjectAllocEvents => {
                    sys_cap.set_can_generate_vm_object_alloc_events(1)
                }
                Capabilities::GenerateNativeMethodBindEvents => {
                    sys_cap.set_can_generate_native_method_bind_events(1)
                }
                Capabilities::GenerateGarbageCollectionEvents => {
                    sys_cap.set_can_generate_garbage_collection_events(1)
                }
                Capabilities::GenerateObjectFreeEvents => {
                    sys_cap.set_can_generate_object_free_events(1)
                }
                Capabilities::ForceEarlyReturn => sys_cap.set_can_force_early_return(1),
                Capabilities::GetOwnedMonitorStackDepthInfo => {
                    sys_cap.set_can_get_owned_monitor_stack_depth_info(1)
                }
                Capabilities::GetConstantPool => sys_cap.set_can_get_constant_pool(1),
                Capabilities::SetNativeMethodPrefix => sys_cap.set_can_set_native_method_prefix(1),
                Capabilities::RetransformClasses => sys_cap.set_can_retransform_classes(1),
                Capabilities::RetransformAnyClass => sys_cap.set_can_retransform_any_class(1),
                Capabilities::GenerateResourceExhaustionHeapEvents => {
                    sys_cap.set_can_generate_resource_exhaustion_heap_events(1)
                }
                Capabilities::GenerateResourceExhaustionThreadsEvents => {
                    sys_cap.set_can_generate_resource_exhaustion_threads_events(1)
                }
                Capabilities::GenerateEarlyVmstart => sys_cap.set_can_generate_early_vmstart(1),
                Capabilities::GenerateEarlyClassHookEvents => {
                    sys_cap.set_can_generate_early_class_hook_events(1)
                }
                Capabilities::GenerateSampledObjectAllocEvents => {
                    sys_cap.set_can_generate_sampled_object_alloc_events(1)
                }
                Capabilities::SupportVirtualThreads => sys_cap.set_can_support_virtual_threads(1),
            }
        }

        sys_cap
    }
}

impl From<sys::jvmtiCapabilities> for HashSet<Capabilities> {
    fn from(val: sys::jvmtiCapabilities) -> Self {
        let mut capabilities = HashSet::new();
        if val.can_tag_objects() == 1 {
            capabilities.insert(Capabilities::TagObjects);
        }
        if val.can_generate_field_modification_events() == 1 {
            capabilities.insert(Capabilities::GenerateFieldModificationEvents);
        }
        if val.can_generate_field_access_events() == 1 {
            capabilities.insert(Capabilities::GenerateFieldAccessEvents);
        }
        if val.can_get_bytecodes() == 1 {
            capabilities.insert(Capabilities::GetBytecodes);
        }
        if val.can_get_synthetic_attribute() == 1 {
            capabilities.insert(Capabilities::GetSyntheticAttribute);
        }
        if val.can_get_owned_monitor_info() == 1 {
            capabilities.insert(Capabilities::GetOwnedMonitorInfo);
        }
        if val.can_get_current_contended_monitor() == 1 {
            capabilities.insert(Capabilities::GetCurrentContendedMonitor);
        }
        if val.can_get_monitor_info() == 1 {
            capabilities.insert(Capabilities::GetMonitorInfo);
        }
        if val.can_pop_frame() == 1 {
            capabilities.insert(Capabilities::PopFrame);
        }
        if val.can_redefine_classes() == 1 {
            capabilities.insert(Capabilities::RedefineClasses);
        }
        if val.can_signal_thread() == 1 {
            capabilities.insert(Capabilities::SignalThread);
        }
        if val.can_get_source_file_name() == 1 {
            capabilities.insert(Capabilities::GetSourceFileName);
        }
        if val.can_get_line_numbers() == 1 {
            capabilities.insert(Capabilities::GetLineNumbers);
        }
        if val.can_get_source_debug_extension() == 1 {
            capabilities.insert(Capabilities::GetSourceDebugExtension);
        }
        if val.can_access_local_variables() == 1 {
            capabilities.insert(Capabilities::AccessLocalVariables);
        }
        if val.can_maintain_original_method_order() == 1 {
            capabilities.insert(Capabilities::MaintainOriginalMethodOrder);
        }
        if val.can_generate_single_step_events() == 1 {
            capabilities.insert(Capabilities::GenerateSingleStepEvents);
        }
        if val.can_generate_exception_events() == 1 {
            capabilities.insert(Capabilities::GenerateExceptionEvents);
        }
        if val.can_generate_frame_pop_events() == 1 {
            capabilities.insert(Capabilities::GenerateFramePopEvents);
        }
        if val.can_generate_breakpoint_events() == 1 {
            capabilities.insert(Capabilities::GenerateBreakpointEvents);
        }
        if val.can_suspend() == 1 {
            capabilities.insert(Capabilities::Suspend);
        }
        if val.can_redefine_any_class() == 1 {
            capabilities.insert(Capabilities::RedefineAnyClass);
        }
        if val.can_get_current_thread_cpu_time() == 1 {
            capabilities.insert(Capabilities::GetCurrentThreadCpuTime);
        }
        if val.can_get_thread_cpu_time() == 1 {
            capabilities.insert(Capabilities::GetThreadCpuTime);
        }
        if val.can_generate_method_entry_events() == 1 {
            capabilities.insert(Capabilities::GenerateMethodEntryEvents);
        }
        if val.can_generate_method_exit_events() == 1 {
            capabilities.insert(Capabilities::GenerateMethodExitEvents);
        }
        if val.can_generate_all_class_hook_events() == 1 {
            capabilities.insert(Capabilities::GenerateAllClassHookEvents);
        }
        if val.can_generate_compiled_method_load_events() == 1 {
            capabilities.insert(Capabilities::GenerateCompiledMethodLoadEvents);
        }
        if val.can_generate_monitor_events() == 1 {
            capabilities.insert(Capabilities::GenerateMonitorEvents);
        }
        if val.can_generate_vm_object_alloc_events() == 1 {
            capabilities.insert(Capabilities::GenerateVmObjectAllocEvents);
        }
        if val.can_generate_native_method_bind_events() == 1 {
            capabilities.insert(Capabilities::GenerateNativeMethodBindEvents);
        }
        if val.can_generate_garbage_collection_events() == 1 {
            capabilities.insert(Capabilities::GenerateGarbageCollectionEvents);
        }
        if val.can_generate_object_free_events() == 1 {
            capabilities.insert(Capabilities::GenerateObjectFreeEvents);
        }
        if val.can_force_early_return() == 1 {
            capabilities.insert(Capabilities::ForceEarlyReturn);
        }
        if val.can_get_owned_monitor_stack_depth_info() == 1 {
            capabilities.insert(Capabilities::GetOwnedMonitorStackDepthInfo);
        }
        if val.can_get_constant_pool() == 1 {
            capabilities.insert(Capabilities::GetConstantPool);
        }
        if val.can_set_native_method_prefix() == 1 {
            capabilities.insert(Capabilities::SetNativeMethodPrefix);
        }
        if val.can_retransform_classes() == 1 {
            capabilities.insert(Capabilities::RetransformClasses);
        }
        if val.can_retransform_any_class() == 1 {
            capabilities.insert(Capabilities::RetransformAnyClass);
        }
        if val.can_generate_resource_exhaustion_heap_events() == 1 {
            capabilities.insert(Capabilities::GenerateResourceExhaustionHeapEvents);
        }
        if val.can_generate_resource_exhaustion_threads_events() == 1 {
            capabilities.insert(Capabilities::GenerateResourceExhaustionThreadsEvents);
        }
        if val.can_generate_early_vmstart() == 1 {
            capabilities.insert(Capabilities::GenerateEarlyVmstart);
        }
        if val.can_generate_early_class_hook_events() == 1 {
            capabilities.insert(Capabilities::GenerateEarlyClassHookEvents);
        }
        if val.can_generate_sampled_object_alloc_events() == 1 {
            capabilities.insert(Capabilities::GenerateSampledObjectAllocEvents);
        }
        if val.can_support_virtual_threads() == 1 {
            capabilities.insert(Capabilities::SupportVirtualThreads);
        }
        capabilities
    }
}
