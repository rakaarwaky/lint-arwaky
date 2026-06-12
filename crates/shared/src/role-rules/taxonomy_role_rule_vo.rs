// PURPOSE: RoleRuleVO — value object containing role compliance rule definitions
use crate::common::taxonomy_common_vo::BooleanVO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RoleRuleVO {
    #[serde(default)]
    pub no_domain_logic: BooleanVO,
    #[serde(default)]
    pub must_implement_service_container_aggregate: BooleanVO,
    #[serde(default)]
    pub lazy_eager_initialization_only: BooleanVO,
    #[serde(default)]
    pub stateless_execution: BooleanVO,
    #[serde(default)]
    pub single_execution_goal: BooleanVO,
    #[serde(default)]
    pub high_level_policy_only: BooleanVO,
    #[serde(default)]
    pub coordinates_multiple_orchestrators: BooleanVO,
    #[serde(default)]
    pub crud_only: BooleanVO,
    #[serde(default)]
    pub no_decision_logic: BooleanVO,
    #[serde(default)]
    pub thread_async_safe: BooleanVO,
    #[serde(default)]
    pub no_domain_data_storage: BooleanVO,
    #[serde(default)]
    pub owns_system_health_transitions: BooleanVO,
    #[serde(default)]
    pub lifecycle_tracking_only: BooleanVO,
    #[serde(default)]
    pub no_primitives: BooleanVO,
}
