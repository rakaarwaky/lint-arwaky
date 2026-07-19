from shared.orphan_detector.contract_orphan_protocol import IOrphanAnalyzerProtocol
from shared.orphan_detector.contract_orphan_aggregate import (
    IOrphanOrchestratorAggregate,
)
from shared.code_analysis.taxonomy_result_vo import LintResult


# ─── Block 1: Class Definition & Constructor ──────────────
class OrphanOrchestrator(IOrphanOrchestratorAggregate):
    def __init__(self, analyzer: IOrphanAnalyzerProtocol):
        self._analyzer = analyzer

    # ─── Block 2: Public Contract (domain aggregate ONLY) ──
    def execute(self, request: ScanRequest) -> list[LintResult]:
        violations: list[LintResult] = []
        for file in request.files():
            try:
                result = self._analyzer.analyze(file)
                violations.extend(result.into_violations())
            except Exception as e:
                violations.append(LintResult.from_analysis_error(file, e))
        return violations

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "OrphanOrchestrator()"

    @classmethod
    def create_default(cls) -> "OrphanOrchestrator":
        return cls(analyzer=CapabilitiesOrphanAnalyzer())
