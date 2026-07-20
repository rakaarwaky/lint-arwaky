#!/usr/bin/env python3
"""Export a selected skill into a single consolidated Markdown file.

The output includes all files within the skill directory under `.agents/skills/<name>/`.
"""

import re
import sys
from pathlib import Path

# Sanitize version strings to a safe filename fragment (CWE-22 mitigation).
SAFE_VERSION_CHARS = re.compile(r"[^0-9A-Za-z.\-]")


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


def prompt_skill(skill_dirs: list[str]) -> str:
    """Show numbered list, prompt for selection, return the chosen skill name."""
    print("Available skills:")
    for i, name in enumerate(skill_dirs, 1):
        print(f"{i:2d}) {name}")
    print()

    while True:
        try:
            choice = input(
                f"Select a skill (1-{len(skill_dirs)}) or 'q' to quit: "
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

        if 0 <= idx < len(skill_dirs):
            return skill_dirs[idx]
        print(f"Error: Please choose a number between 1 and {len(skill_dirs)}.")


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
    sorted_files: list[Path],
    project_root: Path,
    selected_skill: str,
) -> None:
    with open(output_path, "w", encoding="utf-8") as out:
        out.write(f"# Skill: {selected_skill}\n\n")
        out.write(
            f"This document contains all files for skill `{selected_skill}` "
            f"from `.agents/skills/{selected_skill}/`.\n\n"
        )

        out.write("## File List\n\n")
        for f in sorted_files:
            rel = f.relative_to(project_root)
            out.write(f"- [{rel}]({f.as_uri()})\n")
        out.write("\n---\n\n")

        for f in sorted_files:
            rel = f.relative_to(project_root)
            out.write(f"## File: {rel}\n\n")
            lang = _language_for(f)
            if lang:
                out.write(f"```{lang}\n")
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


def main() -> None:
    while True:
        print("\n=== Lint Arwaky Skill Exporter ===")

        project_root, skills_dir = resolve_project_root()

        skill_dirs = list_skill_dirs(skills_dir)
        if not skill_dirs:
            print("Error: No skills found in '.agents/skills' directory.", file=sys.stderr)
            sys.exit(1)

        selected_skill = prompt_skill(skill_dirs)
        print(f"\nProcessing skill: {selected_skill}...")

        skill_path = skills_dir / selected_skill
        files_to_export = collect_skill_files(skill_path)

        output_filename = f"{selected_skill}.md"
        output_path = skills_dir / "exports" / output_filename
        output_path.parent.mkdir(parents=True, exist_ok=True)

        print(f"Collecting {len(files_to_export)} file(s)...")
        sorted_files = sorted(files_to_export)

        print(f"Writing export to {output_path}...")
        write_markdown(
            output_path,
            sorted_files,
            project_root,
            selected_skill,
        )

        print(f"\nSuccess! Consolidated markdown file created: {output_path}")

        try:
            again = input("\nExport another skill? (y/n): ").strip().lower()
        except (EOFError, KeyboardInterrupt):
            print("\nExiting.")
            break
        if again != "y":
            break

    print("Done.")


if __name__ == "__main__":
    main()
