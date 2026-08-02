// PURPOSE: ResolvedImport — Value Object for barrel-resolved import information.
//
// When an import goes through a barrel file (__init__.py, index.ts, mod.rs),
// the original module path hides the source file name and its layer prefix.
// This VO carries the resolution result so checkers can detect the correct layer.
//
// # Example
// ```text
// import:   from modules.shared.src.server import IBlenderConnectionProtocol
// barrel:   modules/shared/src/server/__init__.py
//           → from .contract_connection_protocol import IBlenderConnectionProtocol
// resolved: ResolvedImport {
//     original_module: "modules.shared.src.server",
//     resolved_file:   "contract_connection_protocol",
//     resolved_layer:  Some("contract"),
//     symbol:          "IBlenderConnectionProtocol",
// }
// ```

/// Result of resolving an import through a barrel file.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedImport {
    /// Original module path as written in the import statement.
    /// e.g. `"modules.shared.src.server"`
    pub original_module: String,

    /// Resolved source file stem (filename without extension).
    /// e.g. `"contract_connection_protocol"`
    pub resolved_file: String,

    /// Detected architectural layer from the resolved file name.
    /// e.g. `Some("contract")` from `"contract_connection_protocol"`
    pub resolved_layer: Option<String>,

    /// The symbol that was imported.
    /// e.g. `"IBlenderConnectionProtocol"`
    pub symbol: String,
}

impl ResolvedImport {
    pub fn new(
        original_module: impl Into<String>,
        resolved_file: impl Into<String>,
        resolved_layer: Option<String>,
        symbol: impl Into<String>,
    ) -> Self {
        Self {
            original_module: original_module.into(),
            resolved_file: resolved_file.into(),
            resolved_layer,
            symbol: symbol.into(),
        }
    }

    /// Check if the resolved layer matches the expected layer.
    pub fn matches_layer(&self, expected: &str) -> bool {
        self.resolved_layer.as_deref() == Some(expected)
    }

    /// Check if the resolved file name contains the given suffix.
    /// e.g. `has_suffix("protocol")` → true for `"contract_connection_protocol"`
    pub fn has_suffix(&self, suffix: &str) -> bool {
        self.resolved_file
            .to_lowercase()
            .contains(&format!("_{}", suffix.to_lowercase()))
    }
}
