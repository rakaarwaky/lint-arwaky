// scope_boundary_resolver — Capability for resolving code scope boundaries (JS/TS).
// Implements IScopeBoundaryResolverProtocol: resolve_enclosing_scope, find_scope_bounds.

use std::fs;
use regex::Regex;
use once_cell::sync::Lazy;
use crate::taxonomy::{FilePath, LineNumber, ScopeRef};

static FUNCTION_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| vec![
    Regex::new(r"(?:async\s+)?function\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*\(").unwrap(),
    Regex::new(r"(?:const|let|var)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*(?:async\s*)?(?:\([^)]*\)|[A-Za-z_$][A-Za-z0-9_$]*)\s*=>").unwrap(),
    Regex::new(r"^\s*(?:async\s+|static\s+|private\s+|protected\s+|public\s+)*([A-Za-z_$][A-Za-z0-9_$]*)\s*\(").unwrap(),
]);

static CLASS_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"class\s+([A-Za-z_$][A-Za-z0-9_$]*)(?:\s+extends\s+[A-Za-z_$][A-Za-z0-9_$]*)?").unwrap()
});

const JS_KEYWORDS: &[&str] = &["if", "for", "while", "switch", "catch", "else"];

/// Business logic for detecting and resolving function/class boundaries.
pub struct ScopeBoundaryResolver;

impl ScopeBoundaryResolver {
    pub fn new() -> Self {
        Self
    }

    fn detect_js_scope(stripped_line: &str) -> Option<String> {
        if let Some(captures) = CLASS_PATTERN.captures(stripped_line) {
            if let Some(name) = captures.get(1) {
                return Some(format!("class {}", name.as_str()));
            }
        }
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

    /// Identifies the hierarchy of scopes enclosing a specific line.
    pub fn resolve_enclosing_scope(&self, file_path: &str, target_line: usize) -> Option<String> {
        let content = fs::read_to_string(file_path).ok()?;
        let lines: Vec<&str> = content.lines().collect();

        // Stack of (scope_name, brace_depth_at_open)
        let mut scope_stack: Vec<(String, i32)> = Vec::new();
        let mut brace_depth: i32 = 0;

        for (i, raw_line) in lines.iter().enumerate() {
            let current_line_no = i + 1;
            let stripped = raw_line.trim();

            // Pop expired scopes before processing this line's opening brace
            scope_stack.retain(|(_, depth)| brace_depth > *depth);

            // Detect new scope and push if brace opens on this line
            if let Some(scope_name) = Self::detect_js_scope(stripped) {
                if raw_line.contains('{') {
                    scope_stack.push((scope_name, brace_depth));
                }
            }

            // Update brace depth
            let open = raw_line.chars().filter(|&c| c == '{').count() as i32;
            let close = raw_line.chars().filter(|&c| c == '}').count() as i32;
            brace_depth += open - close;

            // Pop scopes closed by this line
            scope_stack.retain(|(_, depth)| brace_depth > *depth);

            if current_line_no == target_line {
                return if scope_stack.is_empty() {
                    None
                } else {
                    Some(scope_stack.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>().join(" -> "))
                };
            }
        }

        None
    }

    /// Find start/end line numbers of enclosing function body via brace counting.
    pub fn find_scope_bounds(&self, content: &str, scope_line: Option<usize>) -> (Option<usize>, Option<usize>) {
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
}
