// orphan_indicator_evaluator — Evaluates whether a file is an orphan based on layer-specific indicators.
// Implements IOrphanIndicatorProtocol: is_taxonomy_orphan, is_contract_orphan,
//   is_infra_cap_orphan, is_agent_orphan, is_surface_orphan, is_generic_orphan.

use crate::taxonomy::{
    BooleanVO, LayerDefinition, OrphanIndicatorResult,
    InboundLinkMap, InheritanceMap, FileDefinitionMap, Severity,
};
use std::collections::HashSet;

pub const LAYER_AGENT: &str = "agent";
pub const LAYER_CAPABILITIES: &str = "capabilities";
pub const LAYER_CONTRACT: &str = "contract";
pub const LAYER_INFRASTRUCTURE: &str = "infrastructure";
pub const LAYER_SURFACES: &str = "surfaces";
pub const LAYER_TAXONOMY: &str = "taxonomy";

pub struct OrphanIndicatorEvaluator;

impl OrphanIndicatorEvaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn is_taxonomy_orphan(
        &self,
        _fs: Option<&dyn Fn(&str) -> bool>,
        f: &str,
        root_dir: &str,
        definition: Option<&LayerDefinition>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        if f.ends_with("__init__.py") {
            return OrphanIndicatorResult::new(false, String::new(), Severity::HIGH);
        }

        let is_in_barrel = self._is_file_in_barrel(None, f, root_dir, definition);
        let path_str = f.to_string();

        let mut consumers: HashSet<String> = HashSet::new();
        if let Some(inbound_for_file) = inbound_links.mapping.get(&path_str) {
            for inbound in inbound_for_file {
                let inbound_layer = self._detect_layer_simple(inbound, root_dir);
                let layer_str = inbound_layer.clone();
                if layer_str.contains(LAYER_CONTRACT)
                    || layer_str.contains(LAYER_INFRASTRUCTURE)
                    || layer_str.contains(LAYER_CAPABILITIES)
                    || layer_str.contains(LAYER_SURFACES)
                {
                    consumers.insert(inbound.clone());
                }
            }
        }

        if !is_in_barrel.value && consumers.is_empty() {
            return OrphanIndicatorResult::new(
                true,
                "TAXONOMY ACCOUNTABILITY: Missing from barrel AND no usage in Contract, Infra, Capability, or Surface layers.".to_string(),
                Severity::CRITICAL,
            );
        }
        OrphanIndicatorResult::new(false, String::new(), Severity::HIGH)
    }

    fn _detect_layer_simple(&self, file_path: &str, root_dir: &str) -> String {
        let rel = file_path.trim_start_matches(root_dir).trim_start_matches('/');
        for (layer, _) in &[("contract", "contract"), ("infrastructure", "infrastructure"), ("capabilities", "capabilities"), ("surfaces", "surfaces"), ("agent", "agent")] {
            if rel.starts_with(layer) {
                return layer.to_string();
            }
        }
        String::new()
    }

    fn _has_barrel_export_for_file(&self, basename: &str) -> BooleanVO {
        BooleanVO::new(basename == "__init__.py")
    }

    fn _check_contract_import_existence(
        &self,
        classes: &[String],
        inheritance_map: &InheritanceMap,
        target_layers: &[&str],
        file_to_layer: &dyn Fn(&str) -> String,
    ) -> bool {
        for cls in classes {
            if let Some(heir_files) = inheritance_map.mapping.get(cls) {
                for heir_fp in heir_files {
                    let heir_layer = file_to_layer(heir_fp);
                    if target_layers.contains(&heir_layer.as_str()) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn is_contract_orphan(
        &self,
        _fs: Option<&dyn Fn(&str) -> bool>,
        f: &str,
        root_dir: &str,
        file_definitions: &FileDefinitionMap,
        inheritance_map: &InheritanceMap,
    ) -> OrphanIndicatorResult {
        let path_str = f.to_string();
        let basename = f.split('/').next_back().unwrap_or("").to_string();

        if self._has_barrel_export_for_file(&basename).value {
            return OrphanIndicatorResult::new(false, String::new(), Severity::HIGH);
        }

        let classes: Vec<String> = file_definitions.mapping.get(&path_str)
            .map(|v| v.clone())
            .unwrap_or_default();

        if classes.is_empty() {
            return OrphanIndicatorResult::new(false, String::new(), Severity::HIGH);
        }

        let target_layers: Vec<&str> = if basename.ends_with("_port.py") {
            vec![LAYER_INFRASTRUCTURE]
        } else if basename.ends_with("_protocol.py") {
            vec![LAYER_CAPABILITIES]
        } else if basename.ends_with("_aggregate.py") {
            vec![LAYER_AGENT]
        } else {
            vec![LAYER_INFRASTRUCTURE, LAYER_CAPABILITIES, LAYER_AGENT]
        };

        let has_heirs = self._check_contract_import_existence(
            &classes, inheritance_map, &target_layers, &|f| self._detect_layer_simple(f, root_dir)
        );

        if !has_heirs {
            let target_desc = target_layers.join(" / ");
            OrphanIndicatorResult::new(
                true,
                format!(
                    "CONTRACT ORPHAN: File '{}' has no grounded heirs. No implementations found in {}.",
                    basename, target_desc
                ),
                Severity::HIGH,
            )
        } else {
            OrphanIndicatorResult::new(false, String::new(), Severity::HIGH)
        }
    }

    pub fn is_infra_cap_orphan(
        &self,
        is_wired: bool,
        is_reachable: bool,
    ) -> OrphanIndicatorResult {
        if !is_wired && !is_reachable {
            OrphanIndicatorResult::new(
                true,
                "REGISTRATION & EXECUTION: Not registered in Agent Container AND unreachable from any Surface 'button press'.".to_string(),
                Severity::CRITICAL,
            )
        } else if !is_wired {
            OrphanIndicatorResult::new(
                true,
                "REGISTRATION ORPHAN: Reachable from logic but not registered in any Agent Container for Dependency Injection.".to_string(),
                Severity::MEDIUM,
            )
        } else {
            OrphanIndicatorResult::new(false, String::new(), Severity::HIGH)
        }
    }

    pub fn is_agent_orphan(&self, is_wired: bool) -> OrphanIndicatorResult {
        if !is_wired {
            OrphanIndicatorResult::new(
                true,
                "AGENT ORPHAN: Internal agent component is not registered in the DI Container.".to_string(),
                Severity::HIGH,
            )
        } else {
            OrphanIndicatorResult::new(false, String::new(), Severity::HIGH)
        }
    }

    pub fn is_surface_orphan(
        &self,
        f: &str,
        alive_files_set: &Vec<String>,
        definition: Option<&LayerDefinition>,
    ) -> OrphanIndicatorResult {
        let basename = f.split('/').next_back().unwrap_or("").to_string();

        if let Some(def) = definition {
            let epts = &def.orphan_entry_points.values;
            if epts.contains(&basename) || epts.contains(&f.to_string()) {
                return OrphanIndicatorResult::new(false, String::new(), Severity::HIGH);
            }
        }

        if !alive_files_set.contains(&f.to_string()) {
            OrphanIndicatorResult::new(
                true,
                "SURFACE ORPHAN: Surface component is not reachable from any main entry point (CLI/MCP).".to_string(),
                Severity::HIGH,
            )
        } else {
            OrphanIndicatorResult::new(false, String::new(), Severity::HIGH)
        }
    }

    pub fn is_generic_orphan(
        &self,
        f: &str,
        alive_files_set: &Vec<String>,
        inbound_links: &InboundLinkMap,
    ) -> OrphanIndicatorResult {
        let path_str = f.to_string();
        let is_reachable = alive_files_set.contains(&f.to_string());
        let has_inbound = inbound_links.mapping.get(&path_str).map_or(false, |v| !v.is_empty());

        if !is_reachable && !has_inbound {
            OrphanIndicatorResult::new(
                true,
                "GENERIC ORPHAN: Unreachable from entry points and has zero inbound imports.".to_string(),
                Severity::HIGH,
            )
        } else {
            OrphanIndicatorResult::new(false, String::new(), Severity::HIGH)
        }
    }

    fn _is_file_in_barrel(&self, _fs: Option<&dyn Fn(&str) -> String>, file_path: &str, root_dir: &str, definition: Option<&LayerDefinition>) -> BooleanVO {
        if let Some(def) = definition {
            if !def.barrel_completeness.value {
                return BooleanVO::new(true);
            }

            // Simplified: assume barrel check
            // In real implementation, we'd check if the file's stem appears in the barrel's imports
            let _ = (file_path, root_dir);
            return BooleanVO::new(false);
        }
        BooleanVO::new(false)
    }
}