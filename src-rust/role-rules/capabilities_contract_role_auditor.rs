// PURPOSE: ContractRoleChecker — IContractRoleChecker for AES0302: contract port/protocol/aggregate role audits
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
fn aes013_forbidden_inheritance(trait_name: &str) -> String {
    format!(
        "AES013 FORBIDDEN_INHERITANCE: '{}' implemented from forbidden source.",
        trait_name
    )
}

pub struct ContractRoleChecker {}

impl Default for ContractRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_port(&self) -> Vec<LintResult> {
        vec![]
    }
    pub fn check_protocol(&self) -> Vec<LintResult> {
        vec![]
    }

    pub fn check_aggregate(
        &self,
        file: &str,
        content: &str,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if def.forbidden_inheritance.values.is_empty() {
            return;
        }
        let mut forbidden_traits: Vec<String> = Vec::new();
        for line in content.lines() {
            let t = line.trim();
            // Rust: `use crate::layer::TraitName;`
            // Python: `from ..layer.module import TraitName`
            // JS: `import { TraitName } from '../layer/module'`
            let is_import = t.starts_with("use ")
                || (t.starts_with("from ") && t.contains(" import "))
                || (t.starts_with("import ") && t.contains(" from "));
            if !is_import {
                continue;
            }
            for pattern in &def.forbidden_inheritance.values {
                let (layer, suffixes) = Self::resolve_scope(pattern);
                let lower = t.to_lowercase();
                let layer_match = lower.contains(&format!("{}::", layer))
                    || lower.contains(&format!("::{}::", layer))
                    || lower.contains(&format!("{}.", layer))
                    || lower.contains(&format!(".{}.", layer))
                    || lower.contains(&format!("{}/", layer))
                    || lower.contains(&format!("/{}/", layer));
                if !layer_match {
                    continue;
                }
                if !suffixes.is_empty()
                    && !suffixes.iter().any(|s| {
                        lower.contains(&format!("_{}", s)) || lower.contains(&format!("::{}", s))
                    })
                {
                    continue;
                }
                if let Some(name) = t.split("::").last() {
                    let tn = name
                        .trim_end_matches(';')
                        .trim()
                        .trim_start_matches('{')
                        .trim_end_matches('}')
                        .split(',')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    if !tn.is_empty() {
                        forbidden_traits.push(tn);
                    }
                }
            }
        }
        for trait_name in &forbidden_traits {
            // Rust: `impl TraitName for StructName`
            // Python: `class StructName(TraitName):` or `class StructName(TraitName, ...):`
            // JS: `class StructName extends TraitName` or `class StructName implements TraitName`
            let rust_pattern = format!("impl {} for ", trait_name);
            let py_pattern = format!("({}", trait_name);
            let js_extends = format!("extends {}", trait_name);
            let js_implements = format!("implements {}", trait_name);
            if content.contains(&rust_pattern)
                || content.contains(&py_pattern)
                || content.contains(&js_extends)
                || content.contains(&js_implements)
            {
                let msg = aes013_forbidden_inheritance(trait_name);
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES013",
                    Severity::HIGH,
                    &msg,
                ));
            }
        }
    }

    fn resolve_scope(scope: &str) -> (&str, Vec<&str>) {
        if let Some(paren) = scope.find('(') {
            let layer = scope[..paren].trim();
            let inner = scope[paren + 1..].trim_end_matches(')').trim();
            let suffixes: Vec<&str> = if inner.contains('|') {
                inner
                    .split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                inner
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect()
            };
            (layer, suffixes)
        } else {
            (scope.trim(), vec![])
        }
    }
}
