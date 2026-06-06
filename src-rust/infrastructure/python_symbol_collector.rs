/// python_symbol_collector — Collector for symbols and imports from Python AST.
use crate::taxonomy::{MetadataVO, SymbolName, SymbolNameList, ImportInfo};

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

    pub fn defined(&self) -> SymbolNameList { SymbolNameList { values: self.defined.iter().map(|s| SymbolName::new(s.clone())).collect() } }
    pub fn used(&self) -> SymbolNameList { SymbolNameList { values: self.used.iter().map(|s| SymbolName::new(s.clone())).collect() } }
    pub fn exported(&self) -> SymbolNameList { SymbolNameList { values: self.exported.iter().map(|s| SymbolName::new(s.clone())).collect() } }
    pub fn imported_aliases(&self) -> MetadataVO {
        let mut map = std::collections::HashMap::new();
        for (k, v) in &self.imported_aliases {
            map.insert(k.clone(), serde_json::Value::String(v.clone()));
        }
        MetadataVO::new(map)
    }
}
