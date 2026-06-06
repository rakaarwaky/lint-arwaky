//! source_parser_adapter — Orchestrator that delegates to language-specific parsers.
//!
//! Instead of duplicating 2000+ lines of scanner code, this module composes
//! the three language-specific adapters and routes calls based on file extension.

use crate::contract::source_parser_port::ISourceParserPort;
use crate::infrastructure::ast_js_scanner::ASTJSParserAdapter;
use crate::infrastructure::ast_py_scanner::ASTPythonParserAdapter;
use crate::infrastructure::ast_rust_scanner::ASTRustParserAdapter;
use crate::taxonomy::{
    Count, FilePath, ImportInfoList, MetadataVO, PatternList, PrimitiveTypeList,
    PrimitiveViolationList, ResponseData, SourceParserError, SuccessStatus, SymbolName,
};

/// Composite source parser that delegates to language-specific adapters.
///
/// Routing logic:
/// - `.rs` → `ASTRustParserAdapter`
/// - `.ts`, `.tsx`, `.js`, `.jsx` → `ASTJSParserAdapter`
/// - `.py` (and everything else) → `ASTPythonParserAdapter`
pub struct SourceParserOrchestrator {
    python_parser: ASTPythonParserAdapter,
    rust_parser: ASTRustParserAdapter,
    js_parser: ASTJSParserAdapter,
}

impl SourceParserOrchestrator {
    pub fn new() -> Self {
        Self {
            python_parser: ASTPythonParserAdapter::new(),
            rust_parser: ASTRustParserAdapter::new(),
            js_parser: ASTJSParserAdapter::new(),
        }
    }

    fn select_parser(&self, path: &FilePath) -> &dyn ISourceParserPort {
        let p = &path.value;
        if p.ends_with(".rs") {
            return &self.rust_parser;
        }
        if p.ends_with(".ts")
            || p.ends_with(".tsx")
            || p.ends_with(".js")
            || p.ends_with(".jsx")
        {
            return &self.js_parser;
        }
        &self.python_parser
    }
}

impl Default for SourceParserOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl ISourceParserPort for SourceParserOrchestrator {
    fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
        self.select_parser(path).extract_imports(path)
    }

    fn get_raw_symbols(&self, path: &FilePath) -> Result<ResponseData, SourceParserError> {
        self.select_parser(path).get_raw_symbols(path)
    }

    fn get_class_attributes(&self, path: &FilePath) -> ResponseData {
        self.select_parser(path).get_class_attributes(path)
    }

    fn has_all_export(&self, path: &FilePath) -> SuccessStatus {
        self.select_parser(path).has_all_export(path)
    }

    fn find_primitive_violations(
        &self,
        path: &FilePath,
        primitive_types: &PrimitiveTypeList,
    ) -> PrimitiveViolationList {
        self.select_parser(path)
            .find_primitive_violations(path, primitive_types)
    }

    fn find_unused_imports(&self, path: &FilePath) -> ImportInfoList {
        self.select_parser(path).find_unused_imports(path)
    }

    fn get_class_definitions(&self, path: &FilePath) -> Result<MetadataVO, SourceParserError> {
        self.select_parser(path).get_class_definitions(path)
    }

    fn get_function_definitions(&self, path: &FilePath) -> MetadataVO {
        self.select_parser(path).get_function_definitions(path)
    }

    fn is_symbol_exported(&self, path: &FilePath, symbol: &SymbolName) -> SuccessStatus {
        self.select_parser(path).is_symbol_exported(path, symbol)
    }

    fn get_class_methods(&self, path: &FilePath) -> MetadataVO {
        self.select_parser(path).get_class_methods(path)
    }

    fn get_class_bases_map(&self, path: &FilePath) -> MetadataVO {
        self.select_parser(path).get_class_bases_map(path)
    }

    fn get_assignment_targets(&self, path: &FilePath) -> MetadataVO {
        self.select_parser(path).get_assignment_targets(path)
    }

    fn get_control_flow_count(&self, path: &FilePath) -> Count {
        self.select_parser(path).get_control_flow_count(path)
    }

    fn is_barrel_file(&self, path: &FilePath) -> bool {
        self.select_parser(path).is_barrel_file(path)
    }

    fn get_stem(&self, path: &FilePath) -> SymbolName {
        self.select_parser(path).get_stem(path)
    }

    fn is_entry_point(&self, path: &FilePath) -> bool {
        self.select_parser(path).is_entry_point(path)
    }

    fn get_supported_extensions(&self) -> PatternList {
        PatternList::new(vec![
            ".py".to_string(),
            ".rs".to_string(),
            ".ts".to_string(),
            ".tsx".to_string(),
            ".js".to_string(),
            ".jsx".to_string(),
        ])
    }
}
