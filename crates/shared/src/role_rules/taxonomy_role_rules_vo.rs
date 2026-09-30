// PURPOSE: Role-rules value objects — layer name helpers and role violation payloads (AES401-406).
use std::collections::HashSet;

use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::role_rules::taxonomy_role_rules_constant::LAYER_AGENT;
use crate::role_rules::taxonomy_role_rules_constant::LAYER_CAPABILITIES;
use crate::role_rules::taxonomy_role_rules_constant::LAYER_CONTRACT;
use crate::role_rules::taxonomy_role_rules_constant::LAYER_GLOBAL;
use crate::role_rules::taxonomy_role_rules_constant::LAYER_ROOT;
use crate::role_rules::taxonomy_role_rules_constant::LAYER_SURFACES;
use crate::role_rules::taxonomy_role_rules_constant::LAYER_TAXONOMY;
use crate::role_rules::taxonomy_role_rules_constant::LAYER_UTILITY;

// ─── Layer name helpers ──────────────────────────────────────────────

/// Value object holding the set of core layer names.
pub struct LayerNames {}

pub fn layer_agent() -> LayerNameVO {
    LayerNameVO::new(LAYER_AGENT)
}
pub fn layer_capabilities() -> LayerNameVO {
    LayerNameVO::new(LAYER_CAPABILITIES)
}
pub fn layer_taxonomy() -> LayerNameVO {
    LayerNameVO::new(LAYER_TAXONOMY)
}
pub fn layer_contract() -> LayerNameVO {
    LayerNameVO::new(LAYER_CONTRACT)
}
pub fn layer_utility() -> LayerNameVO {
    LayerNameVO::new(LAYER_UTILITY)
}
pub fn layer_surfaces() -> LayerNameVO {
    LayerNameVO::new(LAYER_SURFACES)
}
pub fn layer_root() -> LayerNameVO {
    LayerNameVO::new(LAYER_ROOT)
}
pub fn layer_global() -> LayerNameVO {
    LayerNameVO::new(LAYER_GLOBAL)
}

pub fn all_core_layers() -> Vec<LayerNameVO> {
    vec![
        layer_agent(),
        layer_capabilities(),
        layer_taxonomy(),
        layer_contract(),
        layer_utility(),
        layer_surfaces(),
        layer_root(),
    ]
}

pub fn core_layer_names() -> HashSet<String> {
    all_core_layers().iter().map(|l| l.value.clone()).collect()
}

// ─── Surface tier helpers (AES406) ─────────────────────────────────────

/// Surface tier, resolved from the filename suffix.
///
/// `_router` is utility, matching `crates/shared/skills/aes-surface/SKILL.md` line 72.
/// `_entry` is not a surface suffix at all — entry points are root, and a
/// `surface_*_entry` file is caught by AES102 before AES406 ever sees it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceTier {
    Smart,
    Utility,
    Passive,
}

/// Smart tier suffixes: `_command` / `_controller` / `_page`.
const SMART_SUFFIXES: [&str; 3] = ["_command", "_controller", "_page"];

/// Utility tier suffixes: `_hook` / `_store` / `_action` / `_screen` / `_router`.
const UTILITY_SUFFIXES: [&str; 5] = ["_hook", "_store", "_action", "_screen", "_router"];

/// Resolve the tier from a file stem (filename without extension).
///
/// A stem matching no tier suffix is passive: the remaining legal surface
/// suffixes are `_component` / `_view` / `_layout`, and anything AES102 does not
/// reject is treated the same way.
pub fn classify_surface_tier(stem: &str) -> SurfaceTier {
    if SMART_SUFFIXES.iter().any(|s| stem.ends_with(s)) {
        SurfaceTier::Smart
    } else if UTILITY_SUFFIXES.iter().any(|s| stem.ends_with(s)) {
        SurfaceTier::Utility
    } else {
        SurfaceTier::Passive
    }
}

// ─── Role violation payloads ─────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum AesRoleViolation {
    ConstantPurity {
        reason: Option<LintMessage>,
    },
    PrimitiveUsage {
        primitive: SymbolName,
        reason: Option<LintMessage>,
    },
    ContractPrimitive {
        reason: Option<LintMessage>,
    },
    CapabilityNoProtocol {
        reason: Option<LintMessage>,
    },
    CapabilityNoImplementor {
        reason: Option<LintMessage>,
    },
    CapabilityTooManyTypes {
        count: usize,
        reason: Option<LintMessage>,
    },
    SingleBottleneck {
        reason: Option<LintMessage>,
    },
    UtilityRole {
        reason: Option<LintMessage>,
    },
    AgentNoImplementor {
        reason: Option<LintMessage>,
    },
    AgentTooManyTypes {
        count: usize,
        names: Vec<SymbolName>,
        reason: Option<LintMessage>,
    },
    StatelessExecution {
        reason: Option<LintMessage>,
    },
    HighLevelPolicy {
        reason: Option<LintMessage>,
    },
    CoordinatesMultiple {
        reason: Option<LintMessage>,
    },
    NoDomainLogic {
        reason: Option<LintMessage>,
    },
    LazyEagerInit {
        reason: Option<LintMessage>,
    },
    MustImplementContract {
        reason: Option<LintMessage>,
    },
    AgentFileSizeLimit {
        max_lines: usize,
    },
    PassiveViolation {
        reason: Option<LintMessage>,
    },
    SurfaceRoleViolation {
        reason: Option<LintMessage>,
    },
}
