// PURPOSE: ParserAdapter — routes file extensions to the correct language-specific scanner

use shared::code_analysis::taxonomy_import_source_vo::ImportInfoList;
use shared::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList;
use shared::language_adapters::taxonomy_naming_list_vo::PrimitiveTypeList;
use shared::pipeline_jobs::taxonomy_job_vo::ResponseData;
use shared::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use shared::taxonomy_common_vo::BooleanVO;
use shared::taxonomy_common_vo::Count;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_name_vo::SymbolName;
use shared::taxonomy_suggestion_vo::MetadataVO;
use crate::contract_parser_port::ISourceParserPort;
use crate::taxonomy_parser_error::SourceParserError;
use crate::taxonomy_path_vo::FilePath;

/// Composite source parser that delegates to language-specific adapters via DI.
///
/// Routing logic:
/// - `.rs` → Rust parser
/// - `.ts`, `.tsx`, `.js`, `.jsx` → JS/TS parser
/// - `.py` (and everything else) → Python parser
///
/// Parser instances are injected as trait objects to keep the orchestrator decoupled
/// from concrete infrastructure adapter types.
pub struct SourceParserOrchestrator {
    python_parser: Box<dyn ISourceParserPort>,
    rust_parser: Box<dyn ISourceParserPort>,
    js_parser: Box<dyn ISourceParserPort>,
}

impl SourceParserOrchestrator {
    pub fn new(
        python_parser: Box<dyn ISourceParserPort>,
        rust_parser: Box<dyn ISourceParserPort>,
        js_parser: Box<dyn ISourceParserPort>,
    ) -> Self {
        Self {
            python_parser,
            rust_parser,
            js_parser,
        }
    }

    fn select_parser(&self, path: &FilePath) -> &dyn ISourceParserPort {
        let p = &path.value;
        if p.ends_with(".rs") {
            return &*self.rust_parser;
        }
        if p.ends_with(".ts") || p.ends_with(".tsx") || p.ends_with(".js") || p.ends_with(".jsx") {
            return &*self.js_parser;
        }
        &*self.python_parser
    }
}

impl Default for SourceParserOrchestrator {
    fn default() -> Self {
        // Intentionally create empty parser — caller must use SourceParserOrchestrator::new()
        todo!("SourceParserOrchestrator requires DI — use SourceParserOrchestrator::new() with parser instances")
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

    fn is_barrel_file(&self, path: &FilePath) -> BooleanVO {
        self.select_parser(path).is_barrel_file(path)
    }

    fn get_stem(&self, path: &FilePath) -> SymbolName {
        self.select_parser(path).get_stem(path)
    }

    fn is_entry_point(&self, path: &FilePath) -> BooleanVO {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure_rust_scanner::ASTRustParserAdapter;
    use crate::infrastructure_py_scanner::ASTPythonParserAdapter;
    use crate::infrastructure_js_scanner::ASTJSParserAdapter;

    #[test]
    fn test_orchestrator_routing() {
        let orchestrator = SourceParserOrchestrator::new(
            Box::new(ASTPythonParserAdapter::new()),
            Box::new(ASTRustParserAdapter::new()),
            Box::new(ASTJSParserAdapter::new()),
        );

        // Test rust routing
        let rust_path = FilePath::new("test.rs").unwrap();
        let ext_rust = orchestrator.select_parser(&rust_path).get_supported_extensions();
        assert!(ext_rust.values.contains(&".rs".to_string()));

        // Test js/ts routing
        let ts_path = FilePath::new("test.ts").unwrap();
        let ext_js = orchestrator.select_parser(&ts_path).get_supported_extensions();
        assert!(ext_js.values.contains(&".ts".to_string()));

        // Test python routing
        let py_path = FilePath::new("test.py").unwrap();
        let ext_py = orchestrator.select_parser(&py_path).get_supported_extensions();
        assert!(ext_py.values.contains(&".py".to_string()));
    }
}
