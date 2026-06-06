/// python_ast_tracer — AST-based tracer for Python code analysis.
use crate::contract::naming_variant_port::INamingVariantPort;
use crate::contract::semantic_tracer_port::ISemanticTracerPort;
use crate::infrastructure::PythonNamingVariantProvider;
use crate::taxonomy::{
    CallChainList, DataFlowList, DirectoryPath, FilePath, LineNumber, ResponseData, ScopeRef,
    SemanticError, SymbolName,
};

use async_trait::async_trait;
use std::collections::HashMap;

pub struct PythonTracer;

impl PythonTracer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PythonTracer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ISemanticTracerPort for PythonTracer {
    async fn get_enclosing_scope(
        &self,
        file_path: &FilePath,
        line: LineNumber,
    ) -> Result<Option<ScopeRef>, SemanticError> {
        let path_str = &file_path.value;
        let content = match std::fs::read_to_string(path_str) {
            Ok(c) => c,
            Err(_) => return Ok(None),
        };
        let lines: Vec<&str> = content.lines().collect();
        if line.value == 0 || line.value as usize > lines.len() {
            return Ok(None);
        }
        Ok(Some(ScopeRef::new("module".to_string())))
    }

    async fn trace_call_chain(
        &self,
        _root_dir: &DirectoryPath,
        _target_name: &SymbolName,
    ) -> Result<CallChainList, SemanticError> {
        Ok(CallChainList { values: Vec::new() })
    }

    async fn find_flow(
        &self,
        _file_path: &FilePath,
        _var_name: &SymbolName,
        _start_line: LineNumber,
    ) -> DataFlowList {
        DataFlowList { values: Vec::new() }
    }

    async fn get_variant_dict(&self, name: &SymbolName) -> ResponseData {
        let provider = PythonNamingVariantProvider::new();
        let variant_json = provider.get_variant_dict(name);
        let mut map = HashMap::new();
        if let Some(obj) = variant_json.as_object() {
            for (k, v) in obj {
                map.insert(k.clone(), v.clone());
            }
        }
        ResponseData {
            value: Some(serde_json::json!(map)),
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: HashMap::new(),
        }
    }

    async fn project_wide_rename(
        &self,
        _root_dir: &DirectoryPath,
        _old_name: &SymbolName,
        _new_name: &SymbolName,
    ) -> u32 {
        0
    }

    async fn get_symbol_locations(
        &self,
        _file_path: &FilePath,
        _symbol: &SymbolName,
    ) -> Vec<ResponseData> {
        Vec::new()
    }

    async fn build_variants(&self, name: &SymbolName) -> Vec<SymbolName> {
        let provider = PythonNamingVariantProvider::new();
        let variants = provider.build_variants(name);
        variants.values
    }
}
