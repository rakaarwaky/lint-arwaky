// domain_type_checker — Capability for enforcing domain type usage over primitives.
// Implements IDomainTypeProtocol: find_primitive_violations.

use std::fs;
use regex::Regex;

/// A single primitive usage violation (line, column, type_name).
pub struct PrimitiveViolation {
    pub line: usize,
    pub column: usize,
    pub type_name: String,
}

/// Business logic for detecting illicit primitive usage in class attributes.
pub struct DomainTypeRuleChecker;

impl DomainTypeRuleChecker {
    pub fn new() -> Self {
        Self
    }

    /// Analyzes Python class attributes to ensure they use domain types instead of primitives.
    /// Returns a list of violations (line, column, type_name).
    pub fn find_primitive_violations(
        &self,
        file_path: &str,
        primitive_types: &[&str],
    ) -> Vec<PrimitiveViolation> {
        let Ok(content) = fs::read_to_string(file_path) else {
            return vec![];
        };

        let mut violations: Vec<PrimitiveViolation> = Vec::new();
        let mut inside_class = false;
        let mut class_indent: usize = 0;

        // Pattern to detect class attribute type annotations: `attr: type` or `attr: type = ...`
        // We look for patterns like `    field: str` or `    field: int = 0`
        let attr_pattern = Regex::new(
            r"^(\s+)([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([A-Za-z_][A-Za-z0-9_\[\], |]*)"
        ).unwrap();

        let class_pattern = Regex::new(r"^(\s*)class\s+[A-Za-z_]").unwrap();

        for (i, line) in content.lines().enumerate() {
            let line_no = i + 1;

            // Detect class definition
            if let Some(caps) = class_pattern.captures(line) {
                class_indent = caps.get(1).map(|m| m.as_str().len()).unwrap_or(0);
                inside_class = true;
                continue;
            }

            if !inside_class {
                continue;
            }

            // Exit class if dedented back to or beyond class level (and not blank)
            let stripped = line.trim();
            if !stripped.is_empty() && !stripped.starts_with('#') {
                let current_indent = line.len() - line.trim_start().len();
                if current_indent <= class_indent && !stripped.starts_with("class ") {
                    inside_class = false;
                    continue;
                }
            }

            // Check for attribute type annotations
            if let Some(caps) = attr_pattern.captures(line) {
                let type_annotation = caps.get(3).map(|m| m.as_str().trim()).unwrap_or("");
                let column = caps.get(2).map(|m| m.start()).unwrap_or(0);

                // Extract base type (before [ or |)
                let base_type = type_annotation
                    .split(['[', '|', ' '])
                    .next()
                    .unwrap_or("")
                    .trim();

                if primitive_types.contains(&base_type) {
                    violations.push(PrimitiveViolation {
                        line: line_no,
                        column,
                        type_name: base_type.to_string(),
                    });
                }
            }
        }

        violations
    }
}
