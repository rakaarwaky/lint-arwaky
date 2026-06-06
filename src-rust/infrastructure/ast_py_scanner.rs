/// ast_py_scanner — Orchestrator for Python AST analysis.
use crate::contract::source_parser_port::ISourceParserPort;
use crate::taxonomy::{
    Count, ErrorMessage, FilePath, ImportInfoList, MetadataVO, PrimitiveTypeList, PrimitiveViolationList,
    ResponseData, SourceParserError, SuccessStatus, SymbolName,
};

pub struct ASTPythonParserAdapter;

impl ASTPythonParserAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl ISourceParserPort for ASTPythonParserAdapter {
    fn extract_imports(&self, path: &FilePath) -> Result<ImportInfoList, SourceParserError> {
        Err(SourceParserError {
            path: path.clone(),
            message: ErrorMessage::new("Not yet implemented"),
            ..Default::default()
        })
    }

    fn get_raw_symbols(&self, path: &FilePath) -> Result<ResponseData, SourceParserError> {
        Err(SourceParserError {
            path: path.clone(),
            message: ErrorMessage::new("Not yet implemented"),
            ..Default::default()
        })
    }

    fn get_class_attributes(&self, _path: &FilePath) -> ResponseData {
        ResponseData { value: Some(serde_json::Value::Null), stdout: String::new(), stderr: String::new(), returncode: 0, metadata: std::collections::HashMap::new() }
    }

    fn has_all_export(&self, _path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(false)
    }

    fn find_primitive_violations(
        &self,
        _path: &FilePath,
        _primitive_types: &PrimitiveTypeList,
    ) -> PrimitiveViolationList {
        PrimitiveViolationList::new()
    }

    fn find_unused_imports(&self, _path: &FilePath) -> ImportInfoList {
        ImportInfoList::new()
    }

    fn get_class_definitions(&self, _path: &FilePath) -> Result<MetadataVO, SourceParserError> {
        Ok(MetadataVO::new(std::collections::HashMap::new()))
    }

    fn get_function_definitions(&self, _path: &FilePath) -> MetadataVO {
        MetadataVO::new(std::collections::HashMap::new())
    }

    fn is_symbol_exported(&self, _path: &FilePath, _symbol: &SymbolName) -> SuccessStatus {
        SuccessStatus::new(false)
    }

    fn get_class_methods(&self, _path: &FilePath) -> MetadataVO {
        MetadataVO::new(std::collections::HashMap::new())
    }

    fn get_class_bases_map(&self, _path: &FilePath) -> MetadataVO {
        MetadataVO::new(std::collections::HashMap::new())
    }

    fn get_assignment_targets(&self, _path: &FilePath) -> MetadataVO {
        MetadataVO::new(std::collections::HashMap::new())
    }

    fn get_control_flow_count(&self, _path: &FilePath) -> Count {
        Count::new(0)
    }

    fn is_barrel_file(&self, path: &FilePath) -> bool {
        path.value.ends_with("__init__.py")
    }

    fn get_stem(&self, path: &FilePath) -> SymbolName {
        let p = std::path::Path::new(&path.value);
        SymbolName::new(
            p.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string(),
        )
    }

    fn is_entry_point(&self, path: &FilePath) -> bool {
        path.value.ends_with("main.py") || path.value.ends_with("__main__.py")
    }

    fn get_supported_extensions(&self) -> crate::taxonomy::PatternList {
        crate::taxonomy::PatternList::new(vec![".py".to_string()])
    }
}
