// PURPOSE: ConfigIdentifier — value object for named configuration identifiers
use crate::string_value_object;

string_value_object!(ConfigKey);

impl ConfigKey {
    /// Returns each dot-separated segment of the key.
    pub fn parts(&self) -> Vec<String> {
        self.value.split('.').map(|s| s.to_string()).collect()
    }

    /// Returns the parent key, dropping the last segment. Empty when the
    /// key has no parent (single segment).
    pub fn parent(&self) -> String {
        let parts = self.parts();
        if parts.len() > 1 {
            parts[..parts.len() - 1].join(".")
        } else {
            String::new()
        }
    }

    /// Returns the last segment of the key, or the full value when the
    /// key has no `.` separators.
    pub fn leaf(&self) -> String {
        match self.parts().last() {
            Some(part) => part.clone(),
            None => self.value.clone(),
        }
    }
}