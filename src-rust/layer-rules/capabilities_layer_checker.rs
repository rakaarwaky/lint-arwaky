use crate::layer_rules::taxonomy_definition_vo::LayerDefinition;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;

pub struct ArchLayerChecker {}

impl ArchLayerChecker {
    pub fn new() -> Self {
        Self {}
    }

    fn resolve_scope_inheritance(scope: &str) -> (&str, Vec<&str>) {
        if let Some(paren) = scope.find('(') {
            let layer = scope[..paren].trim();
            let inner = scope[paren + 1..].trim_end_matches(')').trim();
            let suffixes: Vec<&str> = if inner.contains('|') {
                inner.split('|').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
            } else {
                inner.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
            };
            (layer, suffixes)
        } else {
            (scope.trim(), vec![])
        }
    }

    pub fn check_forbidden_inheritance(
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
            if !t.starts_with("use ") {
                continue;
            }
            for pattern in &def.forbidden_inheritance.values {
                let (layer, suffixes) = Self::resolve_scope_inheritance(pattern);
                let lower = t.to_lowercase();
                let layer_match = lower.contains(&format!("{}::", layer))
                    || lower.contains(&format!("::{}::", layer));
                if !layer_match {
                    continue;
                }
                if !suffixes.is_empty() {
                    let suffix_match = suffixes.iter().any(|s| {
                        lower.contains(&format!("_{}", s)) || lower.contains(&format!("::{}", s))
                    });
                    if !suffix_match {
                        continue;
                    }
                }
                if let Some(name) = t.split("::").last() {
                    let trait_name = name
                        .trim_end_matches(';')
                        .trim()
                        .trim_start_matches('{')
                        .trim_end_matches('}')
                        .split(',')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    if !trait_name.is_empty() {
                        forbidden_traits.push(trait_name);
                    }
                }
            }
        }
        for trait_name in &forbidden_traits {
            if content.contains(&format!("impl {} for ", trait_name)) {
                let msg = if !def.forbidden_inheritance_violation_message.value.is_empty() {
                    def.forbidden_inheritance_violation_message.value.clone()
                } else {
                    format!(
                        "AES026 FORBIDDEN_INHERITANCE: '{}' implemented from forbidden source.",
                        trait_name
                    )
                };
                violations.push(LintResult::new_arch(file, 0, "AES026", Severity::HIGH, &msg));
            }
        }
    }

    pub fn check_surface_imports(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if layer != "surfaces" && !layer.starts_with("surfaces(") {
            return;
        }
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("use ")
                && (t.contains("::capabilities::")
                    || t.contains("::infrastructure::")
                    || t.contains("::agent::"))
            {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES023",
                    Severity::HIGH,
                    "AES023 SURFACE_DEPENDENCY: Surface imports from forbidden layer.",
                ));
                break;
            }
        }
    }

    pub fn check_capability_routing(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if layer != "capabilities" && !layer.starts_with("capabilities(") {
            return;
        }
        let structs: Vec<&str> = content
            .lines()
            .filter_map(|l| {
                let t = l.trim();
                if t.starts_with("pub struct ") || t.starts_with("struct ") {
                    Some(t.split_whitespace().nth(1).unwrap_or("").trim_end_matches(';'))
                } else {
                    None
                }
            })
            .filter(|n| !n.is_empty() && !n.starts_with('_'))
            .collect();
        for s in &structs {
            let hi = content.contains(&format!("impl I{}", s))
                || content.contains(&format!(" for {} ", s));
            if !hi && structs.len() <= 3 {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES030",
                    Severity::MEDIUM,
                    &format!("AES030 CAPABILITY_ROUTING: Struct '{}' no trait impl.", s),
                ));
            }
        }
    }
}
