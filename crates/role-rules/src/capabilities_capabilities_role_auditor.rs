// PURPOSE: CapabilitiesRoleChecker — AES403: enforce capability type composition.
//
// ALGORITHM (uses ParseMetadata when available, falls back to line scanning):
//   1. Collect all type declarations (struct/class/enum/interface).
//   2. Rule 1 — Max 3 type declarations per file. If exceeded -> flag CapabilityTooManyTypes.
//      Checked first; if violated, skip implementor check.
//   3. Rule 2 — Must have >= 1 implementor:
//        Rust:   impl <Trait> for <Struct>
//        Python: class <Name>(<Parent>):
//        TS:     class <Name> implements <IProtocol>
//      If none -> flag CapabilityNoImplementor.
//   4. Internal helper types (without implementor pattern) are ALLOWED and not flagged.
//
// NOTE: Import checking is handled by import-rules crate, not role-rules.

use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, ParseMetadata};
use shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct CapabilitiesRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl ICapabilitiesRoleChecker for CapabilitiesRoleChecker {
    fn check_capability_routing(
        &self,
        file: &FileEntry,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if layer != "capabilities" && !layer.starts_with("capabilities(") {
            return;
        }

        if let Some(meta) = &file.parse_metadata {
            self._check_with_metadata(file, meta, violations);
        } else {
            self._check_fallback(file, violations);
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl Default for CapabilitiesRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitiesRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Check using structured ParseMetadata.
    fn _check_with_metadata(
        &self,
        file: &FileEntry,
        meta: &ParseMetadata,
        violations: &mut Vec<LintResult>,
    ) {
        let path_str = file.path.to_string_lossy();
        match meta {
            ParseMetadata::Rust(rust_meta) => {
                let type_count =
                    rust_meta.struct_definitions.len() + rust_meta.enum_definitions.len();
                let struct_names: Vec<&str> = rust_meta
                    .struct_definitions
                    .iter()
                    .map(|s| s.as_str())
                    .collect();

                // Rule 1: max 3 types
                if type_count > 3 {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::HIGH,
                        
                        format!("AES403 CAPABILITY_ROLE: Too many types in capabilities file.\nWHY? Found {} types (struct + enum), max 3 allowed\nFIX: Keep at most 3 types. Move excess structs/enums to the taxonomy layer.", type_count)
,
                    ));
                    return;
                }

                // Rule 2: must have >= 1 struct implementor
                let has_implementor = rust_meta.impl_blocks.iter().any(|imp| {
                    imp.trait_name.is_some()
                        && struct_names.contains(&imp.implementor_type.as_str())
                });
                if !has_implementor {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES403", Severity::MEDIUM,
                        
                        format!("AES403 CAPABILITY_ROLE: No struct implements a _protocol trait.\nWHY? No impl Trait for struct pattern found in {}. At least one struct must implement a _protocol trait.\nFIX: At least one struct in this file must implement the capability _protocol. Convert an existing struct or keep only internal helpers.", path_str)
,
                    ));
                }
            }
            ParseMetadata::Python(py_meta) => {
                let class_count = py_meta.class_declarations.len();
                let implementor_found = py_meta
                    .class_declarations
                    .iter()
                    .any(|c| !c.bases.is_empty());

                if class_count > 3 {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::HIGH,
                        
                        format!("AES403 CAPABILITY_ROLE: Too many types in capabilities file.\nWHY? Found {} classes, max 3 allowed\nFIX: Keep at most 3 types. Move excess structs/enums to the taxonomy layer.", class_count)
,
                    ));
                    return;
                }
                if !implementor_found {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES403", Severity::MEDIUM,
                        
                        format!("AES403 CAPABILITY_ROLE: No struct implements a _protocol trait.\nWHY? No class with parent/inheritance found in {}. At least one class must inherit from a parent class.\nFIX: At least one struct in this file must implement the capability _protocol. Convert an existing struct or keep only internal helpers.", path_str)
,
                    ));
                }
            }
            ParseMetadata::TypeScript(ts_meta) | ParseMetadata::JavaScript(ts_meta) => {
                let type_count = ts_meta.class_declarations.len()
                    + ts_meta.interface_declarations.len()
                    + ts_meta.type_alias_declarations.len();
                let implementor_found = ts_meta
                    .class_declarations
                    .iter()
                    .any(|c| !c.implements.is_empty());

                if type_count > 3 {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::HIGH,
                        
                        format!("AES403 CAPABILITY_ROLE: Too many types in capabilities file.\nWHY? Found {} types (class/interface/enum), max 3 allowed\nFIX: Keep at most 3 types. Move excess structs/enums to the taxonomy layer.", type_count)
,
                    ));
                    return;
                }
                if !implementor_found {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES403", Severity::MEDIUM,
                        
                        format!("AES403 CAPABILITY_ROLE: No struct implements a _protocol trait.\nWHY? No class with 'implements' keyword found in {}. At least one class must implement an interface/protocol.\nFIX: At least one struct in this file must implement the capability _protocol. Convert an existing struct or keep only internal helpers.", path_str)
,
                    ));
                }
            }
            _ => {} // ParseMetadata::Unknown — skip
        }
    }

    /// Fallback: line-based scanning when no parse metadata.
    fn _check_fallback(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy();
        let content = &file.content;
        let lines: Vec<&str> = content.lines().collect();
        let mut type_count: usize = 0;
        let mut implementor_found = false;
        let mut struct_names: Vec<&str> = Vec::new();

        match file.language {
            shared::filesystem::taxonomy_filesystem_vo::Language::Rust => {
                let mut in_cfg_test = false;
                for l in &lines {
                    let t = l.trim();
                    if t.starts_with("#[cfg(test)]") {
                        in_cfg_test = true;
                        continue;
                    }
                    if in_cfg_test {
                        if t.starts_with('}') {
                            in_cfg_test = false;
                        }
                        continue;
                    }
                    let words: Vec<&str> = t.split_whitespace().collect();
                    if (t.starts_with("pub struct ") || t.starts_with("struct "))
                        && words.len() >= 2
                        && let Some(idx) = words.iter().position(|w| *w == "struct")
                        && let Some(name) = words.get(idx + 1)
                    {
                        let name = name.trim_end_matches(';').trim_end_matches('{');
                        if !name.is_empty() && !name.starts_with('_') {
                            type_count += 1;
                            struct_names.push(name);
                        }
                    }
                    if (t.starts_with("pub enum ") || t.starts_with("enum "))
                        && words.len() >= 2
                        && let Some(idx) = words.iter().position(|w| *w == "enum")
                        && let Some(name) = words.get(idx + 1)
                    {
                        let name = name.trim_end_matches(';').trim_end_matches('{');
                        if !name.is_empty() && !name.starts_with('_') {
                            type_count += 1;
                        }
                    }
                }
                if type_count > 3 {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::HIGH,
                        
                        format!("AES403 CAPABILITY_ROLE: Too many types in capabilities file.\nWHY? Found {} types (struct + enum), max 3 allowed\nFIX: Keep at most 3 types. Move excess structs/enums to the taxonomy layer.", type_count)
,
                    ));
                    return;
                }
                let has_implementor = struct_names.iter().any(|s| {
                    lines.iter().any(|l| {
                        let t = l.trim();
                        t.starts_with("impl ")
                            && (t.contains(&format!("for {} ", s))
                                || t.contains(&format!("for {}{{", s))
                                || t.contains(&format!("for {} {{", s)))
                    })
                });
                if !has_implementor {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES403", Severity::MEDIUM,
                        
                        format!("AES403 CAPABILITY_ROLE: No struct implements a _protocol trait.\nWHY? No impl Trait for struct pattern found in {}. At least one struct must implement a _protocol trait.\nFIX: At least one struct in this file must implement the capability _protocol. Convert an existing struct or keep only internal helpers.", path_str)
,
                    ));
                }
            }
            shared::filesystem::taxonomy_filesystem_vo::Language::Python => {
                for l in &lines {
                    let t = l.trim();
                    if t.starts_with("class ") {
                        type_count += 1;
                        if let Some(start) = t.find('(') {
                            let after_paren = &t[start + 1..];
                            if let Some(end) = after_paren.find(')') {
                                let parents = after_paren[..end].trim();
                                if !parents.is_empty() {
                                    implementor_found = true;
                                }
                            }
                        }
                    }
                }
                if type_count > 3 {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::HIGH,
                        
                        format!("AES403 CAPABILITY_ROLE: Too many types in capabilities file.\nWHY? Found {} classes, max 3 allowed\nFIX: Keep at most 3 types. Move excess structs/enums to the taxonomy layer.", type_count)
,
                    ));
                    return;
                }
                if !implementor_found {
                    violations.push(LintResult::new_arch(
                        &path_str, 0, "AES403", Severity::MEDIUM,
                        
                        format!("AES403 CAPABILITY_ROLE: No struct implements a _protocol trait.\nWHY? No class with parent/inheritance found in {}. At least one class must inherit from a parent class.\nFIX: At least one struct in this file must implement the capability _protocol. Convert an existing struct or keep only internal helpers.", path_str)
,
                    ));
                }
            }
            _ => {
                for l in &lines {
                    let t = l.trim();
                    let class_body = t
                        .strip_prefix("export class ")
                        .or_else(|| t.strip_prefix("class "));
                    if let Some(rest) = class_body {
                        type_count += 1;
                        if rest.contains("implements ") {
                            implementor_found = true;
                        }
                        continue;
                    }
                    let iface_body = t
                        .strip_prefix("export interface ")
                        .or_else(|| t.strip_prefix("interface "));
                    if iface_body.is_some() {
                        type_count += 1;
                        continue;
                    }
                    let enum_body = t
                        .strip_prefix("export enum ")
                        .or_else(|| t.strip_prefix("enum "));
                    if enum_body.is_some() {
                        type_count += 1;
                    }
                }
                if type_count > 3 {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::HIGH,
                        
                        format!("AES403 CAPABILITY_ROLE: Too many types in capabilities file.\nWHY? Found {} types, max 3 allowed\nFIX: Keep at most 3 types. Move excess structs/enums to the taxonomy layer.", type_count)
,
                    ));
                    return;
                }
                if !implementor_found {
                    violations.push(LintResult::new_arch(
                        &path_str,
                        0,
                        "AES403",
                        Severity::MEDIUM,
                        
                        format!("AES403 CAPABILITY_ROLE: No struct implements a _protocol trait.\nWHY? No class with 'implements' found in {}.\nFIX: At least one struct in this file must implement the capability _protocol. Convert an existing struct or keep only internal helpers.", path_str)
,
                    ));
                }
            }
        }
    }
}
