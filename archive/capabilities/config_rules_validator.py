"""config_rules_validator — Capability for validating and checking project configuration rules."""

from ..taxonomy import (
    AdapterName,
    AdapterStatus,
    BooleanVO,
    ProjectConfig,
    SuccessStatus,
)

from ..contract import IConfigRulesProtocol


class ConfigRulesValidator(IConfigRulesProtocol):
    """Business logic for interpreting and validating project configuration."""

    def __init__(self, project_config: ProjectConfig):
        self._config = project_config

    def is_adapter_enabled(self, adapter_name: AdapterName) -> SuccessStatus:
        """Determines if a specific adapter should run based on configuration rules."""
        for adapter in self._config.adapters:
            if adapter.name == adapter_name:
                return SuccessStatus(
                    value=BooleanVO(value=adapter.status == AdapterStatus.ENABLED)
                )

        # Default policy: enabled if not explicitly mentioned
        return SuccessStatus(value=BooleanVO(value=True))

    def validate_thresholds(self) -> SuccessStatus:
        """Validates that scoring thresholds are sane."""
        t = self._config.thresholds
        # Score must be 0-100
        if not (0 <= t.score.value <= 100):
            return SuccessStatus(value=BooleanVO(value=False))
        # Complexity and line limits must be positive
        if t.complexity.value <= 0 or t.max_file_lines.value <= 0:
            return SuccessStatus(value=BooleanVO(value=False))
        return SuccessStatus(value=BooleanVO(value=True))
