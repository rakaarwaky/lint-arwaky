pub struct BadNameEntity {
    pub name: String,
    pub value: i32,
    pub active: bool,
}

impl BadNameEntity {
    pub fn new(name: &str, value: i32) -> Self {
        Self { name: name.to_string(), value, active: true }
    }
    pub fn process(&self) -> String {
        format!("{}:{}", self.name, self.value)
    }
    pub fn validate(&self) -> bool {
        self.value > 0 && !self.name.is_empty()
    }
}
