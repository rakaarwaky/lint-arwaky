"""Conditional skip logic. Quality-Analysis is NEVER skipped (Revision 2.0)."""
from __future__ import annotations

import re
from pathlib import Path

from src.common import Logger, now_iso
from src.config import Config

CODE_GLOBS = ("*.rs", "*.py", "*.ts")


class ConditionalSkip:
    def __init__(self, config: Config, log: Logger):
        self.config = config
        self.log = log

    # ── complexity ─────────────────────────────────────────────
    def analyze_complexity(self, feature_folder: str | Path) -> str:
        folder = Path(feature_folder)
        if not folder.is_dir():
            return "unknown"
        files = [f for g in CODE_GLOBS for f in folder.rglob(g)]
        file_count = len(files)
        loc = 0
        for f in files:
            try:
                loc += len(f.read_text(errors="ignore").splitlines())
            except OSError:
                pass
        if file_count <= 3 and loc <= 200:
            return "simple"
        if file_count <= 10 and loc <= 1000:
            return "medium"
        return "complex"

    # ── skip decisions ─────────────────────────────────────────
    def should_skip_business_analyst(self, feature_folder: str, pr_title: str = "") -> bool:
        if re.match(r"^(hotfix|fix|typo|docs|readme|changelog)", pr_title):
            self.log.write("skip", "skip", f"Skipping Business-Analyst for simple fix: {pr_title}")
            return True
        complexity = self.analyze_complexity(feature_folder)
        if complexity == "simple":
            self.log.write("skip", "skip",
                           f"Skipping Business-Analyst for simple feature (complexity: {complexity})")
            return True
        return False

    def should_skip_tech_lead(self, feature_folder: str, pr_title: str = "") -> bool:
        if re.search(r"(security|auth|token|password|encrypt)", pr_title):
            self.log.write("skip", "skip", f"Never skip Tech-Lead for security: {pr_title}")
            return False
        if re.match(r"^(docs|readme|changelog)", pr_title):
            code_files = [f for g in CODE_GLOBS for f in Path(feature_folder).rglob(g)]
            if len(code_files) == 0:
                self.log.write("skip", "skip", "Skipping Tech-Lead for doc-only update")
                return True
        return False

    def should_skip_architect(self, *_args) -> bool:
        return False    # NEVER SKIP — merge gate required

    def should_skip_developer(self, *_args) -> bool:
        return False    # NEVER SKIP — implements changes

    def should_skip_quality_analysis(self, *_args) -> bool:
        return False    # NEVER SKIP per DESIGN.md Revision 2.0

    def qa_mode(self, feature_folder: str, pr_title: str = "") -> str:
        if re.match(r"^(docs|readme|changelog)", pr_title):
            code_files = [f for g in CODE_GLOBS for f in Path(feature_folder).rglob(g)]
            if len(code_files) == 0:
                return "auto-approve-minor"
        return "full-review"

    # ── skip report ────────────────────────────────────────────
    def generate_skip_report(self, node: str, feature: str,
                             feature_path: str, reason: str) -> Path:
        out_dir = self.config.results_dir
        out_dir.mkdir(parents=True, exist_ok=True)
        report_file = out_dir / f"skip-report-{node}-{feature}.md"
        report_file.write_text(f"""## Skip Report — {node}
- **Feature:** {feature}
- **Feature Path:** {feature_path}
- **Skipped:** YES
- **Reason:** {reason}
- **Skipped at:** {now_iso()}

### Unvalidated Assumptions
- [ ] Business logic correctness: ASSUMED VALID
- [ ] Requirements traceability: ASSUMED VALID
- [ ] Edge case coverage: NOT CHECKED

### Architect Action Required
Architect must explicitly validate the assumptions above before producing merged plan.
""")
        self.log.write("skip", "skip_report", f"Skip Report generated for {node}: {report_file}")
        return report_file
