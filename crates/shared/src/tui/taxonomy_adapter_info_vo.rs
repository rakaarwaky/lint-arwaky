// PURPOSE: taxonomy_adapter_info_vo — value object for discovered lint adapter metadata
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterInfo {
    pub name: String,
    pub label: String,
    pub installed: bool,
}

impl fmt::Display for AdapterInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.name,
            if self.installed {
                "installed"
            } else {
                "missing"
            }
        )
    }
}
