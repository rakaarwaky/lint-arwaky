from pydantic import BaseModel, ConfigDict, Field, model_validator
from .path_collection_vo import PatternList
from .symbol_collection_vo import PrimitiveTypeList
from .message_status_vo import Count
from .log_suggestion_vo import BooleanVO
from .error_value_vo import ErrorMessage
from .file_path_vo import DirectoryPath
from .architecture_rule_vo import CustomMessageVO, MandatoryImportRuleVO
from .file_suffix_vo import SuffixPolicyVO
from .layer_content_vo import LayerNameVO


class NamingConfig(BaseModel):
    """Naming convention settings."""

    model_config = ConfigDict(frozen=True)
    word_count: Count = Field(default_factory=lambda: Count(value=3))
    word_count_violation_message: ErrorMessage | None = None


class LayerDefinition(BaseModel):
    """Unified definition of a layer. Computed from layers + rules."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def parse_suffix_list(cls, data: object) -> object:
        if isinstance(data, dict) and "suffix" in data:
            suffixes = data.pop("suffix")
            if isinstance(suffixes, list):
                for item in suffixes:
                    if isinstance(item, dict):
                        if "strict" in item:
                            data["suffix_policy"] = "strict"
                            data["allowed_suffix"] = item["strict"]
                        elif "flexible" in item:
                            data["suffix_policy"] = "flexible"
                            data["allowed_suffix"] = item["flexible"]
                        elif "forbidden" in item:
                            data["forbidden_suffix"] = item["forbidden"]
        return data

    path: DirectoryPath

    @property
    def path_str(self) -> str:
        """Returns the layer path as a normalized string."""
        return str(self.path)

    suffix_policy: SuffixPolicyVO = Field(
        default_factory=lambda: SuffixPolicyVO(value="flexible")
    )
    allowed_suffix: PatternList = Field(default_factory=PatternList)
    forbidden_suffix: PatternList = Field(default_factory=PatternList)
    allowed_import: PatternList = Field(default_factory=PatternList)
    forbidden_import: PatternList = Field(default_factory=PatternList)
    mandatory_import: PatternList = Field(default_factory=PatternList)
    mandatory_import_violation_message: ErrorMessage | None = None
    forbidden_import_violation_message: ErrorMessage | None = None
    word_count: Count | None = None
    exceptions: PatternList = Field(default_factory=PatternList)
    recursive: BooleanVO = Field(default_factory=lambda: BooleanVO(value=True))
    no_primitives: BooleanVO | PrimitiveTypeList = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    mandatory_imports: list[MandatoryImportRuleVO] = Field(default_factory=list)
    barrel_completeness: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    min_lines: Count | None = None
    max_lines: Count | None = None
    word_count_violation_message: ErrorMessage | None = None
    suffix_violation_message: ErrorMessage | None = None
    no_primitives_violation_message: ErrorMessage | None = None
    min_lines_violation_message: ErrorMessage | None = None
    max_lines_violation_message: ErrorMessage | None = None
    barrel_completeness_violation_message: ErrorMessage | None = None
    forbid_internal_all: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    forbid_internal_all_violation_message: ErrorMessage | None = None
    forbidden_bypass: PatternList = Field(
        default_factory=lambda: PatternList(
            values=[
                "#" + " noqa",
                "#" + " type: ignore",
                "#" + " skip",
                "#" + " disable",
                "#" + " pylint: disable",
                "#" + " pylint:disable",
            ]
        ),
        description="Bypass patterns forbidden in comments. Always enforced to prevent cheating.",
    )
    forbidden_bypass_violation_message: ErrorMessage | None = None
    forbidden_bypass_custom_messages: list[CustomMessageVO] = Field(
        default_factory=list
    )
    mandatory_class_definition: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    mandatory_class_definition_violation_message: ErrorMessage | None = None
    dead_inheritance_bypass: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=True)
    )
    dead_inheritance_bypass_violation_message: ErrorMessage | None = None
    dead_inheritance_bypass_custom_messages: list[CustomMessageVO] = Field(
        default_factory=list
    )
    check_orphan: BooleanVO = Field(default_factory=lambda: BooleanVO(value=False))
    orphan_entry_points: PatternList = Field(default_factory=PatternList)
    orphan_violation_message: ErrorMessage | None = None
    check_unused_mandatory_imports: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=True)
    )
    check_unused_mandatory_imports_violation_message: ErrorMessage | None = None
    forbidden_inheritance: PatternList = Field(default_factory=PatternList)
    forbidden_inheritance_violation_message: ErrorMessage | None = None
    no_domain_logic: BooleanVO = Field(default_factory=lambda: BooleanVO(value=False))
    no_domain_logic_violation_message: ErrorMessage | None = None
    must_implement_service_container_aggregate: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    must_implement_service_container_aggregate_violation_message: ErrorMessage | None = None
    lazy_eager_initialization_only: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    lazy_eager_initialization_only_violation_message: ErrorMessage | None = None
    stateless_execution: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    stateless_execution_violation_message: ErrorMessage | None = None
    single_execution_goal: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    single_execution_goal_violation_message: ErrorMessage | None = None
    high_level_policy_only: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    high_level_policy_only_violation_message: ErrorMessage | None = None
    coordinates_multiple_orchestrators: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    coordinates_multiple_orchestrators_violation_message: ErrorMessage | None = None
    crud_only: BooleanVO = Field(default_factory=lambda: BooleanVO(value=False))
    crud_only_violation_message: ErrorMessage | None = None
    no_decision_logic: BooleanVO = Field(default_factory=lambda: BooleanVO(value=False))
    no_decision_logic_violation_message: ErrorMessage | None = None
    thread_async_safe: BooleanVO = Field(default_factory=lambda: BooleanVO(value=False))
    thread_async_safe_violation_message: ErrorMessage | None = None
    no_domain_data_storage: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    no_domain_data_storage_violation_message: ErrorMessage | None = None
    owns_system_health_transitions: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    owns_system_health_transitions_violation_message: ErrorMessage | None = None
    lifecycle_tracking_only: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    lifecycle_tracking_only_violation_message: ErrorMessage | None = None
    forbid_any_type: BooleanVO = Field(default_factory=lambda: BooleanVO(value=False))
    forbid_any_type_violation_message: ErrorMessage | None = None


class LayerMapVO(BaseModel):
    """Collection of layer definitions."""

    model_config = ConfigDict(frozen=True)
    values: dict[LayerNameVO, LayerDefinition] = Field(default_factory=dict)
