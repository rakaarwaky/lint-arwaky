"""metric_analyzer_processor — Capability for analyzing code metrics (complexity, size, trends)."""

from ..taxonomy import (
    AdapterName,
    ColumnNumber,
    Count,
    ErrorCode,
    FilePath,
    LineNumber,
    LintMessage,
    LintResult,
    LintResultList,
    ResponseDataList,
    Score,
    Severity,
)


from ..contract import IMetricAnalyzerProtocol


class MetricAnalyzerProcessor(IMetricAnalyzerProtocol):
    """Business logic for interpreting technical metrics into linting results."""

    def analyze_complexity(
        self, raw_data: ResponseDataList, threshold: Count
    ) -> LintResultList:
        """Interprets raw complexity data into severity-rated results."""
        results = []
        limit = threshold.value
        for issue in raw_data.values:
            data = issue.value
            complexity = data.get("complexity", 0)
            if complexity > limit:
                results.append(
                    LintResult(
                        file=FilePath(value=data["filename"]),
                        line=LineNumber(value=data["lineno"]),
                        column=ColumnNumber(value=data.get("col_offset", 0)),
                        code=ErrorCode(code="complexity"),
                        message=LintMessage(
                            value=f"High complexity ({complexity}) in {data['name']}"
                        ),
                        source=AdapterName(value="radon-processor"),
                        severity=self._get_complexity_severity(Count(value=complexity)),
                    )
                )
        return LintResultList(values=results)

    def analyze_file_size(
        self, file_path: FilePath, line_count: Count, limit: Count
    ) -> LintResultList:
        """Checks if a file exceeds size limits (SRP violation rule)."""
        count = line_count.value
        threshold = limit.value
        if count > threshold:
            return LintResultList(
                values=[
                    LintResult(
                        file=file_path,
                        line=LineNumber(value=1),
                        column=ColumnNumber(value=0),
                        code=ErrorCode(code="SIZE001"),
                        message=LintMessage(
                            value=f"File exceeds {threshold} lines ({count}); potential duplication or SRP violation."
                        ),
                        source=AdapterName(value="metric-processor"),
                        severity=Severity.LOW,
                    )
                ]
            )
        return LintResultList()

    def analyze_quality_trend(
        self, current_score: Score, previous_score: Score
    ) -> LintResultList:
        """Analyzes if code quality is degrading over time."""
        if current_score.value < previous_score.value:
            return LintResultList(
                values=[
                    LintResult(
                        file=FilePath(value="project"),
                        line=LineNumber(value=1),
                        column=ColumnNumber(value=0),
                        code=ErrorCode(code="TREND001"),
                        message=LintMessage(
                            value=f"Quality trend is negative: {previous_score.value} -> {current_score.value}"
                        ),
                        source=AdapterName(value="trend-processor"),
                        severity=Severity.MEDIUM,
                    )
                ]
            )
        return LintResultList()

    def _get_complexity_severity(self, complexity: Count) -> Severity:
        val = complexity.value
        if val > 20:
            return Severity.HIGH
        if val > 10:
            return Severity.MEDIUM
        return Severity.LOW
