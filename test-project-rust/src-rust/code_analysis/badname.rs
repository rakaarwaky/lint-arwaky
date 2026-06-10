// PURPOSE: Test AES010 — badly named file (missing prefix_concept_suffix pattern)
pub struct BadName {
    value: i32,
}

impl BadName {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
