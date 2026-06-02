from pydantic import BaseModel, ConfigDict, Field
from .lint_result_vo import LintResult, LintResultList
from .score_format_vo import Score
from .message_status_vo import ComplianceStatus, Count
from .lint_severity_vo import Severity
from .adapter_name_vo import AdapterName


class GovernanceReport(BaseModel):
    """Aggregated lint scan results entity."""

    model_config = ConfigDict(frozen=False)
    results: LintResultList = Field(default_factory=LintResultList)
    score: Score = Field(default_factory=lambda: Score(value=100.0))
    is_passing: ComplianceStatus = Field(
        default_factory=lambda: ComplianceStatus(value=True)
    )

    def add_result(self, result: LintResult):
        """Add a finding and update score."""
        # Since LintResultList is now a VO, we handle the mutation by creating a new
        # list if needed,
        # but for performance in entities, we can keep the internal list mutable if
        # encapsulated.
        # However, to be a true VO, the list should be replaced.
        new_values = list(self.results.values)
        new_values.append(result)
        self.results = LintResultList(values=new_values)
        self.score = self.score.deduct(result.severity)

    def update_compliance(self, threshold: Score):
        """Update compliance status based on score threshold and critical findings."""
        is_p = self.score.is_passing(threshold)
        has_critical = any(r.severity == Severity.CRITICAL for r in self.results)
        self.is_passing = ComplianceStatus(value=is_p and not has_critical)

    def results_by_source(self, source: AdapterName) -> LintResultList:
        """Filter results by adapter source."""
        source_str = str(source)
        return LintResultList(
            values=[r for r in self.results if str(r.source or "") == source_str]
        )

    @property
    def violation_count(self) -> Count:
        return Count(
            value=len([r for r in self.results if r.severity.score_impact > 0])
        )
