# BAD: Taxonomy importing layer code (AES201)
from capabilities_orphan_analyzer import OrphanAnalyzer  # BAD


@dataclass(frozen=True)
class OrphanResult:
    is_orphan: bool
    reason: str
