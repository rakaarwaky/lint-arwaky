// This agent orchestrator violates AES024 (agent-any-bypass)
// because it bypasses static type-safety by using raw c_void pointer type inside the agent layer.
use crate::taxonomy::removal_types::RemovalType;
use crate::contract::removal_io::IRemovalIO;
use std::ffi::c_void;

pub struct RawPtrOrchestrator {
    // raw pointer or Any equivalent violates type-safety mandate AES024!
    pub context_ptr: *mut c_void,
}

impl RawPtrOrchestrator {
    pub fn new() -> Self {
        Self { context_ptr: std::ptr::null_mut() }
    }
}
