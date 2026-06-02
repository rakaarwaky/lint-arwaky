from pathlib import Path

from ..taxonomy import FilePath, DirectoryPath, ConfigError
from ..contract import IConfigDiscoveryPort

# Supported language-specific config files, in fallback priority order.
_LANGUAGE_CONFIGS = [
    ("rust", "auto_linter.config.rust.yaml"),
    ("javascript", "auto_linter.config.javascript.yaml"),
    ("python", "auto_linter.config.python.yaml"),
]

# File extensions used to detect the dominant language in a directory.
_LANGUAGE_EXTENSIONS: dict[str, set[str]] = {
    "rust": {".rs"},
    "javascript": {".ts", ".tsx", ".js", ".jsx"},
    "python": {".py"},
}


class ConfigDiscoveryProvider(IConfigDiscoveryPort):
    """Provider for discovering configuration files in the file system.

    Supported config files (only these three are allowed):
      - auto_linter.config.rust.yaml
      - auto_linter.config.javascript.yaml
      - auto_linter.config.python.yaml
    """

    def find_env_file(self, start: DirectoryPath | None = None) -> FilePath | ConfigError | None:
        """Walk up from start to find .env file."""
        current = Path(str(start)) if start else Path.cwd()
        for _ in range(5):
            candidate = current / ".env"
            if candidate.is_file():
                return FilePath(value=str(candidate))
            if current.parent == current:
                break
            current = current.parent
        return None

    def find_yaml_config(self, start: DirectoryPath | None = None) -> FilePath | ConfigError | None:
        """Find the best-matching language config by walking up from *start*."""
        start_path = Path(str(start)) if start else Path.cwd()
        dominant = self._detect_dominant_language(start_path)

        ordered = self._priority_order(dominant)

        current = start_path
        for _ in range(5):
            for _lang, name in ordered:
                candidate = current / name
                if candidate.is_file():
                    return FilePath(value=str(candidate))
            if current.parent == current:
                break
            current = current.parent
        return None

    def _priority_order(self, dominant: str | None) -> list[tuple[str, str]]:
        """Return _LANGUAGE_CONFIGS reordered so *dominant* comes first."""
        if dominant is None:
            return _LANGUAGE_CONFIGS
        first = [(lang, name) for lang, name in _LANGUAGE_CONFIGS if lang == dominant]
        rest = [(lang, name) for lang, name in _LANGUAGE_CONFIGS if lang != dominant]
        return first + rest

    def _detect_dominant_language(self, path: Path) -> str | None:
        """Detect the dominant source language inside *path* by file extension."""
        counts: dict[str, int] = {lang: 0 for lang in _LANGUAGE_EXTENSIONS}
        try:
            for entry in path.rglob("*"):
                if not entry.is_file():
                    continue
                suffix = entry.suffix.lower()
                for lang, exts in _LANGUAGE_EXTENSIONS.items():
                    if suffix in exts:
                        counts[lang] += 1
                        break
        except (PermissionError, OSError):
            return None
        total = sum(counts.values())
        if total == 0:
            return None
        dominant = max(counts, key=lambda k: counts[k])
        return dominant if counts[dominant] > 0 else None

    def find_toml_config(self, start: DirectoryPath | None = None) -> FilePath | ConfigError | None:
        """TOML configuration is no longer supported per project guidelines."""
        return None
