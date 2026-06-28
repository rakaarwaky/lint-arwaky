pub struct ExtraEntity {
    pub id: u64,
    pub label: String,
}

impl ExtraEntity {
    pub fn new(id: u64, label: &str) -> Self {
        Self { id, label: label.to_string() }
    }
    pub fn compute(&self) -> u64 {
        self.id * 2
    }
    pub fn is_valid(&self) -> bool {
        self.id > 0
    }
}
