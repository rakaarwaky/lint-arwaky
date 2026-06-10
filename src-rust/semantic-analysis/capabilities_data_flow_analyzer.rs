// PURPOSE: Analyzer: Data Flow analysis logic

use std::collections::HashSet;
use std::fs;

use regex::Regex;

use crate::naming_rules::taxonomy_name_vo::SymbolName;
use crate::semantic_analysis::contract_data_flow_protocol::IDataFlowProtocol;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::DataFlowList;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::source_parsing::taxonomy_path_vo::FilePath;

/// A single data flow entry describing a variable's usage at a line.
pub struct DataFlowEntry {
    pub line: usize,
    pub kind: String, // "Assignment", "Mutation", "Usage"
    pub content: String,
}

/// Business logic for tracking variable lifecycle in JS/TS files.
pub struct DataFlowAnalyzer {}

impl Default for DataFlowAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl DataFlowAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Track assignments and usages of a variable in a JS/TS file.
    pub fn find_flow(
        &self,
        file_path: &str,
        var_name: &str,
        start_line: Option<usize>,
    ) -> Vec<DataFlowEntry> {
        let Ok(content) = fs::read_to_string(file_path) else {
            return vec![];
        };

        // Determine scope bounds — basic brace counting if start_line provided
        let (scope_start, scope_end) = if let Some(line) = start_line {
            let lines: Vec<&str> = content.lines().collect();
            if line > 0 && line <= lines.len() {
                let mut depth = 0i32;
                let mut found = false;
                let mut start = None;
                let mut end = None;
                for (i, l) in lines.iter().enumerate() {
                    let line_no = i + 1;
                    for c in l.chars() {
                        match c {
                            '{' => depth += 1,
                            '}' => depth -= 1,
                            _ => {}
                        }
                    }
                    if line_no == line {
                        found = true;
                        start = if depth > 0 { Some(line_no) } else { None };
                    }
                    if found && depth == 0 && start.is_some() {
                        end = Some(line_no);
                        break;
                    }
                }
                (start, end)
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        let word_pattern = match Regex::new(&format!(r"\b{}\b", regex::escape(var_name))) {
            Ok(r) => r,
            Err(_) => return vec![],
        };
        let assign_pattern = match Regex::new(&format!(
            r"(?:const|let|var)\s+{}\s*=|(?<![=!<>]){}s*=",
            regex::escape(var_name),
            regex::escape(var_name)
        )) {
            Ok(r) => r,
            Err(_) => return vec![],
        };
        let mutation_pattern = match Regex::new(&format!(
            r"\b{}\.(?:push|pop|shift|unshift|splice|sort|reverse|set|delete|add|assign|merge|update|append|extend)\b",
            regex::escape(var_name)
        )) {
            Ok(r) => r,
            Err(_) => return vec![],
        };

        let mut flows: Vec<DataFlowEntry> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();

        for (i, line_str) in content.lines().enumerate() {
            let line_no = i + 1;

            if let Some(s) = scope_start {
                if line_no < s {
                    continue;
                }
            }
            if let Some(e) = scope_end {
                if line_no > e {
                    break;
                }
            }

            if !word_pattern.is_match(line_str) {
                continue;
            }

            let stripped = line_str.trim();
            let entry_str = if let Some(m) = mutation_pattern.find(line_str) {
                // Extract the method name from the match
                let method = m.as_str().split('.').nth(1).unwrap_or("mutation");
                format!("Line {} [Mutation '{}']: {}", line_no, method, stripped)
            } else if assign_pattern.is_match(line_str) {
                format!("Line {} [Assignment]: {}", line_no, stripped)
            } else {
                format!("Line {} [Usage]: {}", line_no, stripped)
            };

            if seen.contains(&entry_str) {
                continue;
            }
            seen.insert(entry_str.clone());

            let kind = if mutation_pattern.is_match(line_str) {
                "Mutation".to_string()
            } else if assign_pattern.is_match(line_str) {
                "Assignment".to_string()
            } else {
                "Usage".to_string()
            };

            flows.push(DataFlowEntry {
                line: line_no,
                kind,
                content: stripped.to_string(),
            });
        }

        flows
    }
}

impl IDataFlowProtocol for DataFlowAnalyzer {
    fn find_flow(
        &self,
        file_path: &FilePath,
        var_name: &SymbolName,
        start_line: LineNumber,
    ) -> DataFlowList {
        let entries = self.find_flow(
            file_path.value(),
            var_name.value(),
            Some(start_line.value() as usize),
        );
        let messages: Vec<ErrorMessage> = entries
            .into_iter()
            .map(|e| ErrorMessage::new(format!("Line {} [{}]: {}", e.line, e.kind, e.content)))
            .collect();
        DataFlowList::new(messages)
    }
}
