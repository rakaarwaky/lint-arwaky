/// scope_boundary_analyzer — Scope boundary detection for JS/TS files.
/// Used by data_flow_analyzer to determine enclosing scope bounds.
use crate::contract::IScopeBoundaryProtocol;
use crate::taxonomy::{
    ContentString, FilePath, LineContentVO, LineNumber, ScopeBounds, SymbolName,
};

pub struct ScopeBoundaryAnalyzer;

impl ScopeBoundaryAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Find scope bounds (start_line, end_line) around a given line.
    pub fn find_scope_bounds(
        &self,
        content: &str,
        line: Option<usize>,
    ) -> (Option<usize>, Option<usize>) {
        let lines: Vec<&str> = content.lines().collect();
        let target = line.unwrap_or(0);
        if target >= lines.len() {
            return (None, None);
        }

        // Simple brace-based scope detection
        let mut depth: i32 = 0;
        let mut scope_start: Option<usize> = None;

        for (i, l) in lines.iter().enumerate() {
            for ch in l.chars() {
                if ch == '{' {
                    if depth == 0 && i <= target {
                        scope_start = Some(i);
                    }
                    depth += 1;
                } else if ch == '}' {
                    depth -= 1;
                    if depth == 0 && i >= target {
                        return (scope_start, Some(i));
                    }
                }
            }
        }

        (scope_start, Some(lines.len().saturating_sub(1)))
    }
}

impl IScopeBoundaryProtocol for ScopeBoundaryAnalyzer {
    fn detect_js_scope(&self, stripped_line: &LineContentVO) -> Option<SymbolName> {
        let line = stripped_line.value().trim();
        if line.starts_with("function ") {
            let name = line
                .trim_start_matches("function ")
                .split(|c: char| c == '(' || c == ' ')
                .next()
                .unwrap_or("");
            if !name.is_empty() {
                return Some(SymbolName::new(name));
            }
        }
        if line.starts_with("class ") {
            let name = line
                .trim_start_matches("class ")
                .split(|c: char| c == '(' || c == ' ' || c == '{')
                .next()
                .unwrap_or("");
            if !name.is_empty() {
                return Some(SymbolName::new(name));
            }
        }
        if let Some(rest) = line.strip_prefix("const ") {
            if rest.contains("= (") || rest.contains("= function") || rest.contains("=>") {
                let name = rest.split('=').next().unwrap_or("").trim();
                if !name.is_empty() {
                    return Some(SymbolName::new(name));
                }
            }
        }
        if let Some(rest) = line.strip_prefix("let ") {
            if rest.contains("= (") || rest.contains("= function") || rest.contains("=>") {
                let name = rest.split('=').next().unwrap_or("").trim();
                if !name.is_empty() {
                    return Some(SymbolName::new(name));
                }
            }
        }
        if let Some(rest) = line.strip_prefix("var ") {
            if rest.contains("= (") || rest.contains("= function") || rest.contains("=>") {
                let name = rest.split('=').next().unwrap_or("").trim();
                if !name.is_empty() {
                    return Some(SymbolName::new(name));
                }
            }
        }
        None
    }

    fn find_scope_bounds(
        &self,
        content: &ContentString,
        scope_line: Option<LineNumber>,
    ) -> ScopeBounds {
        let (start, end) =
            self.find_scope_bounds(content.value(), scope_line.map(|l| l.value() as usize));
        ScopeBounds {
            start: start.map(|s| LineNumber::new(s as i64)),
            end: end.map(|e| LineNumber::new(e as i64)),
        }
    }

    fn get_enclosing_scope(&self, file_path: &FilePath, line: LineNumber) -> Option<SymbolName> {
        let content = std::fs::read_to_string(file_path.value()).ok()?;
        let lines: Vec<&str> = content.lines().collect();
        let target = line.value() as usize;
        if target >= lines.len() {
            return None;
        }
        let mut current_scope: Option<String> = None;
        for (i, l) in lines.iter().enumerate() {
            if i > target {
                break;
            }
            let trimmed = l.trim();
            if let Some(name) = trimmed.strip_prefix("function ") {
                let name = name
                    .split(|c: char| c == '(' || c == ' ')
                    .next()
                    .unwrap_or("");
                if !name.is_empty() {
                    current_scope = Some(name.to_string());
                }
            } else if let Some(name) = trimmed.strip_prefix("class ") {
                let name = name
                    .split(|c: char| c == '(' || c == ' ' || c == '{')
                    .next()
                    .unwrap_or("");
                if !name.is_empty() {
                    current_scope = Some(name.to_string());
                }
            } else if let Some(rest) = trimmed.strip_prefix("const ") {
                if rest.contains("=>") || rest.contains("= function") {
                    let var_name = rest.split('=').next().unwrap_or("").trim();
                    if !var_name.is_empty() {
                        current_scope = Some(var_name.to_string());
                    }
                }
            } else if let Some(rest) = trimmed.strip_prefix("let ") {
                if rest.contains("=>") || rest.contains("= function") {
                    let var_name = rest.split('=').next().unwrap_or("").trim();
                    if !var_name.is_empty() {
                        current_scope = Some(var_name.to_string());
                    }
                }
            } else if let Some(rest) = trimmed.strip_prefix("var ") {
                if rest.contains("=>") || rest.contains("= function") {
                    let var_name = rest.split('=').next().unwrap_or("").trim();
                    if !var_name.is_empty() {
                        current_scope = Some(var_name.to_string());
                    }
                }
            }
        }
        current_scope.map(|s| SymbolName::new(s))
    }
}
