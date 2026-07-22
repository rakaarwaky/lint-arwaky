// PURPOSE: Config merger utility — pure function for merging rules into layer definitions
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::config_system::taxonomy_config_vo::{ArchitectureConfig, ArchitectureRule};
use std::collections::HashMap;

/// Merge all rules into layer definitions.
///
/// Returns (merged_layers, rules_by_layer_index):
/// - `merged_layers`: layer name → merged LayerDefinition
/// - `rules_by_layer_index`: scope string → list of rules (for specialized sub-layer creation)
pub fn merge_config(
    config: &ArchitectureConfig,
) -> (
    HashMap<LayerNameVO, LayerDefinition>,
    HashMap<String, Vec<&ArchitectureRule>>,
) {
    // Step 1: Index all rules by layer scope
    let rules_by_layer = index_rules_by_scope(&config.rules);

    // Step 2: Merge global + base-layer rules into each layer definition
    let mut merged_layers: HashMap<LayerNameVO, LayerDefinition> = HashMap::new();
    for (lname, mut ldef) in config.layers.clone() {
        let lstr = lname.to_string();
        let base_name = match lstr.split('(').next() {
            Some(s) => s.to_string(),
            None => lstr.to_string(),
        };

        // Apply: global rules (key="") + base-layer rules (key=base_name)
        for key in &[String::new(), base_name.clone()] {
            if let Some(rules) = rules_by_layer.get(key.as_str()) {
                for rule in rules {
                    // Skip specialised scoped rules when processing base layers
                    if key.as_str() == base_name && rule.scope.value.contains('(') {
                        continue;
                    }
                    merge_rule_into_definition(&mut ldef, rule);
                }
            }
        }
        merged_layers.insert(lname, ldef);
    }

    // Step 3: Create specialised sub-layer entries from scoped rules
    for rule in &config.rules {
        let scope = rule.scope.to_string();
        if !scope.contains('(') {
            continue;
        }
        create_specialized_sub_layers(&mut merged_layers, &rules_by_layer, &scope);
    }

    (merged_layers, rules_by_layer)
}

/// Index rules by their scope (both base name and full scoped name).
fn index_rules_by_scope(rules: &[ArchitectureRule]) -> HashMap<String, Vec<&ArchitectureRule>> {
    let mut index: HashMap<String, Vec<&ArchitectureRule>> = HashMap::new();
    for rule in rules {
        let scope = rule.scope.to_string();
        let base_key = if scope.is_empty() {
            String::new()
        } else {
            match scope.split('(').next() {
                Some(s) => s.to_string(),
                None => scope.to_string(),
            }
        };
        index.entry(base_key).or_default().push(rule);
        if scope.contains('(') {
            index.entry(scope.clone()).or_default().push(rule);
        }
    }
    index
}

/// Merge a single rule's values into a layer definition.
fn merge_rule_into_definition(ldef: &mut LayerDefinition, rule: &ArchitectureRule) {
    if !rule.exceptions.values.is_empty() {
        for val in &rule.exceptions.values {
            if !ldef.exceptions.values.contains(val) {
                ldef.exceptions.values.push(val.clone());
            }
        }
    }
    if !rule.mandatory.values.is_empty() {
        for val in &rule.mandatory.values {
            if !ldef.mandatory.values.contains(val) {
                ldef.mandatory.values.push(val.clone());
            }
        }
    }
    if !rule.forbidden.values.is_empty() {
        for val in &rule.forbidden.values {
            if !ldef.forbidden.values.contains(val) {
                ldef.forbidden.values.push(val.clone());
            }
        }
    }
    if rule.code_analysis.min_lines.value > 0 {
        ldef.code_analysis.min_lines = rule.code_analysis.min_lines.clone();
    }
    if rule.code_analysis.max_lines.value > 0 {
        ldef.code_analysis.max_lines = rule.code_analysis.max_lines.clone();
    }
    if rule.code_analysis.mandatory_class_definition.value {
        ldef.code_analysis.mandatory_class_definition =
            rule.code_analysis.mandatory_class_definition.clone();
    }
    if !rule.code_analysis.forbidden_inheritance.values.is_empty() {
        for val in &rule.code_analysis.forbidden_inheritance.values {
            if !ldef
                .code_analysis
                .forbidden_inheritance
                .values
                .contains(val)
            {
                ldef.code_analysis
                    .forbidden_inheritance
                    .values
                    .push(val.clone());
            }
        }
    }
    // Enable orphan checking if explicitly set OR if the rule is enabled
    // (AES5xx rules use `enabled: true` to activate orphan detection)
    if rule.orphan.check_orphan.value || rule.enabled.value {
        ldef.orphan.check_orphan = BooleanVO::new(true);
    }
    if !rule.orphan.orphan_entry_points.values.is_empty() {
        for val in &rule.orphan.orphan_entry_points.values {
            if !ldef.orphan.orphan_entry_points.values.contains(val) {
                ldef.orphan.orphan_entry_points.values.push(val.clone());
            }
        }
    }
}

/// Create specialised sub-layer entries from a scoped rule.
///
/// E.g., scope "agent(container|registry)" with base layer "agent":
/// - Clones agent definition
/// - Overlays container-specific rules
/// - Inserts "agent(container)" and "agent(registry)" as new sub-layers
fn create_specialized_sub_layers(
    merged_layers: &mut HashMap<LayerNameVO, LayerDefinition>,
    rules_by_layer: &HashMap<String, Vec<&ArchitectureRule>>,
    scope: &str,
) {
    if let Some(paren_start) = scope.find('(') {
        let base_name = scope[..paren_start].trim();
        let inner = scope[paren_start + 1..].trim_end_matches(')').trim();

        // Get base definition (clone to avoid borrow conflict)
        let base_key_str = base_name.to_string();
        let base_def_opt = {
            let base_key = LayerNameVO::new(&base_key_str);
            merged_layers.get(&base_key).cloned()
        };

        if let Some(base_def) = base_def_opt {
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

            for suffix in suffixes {
                let specialized_key = LayerNameVO::new(format!("{}({})", base_name, suffix));
                if merged_layers.contains_key(&specialized_key) {
                    continue;
                }
                let mut spec_def = base_def.clone();
                if let Some(rules) = rules_by_layer.get(scope) {
                    for r in rules {
                        merge_rule_into_definition(&mut spec_def, r);
                    }
                }
                merged_layers.insert(specialized_key, spec_def);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::taxonomy_common_vo::{Count, PatternList};

    fn make_config(
        layers: HashMap<LayerNameVO, LayerDefinition>,
        rules: Vec<ArchitectureRule>,
    ) -> ArchitectureConfig {
        ArchitectureConfig {
            enabled: BooleanVO::new(true),
            layers,
            rules,
            naming: crate::common::taxonomy_definition_vo::NamingConfig::new(Count::new(2)),
            ignored_paths: crate::common::taxonomy_paths_vo::FilePathList { values: vec![] },
            mandatory_class_definition: BooleanVO::new(false),
        }
    }

    #[test]
    fn merge_empty_config() {
        let config = make_config(HashMap::new(), vec![]);
        let (merged, _) = merge_config(&config);
        assert!(merged.is_empty());
    }

    #[test]
    fn merge_global_rule() {
        let mut layers = HashMap::new();
        layers.insert(LayerNameVO::new("agent"), LayerDefinition::default());
        let rule = ArchitectureRule {
            scope: LayerNameVO::new(""),
            forbidden: PatternList {
                values: vec!["capabilities".to_string()],
            },
            ..Default::default()
        };
        let config = make_config(layers, vec![rule]);
        let (merged, _) = merge_config(&config);
        assert!(merged[&LayerNameVO::new("agent")]
            .forbidden
            .values
            .contains(&"capabilities".to_string()));
    }
}
