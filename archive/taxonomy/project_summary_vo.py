"""project_summary_vo — Multi-project analysis value objects."""

from typing import cast
from pydantic import BaseModel, field_validator, Field
from pydantic.config import ConfigDict
from .file_path_vo import FilePath
from .message_status_vo import ComplianceStatus, Count
from .score_format_vo import Score
from .error_value_vo import ErrorMessage
from .path_collection_vo import PatternList


class ProjectResult(BaseModel):
    """Result from a single project scan."""

    model_config = ConfigDict(frozen=False)
    path: FilePath
    score: Score
    is_passing: ComplianceStatus
    issues: list[dict[str, object]] = Field(default_factory=list)
    adapters: PatternList = Field(default_factory=PatternList)
    error: ErrorMessage | None = None

    @field_validator("path", mode="before")
    @classmethod
    def validate_path(cls, v: object) -> FilePath:
        if isinstance(v, str):
            return FilePath(value=v)
        return cast(FilePath, v)

    @field_validator("score", mode="before")
    @classmethod
    def validate_score(cls, v: object) -> Score:
        if isinstance(v, (int, float)):
            return Score(value=float(v))
        return cast(Score, v)

    @field_validator("is_passing", mode="before")
    @classmethod
    def validate_is_passing(cls, v: object) -> ComplianceStatus:
        if isinstance(v, bool):
            return ComplianceStatus(value=v)
        return cast(ComplianceStatus, v)

    @field_validator("adapters", mode="before")
    @classmethod
    def validate_adapters(cls, v: object) -> PatternList:
        if isinstance(v, list) and all((isinstance(i, str) for i in v)):
            return PatternList(values=v)
        return cast(PatternList, v)

    @field_validator("error", mode="before")
    @classmethod
    def validate_error(cls, v: object) -> ErrorMessage | None:
        if isinstance(v, str):
            return ErrorMessage(value=v)
        return cast(ErrorMessage | None, v)


class AggregatedResults(BaseModel):
    """Aggregated results from multiple projects."""

    model_config = ConfigDict(frozen=False)
    projects: list[ProjectResult] = Field(default_factory=list)
    total_projects: Count = Field(default_factory=lambda: Count(value=0))
    passing_projects: Count = Field(default_factory=lambda: Count(value=0))
    failing_projects: Count = Field(default_factory=lambda: Count(value=0))
    average_score: Score = Field(default_factory=lambda: Score(value=0.0))

    @field_validator("average_score", mode="before")
    @classmethod
    def validate_average_score(cls, v: object) -> Score:
        if isinstance(v, (int, float)):
            return Score(value=float(v))
        return cast(Score, v)

    @field_validator(
        "total_projects", "passing_projects", "failing_projects", mode="before"
    )
    @classmethod
    def validate_counts(cls, v: object) -> Count:
        if isinstance(v, int):
            return Count(value=v)
        return cast(Count, v)

    def to_dict(self) -> dict:
        return {
            "projects": [
                {
                    "path": str(p.path),
                    "score": float(p.score),
                    "is_passing": bool(p.is_passing),
                    "issues_count": len(p.issues),
                    "adapters": list(p.adapters.values or []),
                    "error": str(p.error) if p.error else None,
                }
                for p in self.projects
            ],
            "summary": {
                "total_projects": int(self.total_projects),
                "passing_projects": int(self.passing_projects),
                "failing_projects": int(self.failing_projects),
                "average_score": float(self.average_score),
            },
        }

    def to_text(self) -> str:
        lines = ["Multi-Project Scan Results", "=" * 40]
        for p in self.projects:
            if p.error:
                lines.append(f" ERROR {p.path}: {p.error}")
            else:
                status = " PASS" if p.is_passing else " FAIL"
                lines.append(f"{status} {p.path} (score: {p.score})")
        summary = self.to_dict()["summary"]
        lines.extend(
            [
                "",
                "Summary:",
                f"  Total: {summary['total_projects']}",
                f"  Passing: {summary['passing_projects']}",
                f"  Failing: {summary['failing_projects']}",
                f"  Average Score: {summary['average_score']:.1f}",
            ]
        )
        return "\n".join(lines)
