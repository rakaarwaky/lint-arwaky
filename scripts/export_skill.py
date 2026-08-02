#!/usr/bin/env python3
"""Export ALL skills for a selected language into a single consolidated Markdown file.

The output includes all files from every skill directory matching the language suffix
under `.agents/skills/<name>/`.

Usage:
    # Interactive mode (prompts for language):
    python3 scripts/export_skill.py

    # CLI mode (non-interactive):
    python3 scripts/export_skill.py --lang rust
    python3 scripts/export_skill.py --lang python --output /tmp/rust_skills.md
"""

import argparse
import re
import sys
from pathlib import Path

# Sanitize version strings to a safe filename fragment (CWE-22 mitigation).
SAFE_VERSION_CHARS = re.compile(r"[^0-9A-Za-z.\-]")

LANGUAGES = ["rust", "python", "typescript"]
LANG_SUFFIXES = {lang: f"-{lang}" for lang in LANGUAGES}


def resolve_project_root() -> tuple[Path, Path]:
    """Return (project_root, skills_dir). Exit on missing .agents/skills/."""
    project_root = Path(__file__).resolve().parent.parent
    skills_dir = project_root / ".agents" / "skills"

    if not skills_dir.exists():
        print(f"Error: '.agents/skills' directory not found at {skills_dir}", file=sys.stderr)
        sys.exit(1)
    return project_root, skills_dir


def list_skill_dirs(skills_dir: Path) -> list[str]:
    """Sorted list of skill directory names (subdirectories only)."""
    skill_dirs = []
    for entry in skills_dir.iterdir():
        if entry.is_dir() and not entry.name.startswith("-"):
            skill_dirs.append(entry.name)
    return sorted(skill_dirs)


def filter_by_language(skills: list[str], lang: str) -> list[str]:
    """Return skills whose name ends with -<lang>."""
    suffix = LANG_SUFFIXES[lang]
    return [s for s in skills if s.endswith(suffix)]


def prompt_language() -> str:
    """Show numbered language list, prompt for selection, return chosen language."""
    print("Select language:")
    for i, lang in enumerate(LANGUAGES, 1):
        print(f"  {i}) {lang.capitalize()}")
    print()

    while True:
        try:
            choice = input(f"Select (1-{len(LANGUAGES)}) or 'q' to quit: ").strip()
        except (EOFError, KeyboardInterrupt):
            print("\nExiting.")
            sys.exit(0)

        if choice.lower() == "q":
            print("Exiting.")
            sys.exit(0)

        try:
            idx = int(choice)
        except ValueError:
            print("Error: Invalid input. Please enter a valid number or 'q'.")
            continue

        if 1 <= idx <= len(LANGUAGES):
            return LANGUAGES[idx - 1]
        print(f"Error: Please choose a number between 1 and {len(LANGUAGES)}.")


def collect_skill_files(skill_path: Path) -> set[Path]:
    """Collect all files within the skill directory."""
    files: set[Path] = set()
    if not skill_path.exists():
        return files

    for f in skill_path.rglob("*"):
        if f.is_file():
            files.add(f)

    return files


def sanitize_version(version: str) -> str:
    """CWE-22: strip any character that could escape the .agents/finding directory."""
    safe = SAFE_VERSION_CHARS.sub("_", version)
    return safe or "0.0.0"


def _language_for(path: Path) -> str:
    """Pick a fenced-code-block language identifier based on file extension."""
    if path.name == "Cargo.toml":
        return "toml"
    if path.suffix == ".py":
        return "python"
    if path.suffix in (".js", ".ts"):
        return "javascript"
    if path.suffix == ".md":
        return "markdown"
    if path.suffix == ".yaml" or path.suffix == ".yml":
        return "yaml"
    if path.suffix == ".json":
        return "json"
    if path.suffix == ".rs":
        return "rust"
    return ""


def write_markdown(
    output_path: Path,
    skill_data: dict[str, list[Path]],
    project_root: Path,
    lang: str,
) -> None:
    """Write all skills for a language into one consolidated markdown file."""
    total_files = sum(len(files) for files in skill_data.values())

    with open(output_path, "w", encoding="utf-8") as out:
        out.write(f"# Skills: {lang.capitalize()} ({len(skill_data)} skills, {total_files} files)\n\n")
        out.write(
            f"This document contains all {lang} skills "
            f"from `.agents/skills/`.\n\n"
        )

        # Table of contents
        out.write("## Table of Contents\n\n")
        for skill_name in sorted(skill_data.keys()):
            out.write(f"- [{skill_name}](#{skill_name})\n")
        out.write("\n---\n\n")

        # Each skill
        for skill_name in sorted(skill_data.keys()):
            files = skill_data[skill_name]
            out.write(f"# {skill_name}\n\n")
            out.write(f"**Files:** {len(files)}\n\n")

            # File list
            out.write("## File List\n\n")
            for f in sorted(files):
                rel = f.relative_to(project_root)
                out.write(f"- [{rel}]({f.as_uri()})\n")
            out.write("\n---\n\n")

            # File contents
            for f in sorted(files):
                rel = f.relative_to(project_root)
                out.write(f"## File: {rel}\n\n")
                lang_id = _language_for(f)
                if lang_id:
                    out.write(f"```{lang_id}\n")
                else:
                    out.write("```\n")
                try:
                    content = f.read_text(encoding="utf-8", errors="replace")
                    escaped = content.replace("```", "``` `")
                    out.write(escaped)
                    if not content.endswith("\n"):
                        out.write("\n")
                except OSError as e:
                    out.write(f"/* Error reading file: {e} */\n")
                out.write("```\n\n---\n\n")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Export all skills for a language into a single consolidated Markdown file."
    )
    parser.add_argument(
        "--lang", "-l",
        choices=LANGUAGES,
        help="Language to export (rust/python/typescript). Omit for interactive selection.",
    )
    parser.add_argument(
        "--output", "-o",
        help="Output file path (default: .agents/skills/exports/<lang>.md).",
    )
    return parser.parse_args()


def export_all_skills(
    project_root: Path,
    skills_dir: Path,
    lang: str,
    output_path: Path | None = None,
) -> Path:
    """Export all skills for a language. Returns the output path written."""
    skill_dirs = list_skill_dirs(skills_dir)
    matched = filter_by_language(skill_dirs, lang)

    if not matched:
        print(f"Error: No skills found for language '{lang}'.", file=sys.stderr)
        sys.exit(1)
    # Default output: .agents/finding/<lang>.md
    if output_path is None:
        output_path = project_root / ".agents" / "finding" / f"{lang}.md"
    output_path.parent.mkdir(parents=True, exist_ok=True)

    # Collect files for each skill
    skill_data: dict[str, list[Path]] = {}
    for skill_name in matched:
        skill_path = skills_dir / skill_name
        files = collect_skill_files(skill_path)
        skill_data[skill_name] = sorted(files)
        print(f"  {skill_name}: {len(files)} file(s)")

    print(f"\n  Writing {len(matched)} skills to {output_path}...")
    write_markdown(output_path, skill_data, project_root, lang)
    return output_path


def main() -> None:
    args = parse_args()

    project_root, skills_dir = resolve_project_root()

    # Non-interactive CLI mode
    if args.lang:
        lang = args.lang
        print(f"Exporting all {lang} skills...")
        result = export_all_skills(project_root, skills_dir, lang, args.output)
        print(f"\nSuccess! Exported to: {result}")
        return

    # Interactive mode
    print("\n=== Lint Arwaky Skill Exporter ===")
    lang = prompt_language()
    print(f"\nExporting all {lang} skills...")
    result = export_all_skills(project_root, skills_dir, lang)
    print(f"\nSuccess! Exported to: {result}")


if __name__ == "__main__":
    main()
