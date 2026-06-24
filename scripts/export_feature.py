#!/usr/bin/env python3
"""Export a selected feature crate into a single consolidated Markdown file.

The output includes the crate's own source, its Cargo.toml, and any shared
crate modules transitively reachable through `shared::...` import paths.
"""
import re
import subprocess
import sys
from pathlib import Path

CARGO_TOML = "Cargo.toml"
VERSION_PATTERN = re.compile(r'(?m)^\[package\].*?^version\s*=\s*"([^"]+)"', re.MULTILINE | re.DOTALL)
WORKSPACE_VERSION_PATTERN = re.compile(r'(?m)^\[package\].*?version\.workspace\s*=\s*true', re.MULTILINE | re.DOTALL)
DECL_PATTERN = re.compile(
    r'\bpub(?:\([^)]+\))?\s+(struct|enum|trait|type|fn|const|static|mod)\s+(\w+)'
)
MACRO_PATTERN = re.compile(r'\bmacro_rules!\s+(\w+)')
SHARED_PATH_PATTERN = re.compile(r'\bshared::([a-zA-Z0-9_:]+)')

# Sanitize version strings to a safe filename fragment (CWE-22 mitigation).
SAFE_VERSION_CHARS = re.compile(r'[^0-9A-Za-z.\-]')


def resolve_workspace() -> tuple[Path, Path, Path]:
    """Return (workspace_root, crates_dir, docs_finding_dir). Exit on missing crates/."""
    workspace_root = Path(__file__).resolve().parent.parent
    crates_dir = workspace_root / "crates"
    docs_finding_dir = workspace_root / "docs" / "finding"

    if not crates_dir.exists():
        print(f"Error: 'crates' directory not found at {crates_dir}", file=sys.stderr)
        sys.exit(1)
    return workspace_root, crates_dir, docs_finding_dir


def list_feature_crates(crates_dir: Path) -> list[str]:
    """Sorted list of crate directory names that contain a Cargo.toml, excluding 'shared'."""
    feature_crates = []
    for entry in crates_dir.iterdir():
        if entry.is_dir() and entry.name != "shared" and (entry / CARGO_TOML).exists():
            feature_crates.append(entry.name)
    return sorted(feature_crates)


def prompt_crate(feature_crates: list[str]) -> str:
    """Show numbered list, prompt for selection, return the chosen crate name."""
    print("Available feature crates:")
    for i, name in enumerate(feature_crates, 1):
        print(f"{i:2d}) {name}")
    print()

    while True:
        try:
            choice = input(
                f"Select a crate (1-{len(feature_crates)}) or 'q' to quit: "
            ).strip()
        except (EOFError, KeyboardInterrupt):
            print("\nExiting.")
            sys.exit(0)

        if choice.lower() == "q":
            print("Exiting.")
            sys.exit(0)

        try:
            idx = int(choice) - 1
        except ValueError:
            print("Error: Invalid input. Please enter a valid number or 'q'.")
            continue

        if 0 <= idx < len(feature_crates):
            return feature_crates[idx]
        print(f"Error: Please choose a number between 1 and {len(feature_crates)}.")


def read_crate_version(crate_path: Path, fallback: str = "0.1.0") -> str:
    cargo_toml_path = crate_path / CARGO_TOML
    if not cargo_toml_path.exists():
        return fallback

    try:
        content = cargo_toml_path.read_text(encoding="utf-8", errors="replace")
    except OSError as e:
        print(f"Warning: Could not read {cargo_toml_path} ({e}). Defaulting to {fallback}.")
        return fallback

    if WORKSPACE_VERSION_PATTERN.search(content):
        print(f"Note: Using workspace version for {crate_path.name}")
        return "workspace"

    in_package = False
    for line in content.splitlines():
        stripped = line.strip()
        if stripped == "[package]":
            in_package = True
            continue
        if stripped.startswith("[") and in_package:
            in_package = False
            continue
        if in_package:
            match = re.match(r'^version\s*=\s*"([^"]+)"', stripped)
            if match:
                return match.group(1)

    return fallback


def sanitize_version(version: str) -> str:
    """CWE-22: strip any character that could escape the docs/finding directory."""
    safe = SAFE_VERSION_CHARS.sub("_", version)
    return safe or "0.0.0"


def index_shared_module(shared_src_dir: Path) -> tuple[dict[str, list[Path]], dict[str, list[Path]]]:
    module_to_files: dict[str, list[Path]] = {}
    symbol_to_files: dict[str, list[Path]] = {}

    if not shared_src_dir.exists():
        print("Warning: 'crates/shared/src' directory not found. Shared dependencies cannot be resolved.")
        return module_to_files, symbol_to_files

    print("Indexing shared crate for resolving dependencies...")
    for f in shared_src_dir.rglob("*.rs"):
        if f.name == "mod.rs":
            mod_name = f.parent.name.replace("-", "_")
        else:
            mod_name = f.stem.replace("-", "_")
        module_to_files.setdefault(mod_name, []).append(f)

        try:
            content = f.read_text(encoding="utf-8", errors="replace")
        except OSError as e:
            print(f"Warning: Failed to index file {f} ({e})")
            continue

        for match in DECL_PATTERN.finditer(content):
            decl_type, decl_name = match.group(1), match.group(2)
            if decl_type == "mod":
                module_to_files.setdefault(decl_name, []).append(f)
            else:
                symbol_to_files.setdefault(decl_name, []).append(f)

        for match in MACRO_PATTERN.finditer(content):
            symbol_to_files.setdefault(match.group(1), []).append(f)

    return module_to_files, symbol_to_files


def resolve_dependency_files(
    components: list[str],
    module_to_files: dict[str, list[Path]],
    symbol_to_files: dict[str, list[Path]],
) -> set[Path]:
    resolved: set[Path] = set()
    for comp in components:
        if comp in module_to_files:
            files = module_to_files[comp]
            if len(files) == 1:
                resolved.add(files[0])
            else:
                scored = _score_files(files, components)
                scored.sort(key=lambda item: item[0], reverse=True)
                if scored and scored[0][0] > 0:
                    best_score = scored[0][0]
                    resolved.update(f for score, f in scored if score == best_score)
                else:
                    resolved.add(files[0])

        if comp in symbol_to_files:
            files = symbol_to_files[comp]
            if len(files) == 1:
                resolved.add(files[0])
            else:
                scored = _score_files(files, components)
                scored.sort(key=lambda item: item[0], reverse=True)
                if scored and scored[0][0] > 0:
                    best_score = scored[0][0]
                    resolved.update(f for score, f in scored if score == best_score)
                else:
                    resolved.add(files[0])
    return resolved


def _score_files(files: list[Path], components: list[str]) -> list[tuple[int, Path]]:
    scored: list[tuple[int, Path]] = []
    for f in files:
        f_parts = [p.replace("-", "_") for p in f.parts]
        score = 0
        for i, c in enumerate(components):
            if c in f_parts:
                score += len(components) - i
        scored.append((score, f))
    return scored


def collect_crate_files(crate_path: Path) -> set[Path]:
    files: set[Path] = set()
    src_dir = crate_path / "src"
    important_files = {CARGO_TOML, "build.rs", "README.md", "LICENSE", "LICENSE-MIT", "LICENSE-APACHE"}
    
    for f in crate_path.iterdir():
        if f.is_file() and f.name in important_files:
            files.add(f)
    
    if src_dir.exists():
        for f in src_dir.rglob("*"):
            if f.is_file():
                files.add(f)
    
    return files


def add_shared_feature_dir(
    files: set[Path],
    shared_src_dir: Path,
    feature_dashed: str,
) -> None:
    """Add crates/shared/src/<feature>/**/*.rs when the per-feature folder exists."""
    feature_dir = shared_src_dir / feature_dashed
    if not (feature_dir.exists() and feature_dir.is_dir()):
        return
    for f in feature_dir.rglob("*.rs"):
        if f.is_file():
            files.add(f)


def scan_shared_imports(
    files: set[Path],
    module_to_files: dict[str, list[Path]],
    symbol_to_files: dict[str, list[Path]],
) -> set[Path]:
    print("Scanning source files for imported shared dependencies...")
    extra: set[Path] = set()
    scanned: set[Path] = set()
    
    for f in files:
        if f.suffix != ".rs" or f in scanned:
            continue
        scanned.add(f)
        try:
            content = f.read_text(encoding="utf-8", errors="replace")
        except OSError as e:
            print(f"Warning: Failed to read file {f} for dependency analysis ({e})")
            continue
        for path_str in SHARED_PATH_PATTERN.findall(content):
            components = path_str.split("::")
            extra.update(resolve_dependency_files(components, module_to_files, symbol_to_files))
    return extra


def run_lint_scan(workspace_root: Path, crate_path: Path) -> str:
    import shutil
    cli_bin = shutil.which("lint-arwaky-cli")
    if not cli_bin:
        cli_bin_candidate = workspace_root / "target" / "release" / "lint-arwaky-cli"
        if cli_bin_candidate.exists():
            cli_bin = str(cli_bin_candidate)
        else:
            cli_bin_candidate = workspace_root / "target" / "debug" / "lint-arwaky-cli"
            if cli_bin_candidate.exists():
                cli_bin = str(cli_bin_candidate)
    
    if not cli_bin:
        print("Warning: lint-arwaky-cli not found. Run install.local.sh first.")
        return ""

    try:
        result = subprocess.run(
            [cli_bin, "scan", str(crate_path)],
            capture_output=True,
            text=True,
            timeout=120,
            cwd=str(workspace_root),
        )
        output = result.stdout + result.stderr
        lines = [line.rstrip() for line in output.splitlines() if line.strip()]
        return "\n".join(lines) if lines else ""
    except subprocess.TimeoutExpired:
        print("Warning: lint scan timed out after 120s.")
        return ""
    except OSError as e:
        print(f"Warning: Failed to run lint scan ({e}).")
        return ""


def write_markdown(
    output_path: Path,
    sorted_files: list[Path],
    workspace_root: Path,
    selected_crate: str,
    safe_version: str,
    lint_output: str,
) -> None:
    with open(output_path, "w", encoding="utf-8") as out:
        out.write(f"# Crate: {selected_crate} (v{safe_version})\n\n")
        out.write(
            f"This document contains the source code for feature crate `{selected_crate}` "
        )
        out.write("along with its corresponding and imported definitions from the `shared` crate.\n\n")

        if lint_output:
            out.write("## Problem Statement\n\n")
            out.write("The following issues were detected by `lint-arwaky-cli scan`:\n\n")
            out.write("```\n")
            out.write(lint_output)
            if not lint_output.endswith("\n"):
                out.write("\n")
            out.write("```\n\n")
            out.write("---\n\n")

        out.write("## File List\n\n")
        for f in sorted_files:
            rel = f.relative_to(workspace_root)
            out.write(f"- [{rel}]({f.as_uri()})\n")
        out.write("\n---\n\n")

        for f in sorted_files:
            rel = f.relative_to(workspace_root)
            out.write(f"## File: {rel}\n\n")
            lang = _language_for(f)
            out.write(f"```{lang}\n")
            try:
                content = f.read_text(encoding="utf-8", errors="replace")
                escaped = content.replace("```", "``` `")
                out.write(escaped)
                if not content.endswith("\n"):
                    out.write("\n")
            except OSError as e:
                out.write(f"/* Error reading file: {e} */\n")
            out.write("```\n\n---\n\n")


def _language_for(path: Path) -> str:
    """Pick a fenced-code-block language identifier based on file extension."""
    if path.name == CARGO_TOML:
        return "toml"
    if path.suffix == ".py":
        return "python"
    if path.suffix in (".js", ".ts"):
        return "javascript"
    return "rust"


def main() -> None:
    print("=== Lint Arwaky Feature Exporter ===")

    workspace_root, crates_dir, docs_finding_dir = resolve_workspace()

    feature_crates = list_feature_crates(crates_dir)
    if not feature_crates:
        print("Error: No feature crates found in 'crates' directory.", file=sys.stderr)
        sys.exit(1)

    selected_crate = prompt_crate(feature_crates)
    print(f"\nProcessing crate: {selected_crate}...")

    crate_path = crates_dir / selected_crate
    version = read_crate_version(crate_path)
    safe_version = sanitize_version(version)
    print(f"Version resolved: {version} (filename-safe: {safe_version})")

    shared_src_dir = crates_dir / "shared" / "src"
    module_to_files, symbol_to_files = index_shared_module(shared_src_dir)

    files_to_export = collect_crate_files(crate_path)
    feature_dashed = selected_crate.replace("_", "-")
    add_shared_feature_dir(files_to_export, shared_src_dir, feature_dashed)
    files_to_export.update(
        scan_shared_imports(files_to_export, module_to_files, symbol_to_files)
    )

    print("Running lint-arwaky-cli scan...")
    lint_output = run_lint_scan(workspace_root, crate_path)
    if lint_output:
        print("Lint scan completed.")
    else:
        print("No lint output (clean or scan failed).")

    docs_finding_dir.mkdir(parents=True, exist_ok=True)
    output_filename = f"{selected_crate}_v{safe_version}.md"
    output_path = docs_finding_dir / output_filename

    print(f"Writing export to {output_path}...")
    sorted_files = sorted(files_to_export)
    write_markdown(output_path, sorted_files, workspace_root, selected_crate, safe_version, lint_output)

    print(f"Success! Consolidate markdown file created: {output_path}")


if __name__ == "__main__":
    main()
