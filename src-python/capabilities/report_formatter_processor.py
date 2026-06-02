"""report_formatter_processor — Capability for formatting reports (SARIF, JUnit)."""

from typing import Any
from ..taxonomy import (
    FileFormat,
    FORMAT_TEXT,
    FORMAT_JSON,
    FORMAT_SARIF,
    FORMAT_JUNIT,
    LogOutput,
    GovernanceReport,
    ResponseData,
)

import json
import html


from ..contract import ILintReportFormatterProtocol


class ReportFormatterProcessor(ILintReportFormatterProtocol):
    """Business logic for transforming GovernanceReports into standard formats."""

    def format(self, report: GovernanceReport) -> LogOutput:
        """Standard entry point for formatting as JSON/Dict."""
        return LogOutput(value=json.dumps(self.report_to_dict(report), indent=2))

    def get_formatted_payload(
        self, report: GovernanceReport, output_format: FileFormat = FORMAT_TEXT
    ) -> ResponseData:
        """Unified entry point for getting a formatted payload for the Surface layer."""
        data = self.report_to_dict(report)

        if output_format == FORMAT_SARIF:
            return ResponseData(
                value={"format": "sarif", "data": self.to_sarif(report).value}
            )
        if output_format == FORMAT_JUNIT:
            return ResponseData(
                value={"format": "junit", "data": self.to_junit(report).value}
            )
        if output_format == FORMAT_JSON:
            return ResponseData(value={"format": "json", "data": data})

        # Default to text/dict representation
        return ResponseData(value={"format": "text", "data": data})

    def _get_severity(self, sev: str) -> str:
        mapping = {"high": "error", "medium": "warning", "low": "note"}
        return mapping.get(sev.lower(), "warning")

    def to_sarif(self, report: GovernanceReport) -> LogOutput:
        """Transforms a GovernanceReport into a SARIF formatted string."""
        results_list = []

        # Convert report to internal dict representation first (logic moved from
        # adapter)
        report_data = self.report_to_dict(report)

        for adapter_name, adapter_results in report_data.items():
            if adapter_name in ["score", "is_passing", "summary"]:
                continue

            if not isinstance(adapter_results, list):
                continue

            for error in adapter_results:
                results_list.append(
                    {
                        "ruleId": f"{adapter_name}/{error.get('code', 'unknown')}",
                        "level": self._get_severity(error.get("severity", "medium")),
                        "message": {"text": error.get("message", "")},
                        "locations": [
                            {
                                "physicalLocation": {
                                    "artifactLocation": {
                                        "uri": error.get("file", "unknown")
                                    },
                                    "region": {
                                        "startLine": error.get("line", 1),
                                        "startColumn": error.get("column", 1),
                                    },
                                }
                            }
                        ],
                    }
                )

        sarif = {
            "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
            "version": "2.1.0",
            "runs": [
                {"tool": {"driver": {"name": "Auto-Linter"}}, "results": results_list}
            ],
        }
        return LogOutput(value=json.dumps(sarif, indent=2))

    def to_junit(self, report: GovernanceReport) -> LogOutput:
        """Transforms a GovernanceReport into a JUnit XML formatted string."""
        report_data = self.report_to_dict(report)
        xml = ['<?xml version="1.0" encoding="UTF-8"?>']

        total_tests = 0
        total_failures = 0
        testsuites = []

        for adapter_name, adapter_results in report_data.items():
            if adapter_name in ["score", "is_passing", "summary"]:
                continue

            if not isinstance(adapter_results, list):
                continue

            failure_count = len(adapter_results)
            testsuite_lines = []
            testsuite_lines.append(
                f'  <testsuite name="{adapter_name}" tests="{max(1, failure_count)}" failures="{failure_count}">'
            )

            if failure_count == 0:
                testsuite_lines.append(
                    f'    <testcase name="lint_{adapter_name}" classname="{adapter_name}"/>'
                )
                total_tests += 1
            else:
                for i, error in enumerate(adapter_results):
                    msg = html.escape(error.get("message", ""), quote=True)
                    testsuite_lines.append(
                        f'    <testcase name="lint_{adapter_name}_{i}" classname="{adapter_name}">'
                    )
                    testsuite_lines.append(
                        f'      <failure message="Linting failed">{msg}</failure>'
                    )
                    testsuite_lines.append("    </testcase>")
                    total_tests += 1
                    total_failures += 1

            testsuite_lines.append("  </testsuite>")
            testsuites.extend(testsuite_lines)

        xml.append(
            f'<testsuites name="Auto-Linter" tests="{total_tests}" failures="{total_failures}">'
        )
        xml.extend(testsuites)
        xml.append("</testsuites>")

        return LogOutput(value="\n".join(xml))

    def report_to_dict(self, report: GovernanceReport) -> dict[Any, Any]:
        """Converts GovernanceReport entity to a plain dictionary for formatting."""
        data = {
            "score": float(report.score),
            "is_passing": bool(report.is_passing),
            "summary": {
                "violation_count": report.violation_count,
                "adapter_count": len(report.sources),
            },
        }

        for source in report.sources:
            source_name = str(source)
            results = report.results_by_source(source)
            data[source_name] = [
                {
                    "file": str(res.file),
                    "line": int(res.line),
                    "column": int(res.column),
                    "code": str(res.code),
                    "message": str(res.message),
                    "severity": str(res.severity),
                    "enclosing_scope": str(res.enclosing_scope)
                    if res.enclosing_scope
                    else None,
                }
                for res in results
            ]
        return data
