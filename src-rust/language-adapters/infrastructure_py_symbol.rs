/// python_symbol_scanner — Scanner for symbols and imports from Python AST.
use /* UNKNOWN: ImportInfo */ crate::code_analysis::taxonomy_source_vo::ImportInfo;
use /* UNKNOWN: MetadataVO */ crate::shared_common::taxonomy_suggestion_vo::MetadataVO;
use /* UNKNOWN: SymbolName */ crate::naming_rules::taxonomy_symbol_vo::SymbolName;
use /* UNKNOWN: SymbolNameList */ crate::naming_rules::taxonomy_symbols_vo::SymbolNameList;

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
        Self {
            defined: Vec::new(),
            used: Vec::new(),
            exported: Vec::new(),
            imported_aliases: std::collections::HashMap::new(),
            class_bases: std::collections::HashMap::new(),
            imports_list: Vec::new(),
        }
    }

    pub fn defined(&self) -> SymbolNameList {
        SymbolNameList {
            values: self
                .defined
                .iter()
                .map(|s| SymbolName::new(s.clone()))
                .collect(),
        }
    }
    pub fn used(&self) -> SymbolNameList {
        SymbolNameList {
            values: self
                .used
                .iter()
                .map(|s| SymbolName::new(s.clone()))
                .collect(),
        }
    }
    pub fn exported(&self) -> SymbolNameList {
        SymbolNameList {
            values: self
                .exported
                .iter()
                .map(|s| SymbolName::new(s.clone()))
                .collect(),
        }
    }
    pub fn imported_aliases(&self) -> MetadataVO {
        let map: std::collections::HashMap<String, serde_json::Value> = self
            .imported_aliases
            .iter()
            .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
            .collect();
        MetadataVO::new(map)
    }

    pub fn class_bases(&self) -> &std::collections::HashMap<String, Vec<String>> {
        &self.class_bases
    }

    pub fn imports_list(&self) -> &[ImportInfo] {
        &self.imports_list
    }
}
