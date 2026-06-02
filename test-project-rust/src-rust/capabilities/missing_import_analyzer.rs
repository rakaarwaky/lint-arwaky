// This capability file violates AES002 (mandatory-import-missing)
// because capabilities must import from contract(protocol) and taxonomy,
// but this file has zero imports.
pub struct MissingImportAnalyzer {
    pub name: String,
}
