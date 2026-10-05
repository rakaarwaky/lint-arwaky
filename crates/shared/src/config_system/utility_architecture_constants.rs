// PURPOSE: Hardcoded AES business rules - the architecture the tool enforces,
//          independent of any project's config file.
//
// Every value here is product business logic: which layer may import which,
// which suffixes a layer accepts, which limits apply. A project config file may
// only switch a rule on or off and add exceptions; it cannot change these
// values.
//
// The tables below reproduce exactly what the rule engine read from
// `config/lint_arwaky.config.yaml` before the values moved into code. Rows are
// keyed by layer scope, one row per rule per scope. `merge_config` folds them
// into the layer definitions the checkers read, including the specialised
// sub-layers (`taxonomy(vo)`, `contract(protocol)`, ...).
//
// Do not edit a value here to silence a violation in one repository - fix the
// code instead.

use crate::taxonomy_config_system_vo::{ArchitectureConfig, ArchitectureRule};
use shared_common::taxonomy_code_analysis_vo::CodeAnalysisRuleVO;
use shared_common::taxonomy_common_vo::{BooleanVO, Count, PatternList, SuffixPolicyVO};
use shared_common::taxonomy_definition_vo::{LayerDefinition, LayerNamingConfig, NamingConfig};
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_suggestion_vo::DescriptionVO;
use std::collections::HashMap;

// ─── Rule row layout ───────────────────────────────────────────────────────

// A rule row is a tuple, in this order: code, layer scope, then the constraint
// lists. Tuples keep this file free of struct/enum/trait/alias/impl
// definitions, which AES404 requires of `utility_*` files. Named indices keep
// the builder readable.

const CODE: usize = 0;
const SCOPE: usize = 1;
const ALLOWED: usize = 2;
const MANDATORY: usize = 3;
const FORBIDDEN: usize = 4;
const EXCEPTIONS: usize = 5;
const FORBIDDEN_BYPASS: usize = 6;

fn list(values: &[&str]) -> PatternList {
    PatternList::new(values.iter().map(|v| (*v).to_string()).collect::<Vec<_>>())
}

/// Split a comma-joined cell into a pattern list. Each constraint cell in a row
/// stores its values comma-joined, so the whole table stays a flat `&[&str]`.
fn split_list(cell: &str) -> PatternList {
    if cell.is_empty() {
        return PatternList::default();
    }
    PatternList::new(cell.split(',').map(|v| v.to_string()).collect::<Vec<_>>())
}

/// Build one rule from a row. `code_analysis` only carries a forbidden-bypass
/// list when the row has one; every other field keeps its struct default, which
/// is the value the engine saw before.
fn into_rule(row: &[&str]) -> ArchitectureRule {
    let code_analysis = if row[FORBIDDEN_BYPASS].is_empty() {
        CodeAnalysisRuleVO::default()
    } else {
        CodeAnalysisRuleVO {
            forbidden_bypass: split_list(row[FORBIDDEN_BYPASS]),
            ..Default::default()
        }
    };
    ArchitectureRule {
        name: DescriptionVO::new(row[CODE]),
        description: DescriptionVO::new(""),
        rule_type: ErrorCode::raw(row[CODE]),
        enabled: BooleanVO::new(true),
        scope: LayerNameVO::new(row[SCOPE]),
        exceptions: split_list(row[EXCEPTIONS]),
        allowed: split_list(row[ALLOWED]),
        forbidden: split_list(row[FORBIDDEN]),
        mandatory: split_list(row[MANDATORY]),
        code_analysis,
        ..Default::default()
    }
}

// ─── The seven AES layers and their suffix policy (AES102) ─────────────────

/// Layer definitions carry only the suffix policy; the dependency matrix lives
/// in the rule table and is folded in by `merge_config`.
fn hardcoded_layers() -> HashMap<LayerNameVO, LayerDefinition> {
    let mut layers = HashMap::new();

    layers.insert(
        LayerNameVO::new("agent"),
        LayerDefinition {
            naming: LayerNamingConfig {
                suffix_policy: SuffixPolicyVO::new("strict".to_string()),
                allowed_suffix: list(&["orchestrator"]),
                forbidden_suffix: list(&[]),
            },
            ..Default::default()
        },
    );

    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition {
            naming: LayerNamingConfig {
                suffix_policy: SuffixPolicyVO::new("flexible".to_string()),
                allowed_suffix: list(&[]),
                forbidden_suffix: list(&[
                    "vo",
                    "entity",
                    "error",
                    "event",
                    "constant",
                    "constants",
                    "protocol",
                    "aggregate",
                    "utility",
                    "request",
                    "response",
                ]),
            },
            ..Default::default()
        },
    );

    layers.insert(
        LayerNameVO::new("contract"),
        LayerDefinition {
            naming: LayerNamingConfig {
                suffix_policy: SuffixPolicyVO::new("strict".to_string()),
                allowed_suffix: list(&["protocol", "aggregate"]),
                forbidden_suffix: list(&[]),
            },
            ..Default::default()
        },
    );

    layers.insert(
        LayerNameVO::new("root"),
        LayerDefinition {
            naming: LayerNamingConfig {
                suffix_policy: SuffixPolicyVO::new("strict".to_string()),
                allowed_suffix: list(&["entry", "container"]),
                forbidden_suffix: list(&[]),
            },
            ..Default::default()
        },
    );

    layers.insert(
        LayerNameVO::new("surface"),
        LayerDefinition {
            naming: LayerNamingConfig {
                suffix_policy: SuffixPolicyVO::new("strict".to_string()),
                allowed_suffix: list(&[
                    "command",
                    "controller",
                    "page",
                    "view",
                    "component",
                    "router",
                    "layout",
                    "hook",
                    "store",
                    "action",
                    "screen",
                ]),
                forbidden_suffix: list(&[]),
            },
            ..Default::default()
        },
    );

    layers.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition {
            naming: LayerNamingConfig {
                suffix_policy: SuffixPolicyVO::new("strict".to_string()),
                allowed_suffix: list(&[
                    "vo", "entity", "error", "event", "constant", "request", "response",
                ]),
                forbidden_suffix: list(&[]),
            },
            ..Default::default()
        },
    );

    layers.insert(
        LayerNameVO::new("utility"),
        LayerDefinition {
            naming: LayerNamingConfig {
                suffix_policy: SuffixPolicyVO::new("flexible".to_string()),
                allowed_suffix: list(&[]),
                forbidden_suffix: list(&[
                    "vo",
                    "entity",
                    "error",
                    "event",
                    "constant",
                    "protocol",
                    "aggregate",
                    "request",
                    "response",
                ]),
            },
            ..Default::default()
        },
    );

    layers
}

// ─── Every AES rule, one row per layer scope ───────────────────────────────

fn hardcoded_rules() -> Vec<ArchitectureRule> {
    RULES.iter().map(|row| into_rule(row)).collect()
}

static RULES: &[&[&str]] = &[
    &["AES101", "taxonomy", "", "", "", "", ""],
    &["AES101", "utility", "", "", "", "", ""],
    &["AES101", "contract", "", "", "", "", ""],
    &["AES101", "capabilities", "", "", "", "", ""],
    &["AES101", "agent", "", "", "", "", ""],
    &["AES101", "surface", "", "", "", "", ""],
    &["AES101", "root", "", "", "", "", ""],
    &["AES102", "taxonomy", "", "", "", "", ""],
    &["AES102", "utility", "", "", "", "", ""],
    &["AES102", "contract", "", "", "", "", ""],
    &["AES102", "capabilities", "", "", "", "", ""],
    &["AES102", "agent", "", "", "", "", ""],
    &["AES102", "surface", "", "", "", "", ""],
    &["AES102", "root", "", "", "", "", ""],
    &["AES103", "", "", "", "", "", ""],
    &[
        "AES201",
        "taxonomy(vo)",
        "taxonomy",
        "",
        "agent,surface,contract,utility,capabilities,root",
        "",
        "",
    ],
    &[
        "AES201",
        "taxonomy(request,response)",
        "taxonomy",
        "",
        "agent,surface,contract,utility,capabilities,root",
        "",
        "",
    ],
    &[
        "AES201",
        "taxonomy(entity,error,event)",
        "taxonomy",
        "taxonomy(vo|constant)",
        "agent,surface,contract,utility,capabilities,root",
        "",
        "",
    ],
    &[
        "AES201",
        "taxonomy(constant)",
        "taxonomy",
        "",
        "agent,surface,contract,utility,capabilities,root",
        "",
        "",
    ],
    &[
        "AES201",
        "utility",
        "taxonomy",
        "",
        "agent,surface,contract,capabilities,root,utility",
        "",
        "",
    ],
    &[
        "AES201",
        "contract(protocol)",
        "taxonomy,contract",
        "taxonomy",
        "agent,surface,capabilities,contract(aggregate),root",
        "",
        "",
    ],
    &[
        "AES201",
        "contract(aggregate)",
        "taxonomy,contract",
        "taxonomy",
        "agent,surface,capabilities,root",
        "",
        "",
    ],
    &[
        "AES201",
        "capabilities",
        "taxonomy,contract,utility",
        "taxonomy,contract(protocol)",
        "surface,agent,capabilities,root",
        "",
        "",
    ],
    &[
        "AES201",
        "agent(orchestrator)",
        "taxonomy,contract(aggregate),contract(protocol),utility",
        "taxonomy,contract(aggregate)",
        "surface,capabilities,root",
        "",
        "",
    ],
    &[
        "AES201",
        "root",
        "taxonomy,contract,capabilities,agent,surface",
        "",
        "",
        "",
        "",
    ],
    &[
        "AES201",
        "surface(command|controller|page)",
        "taxonomy,contract(aggregate),utility",
        "",
        "agent,capabilities,contract(protocol),root",
        "",
        "",
    ],
    &[
        "AES201",
        "surface(hook|store|action|screen|router)",
        "taxonomy",
        "",
        "agent,capabilities,contract(protocol),surface(command|controller|page|entry),root",
        "",
        "",
    ],
    &[
        "AES201",
        "surface(component|view|layout)",
        "taxonomy",
        "",
        "agent,contract,capabilities,surface(command|controller|page|entry|hook|store|action|screen|router),root",
        "",
        "",
    ],
    &[
        "AES202",
        "taxonomy(vo)",
        "taxonomy",
        "",
        "agent,surface,contract,utility,capabilities,root",
        "",
        "",
    ],
    &[
        "AES202",
        "taxonomy(request,response)",
        "taxonomy",
        "",
        "agent,surface,contract,utility,capabilities,root",
        "",
        "",
    ],
    &[
        "AES202",
        "taxonomy(entity,error,event)",
        "taxonomy",
        "taxonomy(vo|constant)",
        "agent,surface,contract,utility,capabilities,root",
        "",
        "",
    ],
    &[
        "AES202",
        "taxonomy(constant)",
        "taxonomy",
        "",
        "agent,surface,contract,utility,capabilities,root",
        "",
        "",
    ],
    &[
        "AES202",
        "utility",
        "taxonomy",
        "",
        "agent,surface,contract,capabilities,root,utility",
        "",
        "",
    ],
    &[
        "AES202",
        "contract(protocol)",
        "taxonomy,contract",
        "taxonomy",
        "agent,surface,capabilities,contract(aggregate),root",
        "",
        "",
    ],
    &[
        "AES202",
        "contract(aggregate)",
        "taxonomy,contract",
        "taxonomy",
        "agent,surface,capabilities,root",
        "",
        "",
    ],
    &[
        "AES202",
        "capabilities",
        "taxonomy,contract,utility",
        "taxonomy,contract(protocol)",
        "surface,agent,capabilities,root",
        "",
        "",
    ],
    &[
        "AES202",
        "agent(orchestrator)",
        "taxonomy,contract(aggregate),contract(protocol),utility",
        "taxonomy,contract(aggregate)",
        "surface,capabilities,root",
        "",
        "",
    ],
    &[
        "AES202",
        "root",
        "taxonomy,contract,capabilities,agent,surface",
        "",
        "",
        "",
        "",
    ],
    &[
        "AES202",
        "surface(command|controller|page)",
        "taxonomy,contract(aggregate),utility",
        "",
        "agent,capabilities,contract(protocol),root",
        "",
        "",
    ],
    &[
        "AES202",
        "surface(hook|store|action|screen|router)",
        "taxonomy",
        "",
        "agent,capabilities,contract(protocol),surface(command|controller|page|entry),root",
        "",
        "",
    ],
    &[
        "AES202",
        "surface(component|view|layout)",
        "taxonomy",
        "",
        "agent,contract,capabilities,surface(command|controller|page|entry|hook|store|action|screen|router),root",
        "",
        "",
    ],
    &["AES203", "taxonomy", "", "", "", "", ""],
    &["AES203", "contract", "", "", "", "", ""],
    &["AES203", "utility", "", "", "", "", ""],
    &["AES203", "capabilities", "", "", "", "", ""],
    &["AES203", "agent", "", "", "", "", ""],
    &["AES203", "surface", "", "", "", "", ""],
    &["AES204", "taxonomy", "", "", "", "", ""],
    &["AES204", "contract", "", "", "", "", ""],
    &["AES204", "utility", "", "", "", "", ""],
    &["AES204", "capabilities", "", "", "", "", ""],
    &["AES204", "agent", "", "", "", "", ""],
    &["AES204", "surface", "", "", "", "", ""],
    &["AES205", "taxonomy", "", "", "", "", ""],
    &["AES205", "contract", "", "", "", "", ""],
    &["AES205", "utility", "", "", "", "", ""],
    &["AES205", "capabilities", "", "", "", "", ""],
    &["AES205", "agent", "", "", "", "", ""],
    &["AES205", "surface", "", "", "", "", ""],
    &["AES301", "taxonomy", "", "", "", "", ""],
    &["AES301", "contract", "", "", "", "", ""],
    &["AES301", "utility", "", "", "", "", ""],
    &["AES301", "capabilities", "", "", "", "", ""],
    &["AES301", "agent", "", "", "", "", ""],
    &["AES301", "surface", "", "", "", "", ""],
    &["AES302", "taxonomy", "", "", "", "", ""],
    &["AES302", "contract", "", "", "", "", ""],
    &["AES302", "utility", "", "", "", "", ""],
    &["AES302", "capabilities", "", "", "", "", ""],
    &["AES302", "agent", "", "", "", "", ""],
    &["AES302", "surface", "", "", "", "", ""],
    &["AES303", "taxonomy", "", "", "", "", ""],
    &["AES303", "contract", "", "", "", "", ""],
    &["AES303", "utility", "", "", "", "", ""],
    &["AES303", "capabilities", "", "", "", "", ""],
    &["AES303", "agent", "", "", "", "", ""],
    &["AES303", "surface", "", "", "", "", ""],
    &[
        "AES304",
        "taxonomy",
        "",
        "",
        "",
        "",
        "#[allow(,unwrap,expect,panic,todo,unimplemented,unreachable,noqa,type: ignore,eslint-disable,// @ts-ignore,// @ts-expect-error,// @ts-nocheck",
    ],
    &[
        "AES304",
        "utility",
        "",
        "",
        "",
        "",
        "#[allow(,unwrap,expect,panic,todo,unimplemented,unreachable,noqa,type: ignore,eslint-disable,// @ts-ignore,// @ts-expect-error,// @ts-nocheck",
    ],
    &[
        "AES304",
        "contract",
        "",
        "",
        "",
        "",
        "#[allow(,unwrap,expect,panic,todo,unimplemented,unreachable,noqa,type: ignore,eslint-disable,// @ts-ignore,// @ts-expect-error,// @ts-nocheck",
    ],
    &[
        "AES304",
        "capabilities",
        "",
        "",
        "",
        "",
        "#[allow(,unwrap,expect,panic,todo,unimplemented,unreachable,noqa,type: ignore,eslint-disable,// @ts-ignore,// @ts-expect-error,// @ts-nocheck",
    ],
    &[
        "AES304",
        "agent",
        "",
        "",
        "",
        "",
        "#[allow(,unwrap,expect,panic,todo,unimplemented,unreachable,noqa,type: ignore,eslint-disable,// @ts-ignore,// @ts-expect-error,// @ts-nocheck",
    ],
    &[
        "AES304",
        "surface",
        "",
        "",
        "",
        "",
        "#[allow(,unwrap,expect,panic,todo,unimplemented,unreachable,noqa,type: ignore,eslint-disable,// @ts-ignore,// @ts-expect-error,// @ts-nocheck",
    ],
    &[
        "AES304",
        "root",
        "",
        "",
        "",
        "",
        "#[allow(,unwrap,expect,panic,todo,unimplemented,unreachable,noqa,type: ignore,eslint-disable,// @ts-ignore,// @ts-expect-error,// @ts-nocheck",
    ],
    &[
        "AES305",
        "taxonomy",
        "",
        "",
        "",
        "capabilities_adapter_health_checker",
        "",
    ],
    &[
        "AES305",
        "contract",
        "",
        "",
        "",
        "capabilities_adapter_health_checker",
        "",
    ],
    &[
        "AES305",
        "utility",
        "",
        "",
        "",
        "capabilities_adapter_health_checker",
        "",
    ],
    &[
        "AES305",
        "capabilities",
        "",
        "",
        "",
        "capabilities_adapter_health_checker",
        "",
    ],
    &[
        "AES305",
        "agent",
        "",
        "",
        "",
        "capabilities_adapter_health_checker",
        "",
    ],
    &[
        "AES305",
        "surface",
        "",
        "",
        "",
        "capabilities_adapter_health_checker",
        "",
    ],
    &["AES401", "taxonomy(constant)", "", "", "", "", ""],
    &["AES401", "taxonomy(entity,error,event)", "", "", "", "", ""],
    &["AES401", "taxonomy(vo)", "", "", "", "", ""],
    &["AES401", "taxonomy(request,response)", "", "", "", "", ""],
    &["AES402", "contract(aggregate)", "", "", "", "", ""],
    &["AES402", "contract(protocol)", "", "", "", "", ""],
    &["AES403", "capabilities", "", "", "", "", ""],
    &["AES404", "utility", "", "", "", "", ""],
    &["AES405", "agent(orchestrator)", "", "", "", "", ""],
    &[
        "AES406",
        "surface(command|controller|page|router|entry)",
        "",
        "",
        "",
        "",
        "",
    ],
    &[
        "AES406",
        "surface(hook|store|action|screen)",
        "",
        "",
        "",
        "",
        "",
    ],
    &[
        "AES406",
        "surface(component|view|layout)",
        "",
        "",
        "",
        "",
        "",
    ],
    &["AES501", "taxonomy", "", "", "", "", ""],
    &["AES502", "contract", "", "", "", "", ""],
    &["AES503", "capabilities", "", "", "", "", ""],
    &["AES504", "utility", "", "", "", "", ""],
    &["AES505", "agent", "", "", "", "", ""],
    &["AES506", "surface", "", "", "", "", ""],
    &["AES601", "", "", "", "", "", ""],
    &["AES602", "", "", "", "", "", ""],
    &["AES603", "", "", "", "", "", ""],
    &["AES604", "", "", "", "", "", ""],
    &["AES605", "", "", "", "", "", ""],
    &["AES701", "", "", "", "", "", ""],
    &["AES702", "", "", "", "", "", ""],
    &["AES703", "", "", "", "", "", ""],
    &["AES704", "", "", "", "", "", ""],
];

// ─── Public API ────────────────────────────────────────────────────────────

/// The AES architecture as the tool enforces it, independent of user config.
pub fn hardcoded_default_architecture() -> ArchitectureConfig {
    ArchitectureConfig {
        enabled: BooleanVO::new(true),
        layers: hardcoded_layers(),
        rules: hardcoded_rules(),
        naming: NamingConfig::new(Count::new(3)),
        ignored_paths: Default::default(),
        mandatory_class_definition: BooleanVO::new(false),
    }
}
