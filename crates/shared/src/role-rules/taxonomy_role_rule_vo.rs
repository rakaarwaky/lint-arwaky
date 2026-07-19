// PURPOSE: RoleRuleVO — value object containing role compliance rule definitions
use crate::common::taxonomy_common_vo::{bool, PatternList};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RoleRuleVO {
    #[serde(default)]
    pub no_domain_logic: bool,
    #[serde(default)]
    pub must_implement_service_container_aggregate: bool,
    #[serde(default)]
    pub lazy_eager_initialization_only: bool,
    #[serde(default)]
    pub stateless_execution: bool,
    #[serde(default)]
    pub single_execution_goal: bool,
    #[serde(default)]
    pub high_level_policy_only: bool,
    #[serde(default)]
    pub coordinates_multiple_orchestrators: bool,
    #[serde(default)]
    pub crud_only: bool,
    #[serde(default)]
    pub no_decision_logic: bool,
    #[serde(default)]
    pub thread_async_safe: bool,
    #[serde(default)]
    pub no_domain_data_storage: bool,
    #[serde(default)]
    pub owns_system_health_transitions: bool,
    #[serde(default)]
    pub lifecycle_tracking_only: bool,
    #[serde(default)]
    pub no_primitives: bool,
    #[serde(default)]
    pub forbidden_inheritance: PatternList,
}
