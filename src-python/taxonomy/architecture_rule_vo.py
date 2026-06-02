"""architecture_rule_vo — Value objects for architecture rules."""

from pydantic import BaseModel, ConfigDict, Field
from .path_collection_vo import PatternList
from .symbol_collection_vo import PrimitiveTypeList
from .message_status_vo import Count
from .error_code_vo import ErrorCode
from .log_suggestion_vo import BooleanVO, DescriptionVO
from .error_value_vo import ErrorMessage
from .layer_content_vo import LayerNameVO
from .file_suffix_vo import SuffixVO, SuffixPolicyVO


class CustomMessageVO(BaseModel):
    """A pattern-to-message mapping for custom violation messages."""

    model_config = ConfigDict(frozen=True)
    pattern: str  # Regex pattern
    message: ErrorMessage


class MandatoryImportRuleVO(BaseModel):
    """Defines mandatory imports for specific file suffixes."""

    model_config = ConfigDict(frozen=True)
    suffix: SuffixVO
    imports: PatternList


class ArchitectureRule(BaseModel):
    """Rule definition for a layer or set of files."""

    model_config = ConfigDict(arbitrary_types_allowed=True)

    name: DescriptionVO | None = None
    description: DescriptionVO | None = None
    rule_type: ErrorCode = Field(default_factory=lambda: ErrorCode(code="internal"))
    scope: LayerNameVO | PatternList | str = Field(
        default_factory=lambda: LayerNameVO(value="global")
    )

    # Rule parameters
    word_count: Count | None = None
    exceptions: PatternList = Field(
        default_factory=PatternList
    )  # Filenames exempt from this rule
    allowed_import: PatternList = Field(default_factory=PatternList)
    forbidden_import: PatternList = Field(default_factory=PatternList)
    mandatory_import: PatternList = Field(default_factory=PatternList)
    mandatory_import_violation_message: ErrorMessage | None = None
    forbidden_import_violation_message: ErrorMessage | None = None
    suffix_policy: SuffixPolicyVO | None = None
    allowed_suffix: PatternList = Field(default_factory=PatternList)
    forbidden_suffix: PatternList = Field(default_factory=PatternList)

    # Integrity parameters (flattened)
    no_primitives: BooleanVO | PrimitiveTypeList | None = None
    mandatory_imports: list[MandatoryImportRuleVO] | None = None
    barrel_completeness: BooleanVO | None = None
    min_lines: Count | None = None
    max_lines: Count | None = None
    word_count_violation_message: ErrorMessage | None = None
    suffix_violation_message: ErrorMessage | None = None
    no_primitives_violation_message: ErrorMessage | None = None
    min_lines_violation_message: ErrorMessage | None = None
    max_lines_violation_message: ErrorMessage | None = None
    barrel_completeness_violation_message: ErrorMessage | None = None
    forbidden_bypass: PatternList = Field(default_factory=PatternList)
    forbidden_bypass_violation_message: ErrorMessage | None = None
    forbidden_bypass_custom_messages: list[CustomMessageVO] | None = None
    forbid_internal_all: BooleanVO | None = None
    forbid_internal_all_violation_message: ErrorMessage | None = None
    mandatory_class_definition: BooleanVO | None = None
    mandatory_class_definition_violation_message: ErrorMessage | None = None
    dead_inheritance_bypass: BooleanVO | None = None
    dead_inheritance_bypass_violation_message: ErrorMessage | None = None
    dead_inheritance_bypass_custom_messages: list[CustomMessageVO] | None = None
    check_orphan: BooleanVO | None = None
    orphan_entry_points: PatternList = Field(default_factory=PatternList)
    orphan_violation_message: ErrorMessage | None = None
    check_unused_mandatory_imports: BooleanVO | None = None
    check_unused_mandatory_imports_violation_message: ErrorMessage | None = None
    forbidden_inheritance: PatternList = Field(default_factory=PatternList)
    forbidden_inheritance_violation_message: ErrorMessage | None = None

    # Role Integrity Parameters
    no_domain_logic: BooleanVO | None = None
    no_domain_logic_violation_message: ErrorMessage | None = None
    must_implement_service_container_aggregate: BooleanVO | None = None
    must_implement_service_container_aggregate_violation_message: ErrorMessage | None = None
    lazy_eager_initialization_only: BooleanVO | None = None
    lazy_eager_initialization_only_violation_message: ErrorMessage | None = None
    stateless_execution: BooleanVO | None = None
    stateless_execution_violation_message: ErrorMessage | None = None
    single_execution_goal: BooleanVO | None = None
    single_execution_goal_violation_message: ErrorMessage | None = None
    high_level_policy_only: BooleanVO | None = None
    high_level_policy_only_violation_message: ErrorMessage | None = None
    coordinates_multiple_orchestrators: BooleanVO | None = None
    coordinates_multiple_orchestrators_violation_message: ErrorMessage | None = None
    crud_only: BooleanVO | None = None
    crud_only_violation_message: ErrorMessage | None = None
    no_decision_logic: BooleanVO | None = None
    no_decision_logic_violation_message: ErrorMessage | None = None
    thread_async_safe: BooleanVO | None = None
    thread_async_safe_violation_message: ErrorMessage | None = None
    no_domain_data_storage: BooleanVO | None = None
    no_domain_data_storage_violation_message: ErrorMessage | None = None
    owns_system_health_transitions: BooleanVO | None = None
    owns_system_health_transitions_violation_message: ErrorMessage | None = None
    lifecycle_tracking_only: BooleanVO | None = None
    lifecycle_tracking_only_violation_message: ErrorMessage | None = None
    forbid_any_type: BooleanVO | None = None
    forbid_any_type_violation_message: ErrorMessage | None = None


class LegacyLayerRule(BaseModel):
    """Represents a legacy architecture rule (from -> to description)."""

    model_config = ConfigDict(frozen=True)
    source_layer: LayerNameVO
    forbidden_target: LayerNameVO
    description: ErrorMessage = Field(default_factory=lambda: ErrorMessage(value=""))


class LegacyLayerRuleList(BaseModel):
    """Collection of legacy layer rules."""

    model_config = ConfigDict(frozen=True)
    values: list[LegacyLayerRule] = Field(default_factory=list)

    def __iter__(self):
        return iter(self.values)

    def __len__(self) -> int:
        return len(self.values)
