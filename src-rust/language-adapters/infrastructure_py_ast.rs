/// python_ast_tracer — AST-based tracer for Python code analysis.
use crate::naming_rules::contract_variant_port::INamingVariantPort;
use crate::semantic_analysis::contract_tracer_port::ISemanticTracerPort;
use /* UNKNOWN: CallChainList */ crate::naming_rules::taxonomy_symbols_vo::CallChainList;
use crate::shared_common::taxonomy_common_vo::Count;
use /* UNKNOWN: DataFlowList */ crate::shared_common::taxonomy_common_vo::DataFlowList;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use /* UNKNOWN: LineNumber */ crate::shared_common::taxonomy_common_vo::LineNumber;
use /* UNKNOWN: ResponseData */ crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use /* UNKNOWN: ResponseDataList */ crate::shared_common::taxonomy_common_vo::ResponseDataList;
use /* UNKNOWN: ScopeRef */ crate::shared_common::taxonomy_lint_vo::ScopeRef;
use /* UNKNOWN: SemanticError */ crate::semantic_analysis::taxonomy_tracer_error::SemanticError;
use /* UNKNOWN: SymbolName */ crate::naming_rules::taxonomy_symbol_vo::SymbolName;
use /* UNKNOWN: SymbolNameList */ crate::naming_rules::taxonomy_symbols_vo::SymbolNameList;

use async_trait::async_trait;

pub struct PythonTracer {
    naming_provider: Box<dyn INamingVariantPort>,
}

impl PythonTracer {
    pub fn new(naming_provider: Box<dyn INamingVariantPort>) -> Self {
        Self { naming_provider }
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
        let value = self.naming_provider.get_variant_dict(name);
        ResponseData {
            value: Some(value),
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: std::collections::HashMap::new(),
        }
    }

    async fn project_wide_rename(
        &self,
        _root_dir: &DirectoryPath,
        _old_name: &SymbolName,
        _new_name: &SymbolName,
    ) -> Count {
        Count::new(0)
    }

    async fn get_symbol_locations(
        &self,
        _file_path: &FilePath,
        _symbol: &SymbolName,
    ) -> ResponseDataList {
        ResponseDataList { values: vec![] }
    }

    async fn build_variants(&self, name: &SymbolName) -> SymbolNameList {
        self.naming_provider.build_variants(name)
    }
}
