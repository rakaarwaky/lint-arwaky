// PURPOSE: taxonomy_resolved_import_vo — Value object for barrel import resolution results.
// Moved from utility_import_resolver.rs to satisfy AES404 (utility files must not define structs/enums).

/// Result of resolving an import through a barrel file.
/// Contains the resolved file path and the layer detected from that path.
#[derive(Debug, Clone)]
pub struct ResolvedImport {
    /// Original module path as written in the import statement
    pub original_module: String,
    /// Resolved file path (e.g., "capabilities_payment_service.py")
    pub resolved_file: String,
    /// Detected layer from the resolved file name
    pub resolved_layer: Option<String>,
    /// The symbol that was imported
    pub symbol: String,
}
