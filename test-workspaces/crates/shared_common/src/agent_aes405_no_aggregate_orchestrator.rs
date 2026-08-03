// AES405: no struct implements an _aggregate trait
pub struct OrchestratorEntity;

impl OrchestratorEntity {
    pub fn orchestrate(&self) {
        println!("orchestrating");
    }
}
