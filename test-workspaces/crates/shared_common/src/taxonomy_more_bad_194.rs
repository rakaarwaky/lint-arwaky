pub struct MoreBadEntity {
    pub id: u64,
    pub name: String,
    pub active: bool,
}

impl MoreBadEntity {
    pub fn new(id: u64, name: &str) -> Self {
        Self { id, name: name.to_string(), active: true }
    }
    pub fn process(&self) -> String {
        format!("{}:{}", self.name, self.id)
    }
    pub fn validate(&self) -> bool {
        self.id > 0 && !self.name.is_empty()
    }
    pub fn activate(&mut self) {
        self.active = true;
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}
