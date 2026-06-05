/// python_symbol_collector — Collector for symbols and imports from Python AST.
use crate::taxonomy::{Count, InheritanceMap, MetadataVO, SymbolName, SymbolNameList, ImportInfo, LineNumber, ModuleName};

pub struct SymbolCollector {
    defined: Vec<String>,
    used: Vec<String>,
    exported: Vec<String>,
    imported_aliases: std::collections::HashMap<String, String>,
    class_bases: std::collections::HashMap<String, Vec<String>>,
    imports_list: Vec<ImportInfo>,
}

impl SymbolCollector {
    pub fn new() -> Self {
        Self { defined: Vec::new(), used: Vec::new(), exported: Vec::new(), imported_aliases: std::collections::HashMap::new(), class_bases: std::collections::HashMap::new(), imports_list: Vec::new() }
    }

    pub fn defined(&self) -> SymbolNameList { SymbolNameList::new(self.defined.iter().map(|s| SymbolName::new(s.clone())).collect()) }
    pub fn used(&self) -> SymbolNameList { SymbolNameList::new(self.used.iter().map(|s| SymbolName::new(s.clone())).collect()) }
    pub fn exported(&self) -> SymbolNameList { SymbolNameList::new(self.exported.iter().map(|s| SymbolName::new(s.clone())).collect()) }
    pub fn imported_aliases(&self) -> MetadataVO { MetadataVO::new(serde_json::to_value(&self.imported_aliases).unwrap_or_default()) }
}
