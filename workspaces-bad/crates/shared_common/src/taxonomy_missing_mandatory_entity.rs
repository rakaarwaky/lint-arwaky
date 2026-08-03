pub struct MissingMandatoryEntity {
    pub id: u64,
    pub label: String,
}

impl MissingMandatoryEntity {
    pub fn new(id: u64, label: &str) -> Self {
        Self { id, label: label.to_string() }
    }
}
