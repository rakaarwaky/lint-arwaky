/// python_ast_tracer — AST-based tracer for Python code analysis.
use crate::contract::semantic_tracer_port::ISemanticTracerPort;
use crate::infrastructure::PythonNamingVariantProvider;
use crate::taxonomy::{
    CallChainList, Count, DataFlowList, DirectoryPath, FilePath, LineNumber, ResponseData,
    ResponseDataList, ScopeRef, SymbolName, SymbolNameList,
};

use async_trait::async_trait;

pub struct PythonTracer;

impl PythonTracer {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ISemanticTracerPort for PythonTracer {
    async fn get_enclosing_scope(
        &self,
        file_path: &FilePath,
        line: LineNumber,
    ) -> Result<Option<ScopeRef>, crate::taxonomy::SemanticError> {
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
    ) -> Result<CallChainList, crate::taxonomy::SemanticError> {
        Ok(CallChainList::new(Vec::new()))
    }

    async fn find_flow(
        &self,
        _file_path: &FilePath,
        _var_name: &SymbolName,
        _start_line: LineNumber,
    ) -> DataFlowList {
        DataFlowList::new(Vec::new())
    }

    async fn get_variant_dict(&self, name: &SymbolName) -> ResponseData {
        PythonNamingVariantProvider::new()
            .get_variant_dict(name.clone())
            .unwrap_or_else(|_| ResponseData::new(serde_json::Value::Null))
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

    async fn build_variants(&self, _name: &SymbolName) -> Vec<SymbolName> {
        Vec::new()
    }
}
