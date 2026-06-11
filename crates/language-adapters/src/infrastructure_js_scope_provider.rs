// PURPOSE: JSScopeProvider — provides JavaScript scope metadata for architecture analysis

use language_adapters::contract_scope_port::IJavascriptScopePort;
use language_adapters::taxonomy_semantic_error::SemanticError;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_layer_vo::LineContentVO;
use shared::taxonomy_lint_vo::ScopeBounds;
use shared::taxonomy_name_vo::SymbolName;
use regex::Regex;

pub struct JSScopeProvider {}

impl Default for JSScopeProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl JSScopeProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl IJavascriptScopePort for JSScopeProvider {
    async fn detect_js_scope(
        &self,
        stripped_line: &LineContentVO,
    ) -> Result<Option<SymbolName>, SemanticError> {
        let line_str = &stripped_line.value;
        let class_re = match Regex::new(
            r"class\s+([A-Za-z_\$][A-Za-z0-9_\$]*)(?:\s+extends\s+[A-Za-z_\$][A-Za-z0-9_\$]*)?",
        ) {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };
        if let Some(caps) = class_re.captures(line_str) {
            return Ok(Some(SymbolName::new(format!("class {}", &caps[1]))));
        }
        let func_re = match Regex::new(r"(?:async\s+)?function\s+([A-Za-z_\$][A-Za-z0-9_\$]*)\s*\(")
        {
            Ok(r) => r,
            Err(_) => return Ok(None),
        };
        if let Some(caps) = func_re.captures(line_str) {
            let name = &caps[1];
            if !["if", "for", "while", "switch", "catch", "else"].contains(&name) {
                return Ok(Some(SymbolName::new(format!("function {}", name))));
            }
        }
        Ok(None)
    }

    async fn find_scope_bounds(
        &self,
        _lines: &shared::taxonomy_common_vo::LineContentList,
        _scope_line: Option<LineNumber>,
    ) -> Result<Option<ScopeBounds>, SemanticError> {
        // Placeholder
        Ok(None)
    }
}
