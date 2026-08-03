pub struct StatefulOrchestrator {
    pub counter: u32,
}
impl StatefulOrchestrator {
    pub fn run(&mut self) { self.counter += 1; }
}
