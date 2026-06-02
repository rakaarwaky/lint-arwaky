"""severity_vo — Severity level value objects."""

from enum import Enum


class Severity(str, Enum):
    """Lint finding impact level."""

    INFO = "info"
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"

    @property
    def score_impact(self) -> float:
        return _SEVERITY_IMPACT[self]


_SEVERITY_IMPACT = {
    Severity.INFO: 0,
    Severity.LOW: 1,
    Severity.MEDIUM: 2,
    Severity.HIGH: 3,
    Severity.CRITICAL: 5,
}
