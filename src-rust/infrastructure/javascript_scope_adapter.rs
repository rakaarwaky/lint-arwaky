//! javascript_scope_tracer — Enclosing scope detection for JS/TS files.

use crate::contract::javascript_scope_port::IJsTracerPort;
use crate::taxonomy::{ErrorMessage, FilePath, LineNumber, ScopeRef, SemanticError};
use regex::Regex;

pub struct JSScopeTracer;

impl JSScopeTracer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JSScopeTracer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl IJsTracerPort for JSScopeTracer {
    async fn show_enclosing_scope(
        &self,
        file_path: &FilePath,
        line: LineNumber,
    ) -> Result<Option<ScopeRef>, SemanticError> {
        let path_str = &file_path.value;
        if !std::path::Path::new(path_str).exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(path_str)
            .map_err(|e| SemanticError::new(ErrorMessage::new(format!("Failed to read: {}", e))))?;
        let lines: Vec<&str> = content.lines().collect();
        let target = line.value as usize;
        if target == 0 || target > lines.len() {
            return Ok(None);
        }

        let class_re = Regex::new(
            r"class\s+([A-Za-z_$][A-Za-z0-9_$]*)(?:\s+extends\s+[A-Za-z_$][A-Za-z0-9_$]*)?",
        )
        .unwrap();
        let func_re =
            Regex::new(r"(?:async\s+)?function\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*\(").unwrap();
        let method_re =
            Regex::new(r"^\s+(?:async\s+)?([A-Za-z_$][A-Za-z0-9_$]*)\s*\([^)]*\)\s*\{").unwrap();

        let mut scope_stack: Vec<(String, usize)> = Vec::new();
        let mut brace_depth: i32 = 0;
        let mut scope_depths: Vec<i32> = Vec::new();

        for (i, raw_line) in lines.iter().enumerate() {
            let current_line = i + 1;
            let stripped = raw_line.trim();

            if let Some(caps) = class_re.captures(stripped) {
                let name = format!("class {}", &caps[1]);
                scope_stack.push((name, current_line));
                scope_depths.push(brace_depth);
            } else if let Some(caps) = func_re.captures(stripped) {
                let name = &caps[1];
                if !["if", "for", "while", "switch", "catch", "else"].contains(&name) {
                    let name = format!("function {}", name);
                    scope_stack.push((name, current_line));
                    scope_depths.push(brace_depth);
                }
            } else if let Some(caps) = method_re.captures(stripped) {
                let name = &caps[1];
                if !["if", "for", "while", "switch", "catch", "else"].contains(&name) {
                    let name = format!("method {}", name);
                    scope_stack.push((name, current_line));
                    scope_depths.push(brace_depth);
                }
            }

            let open = stripped.matches('{').count() as i32;
            let close = stripped.matches('}').count() as i32;
            brace_depth += open - close;

            while let Some(&depth) = scope_depths.last() {
                if brace_depth <= depth && !scope_stack.is_empty() {
                    scope_stack.pop();
                    scope_depths.pop();
                } else {
                    break;
                }
            }

            if current_line == target {
                break;
            }
        }

        if let Some((name, start_line)) = scope_stack.last() {
            Ok(Some(ScopeRef {
                name: name.clone(),
                kind: if name.starts_with("class") {
                    "class".to_string()
                } else {
                    "function".to_string()
                },
                file: Some(file_path.clone()),
                start_line: Some(LineNumber::new(*start_line as i64)),
                end_line: None,
            }))
        } else {
            Ok(Some(ScopeRef::new("module".to_string())))
        }
    }
}
