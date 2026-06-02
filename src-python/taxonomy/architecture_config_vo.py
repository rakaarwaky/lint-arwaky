from pydantic import BaseModel, ConfigDict, Field, model_validator
from .layer_content_vo import LayerNameVO
from .layer_definition_vo import LayerDefinition, NamingConfig
from .architecture_rule_vo import ArchitectureRule, LegacyLayerRuleList
from .log_suggestion_vo import BooleanVO
from .path_collection_vo import FilePathList
from .error_value_vo import ErrorMessage


class ArchitectureConfig(BaseModel):
    """Configuration for architectural rules."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def flatten_rules(cls, data: object) -> object:
        if isinstance(data, dict) and "rules" in data:
            rules_raw = data["rules"]
            if isinstance(rules_raw, dict):
                flattened = []
                for group in ["global", "internal", "external"]:
                    if group in rules_raw:
                        flattened.extend(rules_raw[group])
                for key, val in rules_raw.items():
                    if key not in ["global", "internal", "external"] and isinstance(
                        val, list
                    ):
                        flattened.extend(val)
                data["rules"] = flattened
        return data

    enabled: BooleanVO = Field(default_factory=lambda: BooleanVO(value=False))
    layers: dict[LayerNameVO, LayerDefinition] = Field(default_factory=dict)
    rules: list[ArchitectureRule] = Field(default_factory=list)
    governance_rules: LegacyLayerRuleList = Field(default_factory=LegacyLayerRuleList)
    naming: NamingConfig = NamingConfig()
    ignored_paths: FilePathList = Field(default_factory=FilePathList)
    mandatory_import_violation_message: ErrorMessage | None = None
    mandatory_class_definition: BooleanVO = Field(
        default_factory=lambda: BooleanVO(value=False)
    )
    mandatory_class_definition_violation_message: ErrorMessage | None = None
