// scope_boundary_analyzer — JS/TS scope boundary detection.
// Implements IScopeBoundaryProtocol: detect_js_scope, find_scope_bounds, get_enclosing_scope.

use std::fs;
use regex::Regex;
use once_cell::sync::Lazy;
use crate::taxonomy::{FilePath, LineNumber};

/// Patterns to detect JS/TS function definitions
static FUNCTION_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| vec![
    // Standard function declarations
    Regex::new(r"(?:async\s+)?function\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*\(").unwrap(),
    // Arrow functions assigned to const/let/var
    Regex::new(r"(?:const|let|var)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*(?:async\s*)?(?:\([^)]*\)|[A-Za-z_$][A-Za-z0-9_$]*)\s*=>").unwrap(),
    // Method definitions in classes
    Regex::new(r"^\s*(?:async\s+|static\s+|private\s+|protected\s+|public\s+)*([A-Za-z_$][A-Za-z0-9_$]*)\s*\(").unwrap(),
]);

static CLASS_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"class\s+([A-Za-z_$][A-Za-z0-9_$]*)(?:\s+extends\s+[A-Za-z_$][A-Za-z0-9_$]*)?").unwrap()
});

const JS_KEYWORDS: &[&str] = &["if", "for", "while", "switch", "catch", "else"];

/// Capability for detecting JS/TS scope boundaries.
pub struct ScopeBoundaryAnalyzer;

impl ScopeBoundaryAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Detect if a stripped JS/TS line opens a named scope.
    pub fn detect_js_scope(&self, stripped_line: &str) -> Option<String> {
        // Check for class definition first
        if let Some(captures) = CLASS_PATTERN.captures(stripped_line) {
            if let Some(name) = captures.get(1) {
                return Some(format!("class {}", name.as_str()));
            }
        }

        // Check for function patterns
        for pattern in FUNCTION_PATTERNS.iter() {
            if let Some(captures) = pattern.captures(stripped_line) {
                if let Some(name) = captures.get(1) {
                    let n = name.as_str();
                    if !JS_KEYWORDS.contains(&n) {
                        return Some(format!("function {}", n));
                    }
                }
            }
        }

        None
    }

    /// Find start/end line numbers of enclosing function body via brace counting.
    pub fn find_scope_bounds(
        &self,
        content: &str,
        scope_line: Option<usize>,
    ) -> (Option<usize>, Option<usize>) {
        let scope_line = match scope_line {
            Some(l) => l,
            None => return (None, None),
        };

        let lines: Vec<&str> = content.lines().collect();
        let mut brace_depth: i32 = 0;
        let mut scope_start: Option<usize> = None;
        let mut scope_end: Option<usize> = None;

        for i in (scope_line.saturating_sub(1))..lines.len() {
            let line = lines[i];
            if line.contains('{') && scope_start.is_none() {
                scope_start = Some(i + 1);
                brace_depth = 1;
                continue;
            }
            if scope_start.is_some() {
                brace_depth += line.chars().filter(|&c| c == '{').count() as i32;
                brace_depth -= line.chars().filter(|&c| c == '}').count() as i32;
                if brace_depth <= 0 {
                    scope_end = Some(i + 1);
                    break;
                }
            }
        }

        (scope_start, scope_end)
    }

    /// Find the nearest enclosing function or class for a given 1-indexed line.
    pub fn get_enclosing_scope(&self, file_path: &str, target_line: usize) -> Option<String> {
        let content = fs::read_to_string(file_path).ok()?;
        let lines: Vec<&str> = content.lines().collect();

        // Stack of (scope_name, brace_depth_when_opened)
        let mut scope_stack: Vec<(String, i32)> = Vec::new();
        let mut brace_depth: i32 = 0;

        for (i, raw_line) in lines.iter().enumerate() {
            let current_line_no = i + 1;
            let stripped = raw_line.trim();

            // Pop expired scopes
            while let Some((_, depth)) = scope_stack.last() {
                if brace_depth <= *depth {
                    scope_stack.pop();
                } else {
                    break;
                }
            }

            // Detect and push new scope
            if let Some(scope_name) = self.detect_js_scope(stripped) {
                if raw_line.contains('{') {
                    scope_stack.push((scope_name, brace_depth));
                }
            }

            // Apply brace count for this line
            let open = raw_line.chars().filter(|&c| c == '{').count() as i32;
            let close = raw_line.chars().filter(|&c| c == '}').count() as i32;
            brace_depth += open - close;

            // Pop scopes that closed on this line
            while let Some((_, depth)) = scope_stack.last() {
                if brace_depth <= *depth {
                    scope_stack.pop();
                } else {
                    break;
                }
            }

            if current_line_no == target_line {
                break;
            }
        }

        if scope_stack.is_empty() {
            None
        } else {
            Some(scope_stack.iter().map(|(name, _)| name.as_str()).collect::<Vec<_>>().join(" -> "))
        }
    }
}
