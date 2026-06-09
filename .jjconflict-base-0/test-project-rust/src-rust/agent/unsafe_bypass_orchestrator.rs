pub struct UnsafeBypassOrchestrator;
impl UnsafeBypassOrchestrator {
    pub fn unsafe_run(&self) {
        let val = Some(42);
        let _ = val.unwrap();
        let raw: *const i32 = &42;
        let _ = raw;
    }
}
