# BAD: Data class defined in capabilities layer (AES201)
from dataclasses import dataclass


@dataclass
class OrphanResult:
    is_orphan: bool
    reason: str


class CapabilitiesOrphanAnalyzer:
    def analyze(self) -> OrphanResult:
        return OrphanResult(is_orphan=True, reason="")
