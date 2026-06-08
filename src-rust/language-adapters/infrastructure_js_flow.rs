/// javascript_flow_tracer — Variable flow tracking for JS/TS files.
use crate::language_adapters::contract_flow_port::IJavascriptFlowPort;
use /* UNKNOWN: DataFlowList */ crate::shared_common::taxonomy_common_vo::DataFlowList;
use /* UNKNOWN: ErrorMessage */ crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use /* UNKNOWN: LineNumber */ crate::shared_common::taxonomy_common_vo::LineNumber;
use /* UNKNOWN: SemanticError */ crate::semantic_analysis::taxonomy_tracer_error::SemanticError;
use /* UNKNOWN: SymbolName */ crate::naming_rules::taxonomy_symbol_vo::SymbolName;
use regex::Regex;

pub struct JSFlowAdapter {}

impl JSFlowAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl IJavascriptFlowPort for JSFlowAdapter {
    async fn find_flow(
        &self,
        file_path: &FilePath,
        var_name: &SymbolName,
        _start_line: Option<LineNumber>,
    ) -> Result<DataFlowList, SemanticError> {
        let path_str = &file_path.value;
        let var_str = &var_name.value;
        if !std::path::Path::new(path_str).exists() {
            return Err(SemanticError::new(ErrorMessage::new(format!(
                "File does not exist: {}",
                path_str
            ))));
        }
        let content = std::fs::read_to_string(path_str)
            .map_err(|e| SemanticError::new(ErrorMessage::new(format!("Failed to read: {}", e))))?;
        let lines: Vec<&str> = content.lines().collect();
        let word_pattern = match Regex::new(&format!(r"\b{}", regex::escape(var_str))) {
            Ok(r) => r,
            Err(_) => return Err(SemanticError::new(ErrorMessage::new("regex compilation failed"))),
        };
        let mut flows: Vec<ErrorMessage> = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if word_pattern.is_match(line) {
                flows.push(ErrorMessage::new(format!(
                    "Line {} [Usage]: {}",
                    i + 1,
                    line.trim()
                )));
            }
        }
        Ok(DataFlowList::new(flows))
    }

    async fn trace_flow(&self, _path: &FilePath) -> Result<DataFlowList, SemanticError> {
        Ok(DataFlowList::new(vec![]))
    }
}
