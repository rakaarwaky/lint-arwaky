"""lint_reporting_protocol — Protocols for report formatting and output.

Defines interfaces for transforming GovernanceReports into various output formats.
"""

from abc import ABC, abstractmethod

from ..taxonomy import FileFormat, GovernanceReport, LogOutput, ResponseData


class ILintReportFormatterProtocol(ABC):
    """Protocol for formatting lint reports into various representations."""

    @abstractmethod
    def format(self, report: GovernanceReport) -> LogOutput:
        """Format the report as a plain text representation."""
        ...

    @abstractmethod
    def get_formatted_payload(
        self, report: GovernanceReport, output_format: FileFormat
    ) -> ResponseData:
        """Return formatted data for surface consumption."""
        ...

    @abstractmethod
    def to_sarif(self, report: GovernanceReport) -> LogOutput:
        """Convert report to SARIF format."""
        ...

    @abstractmethod
    def to_junit(self, report: GovernanceReport) -> LogOutput:
        """Convert report to JUnit XML format."""
        ...
