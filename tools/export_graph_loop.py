#!/usr/bin/env python3
"""Export the entire .agents/graph-loop/ folder into a single Markdown file.

Output is saved to .agents/finding/graph-loop.md.

Usage:
    python3 tools/export_graph_loop.py
    python3 tools/export_graph_loop.py --output /tmp/graph-loop.md
"""

import argparse
import sys
from pathlib import Path

SKIP_DIRS = {"locks", "reports", "results"}
SKIP_FILES = set()


def resolve_workspace() -> tuple[Path, Path]:
    workspace_root = Path(__file__).resolve().parent.parent
    graph_loop_dir = workspace_root / ".agents" / "graph-loop"
    finding_dir = workspace_root / ".agents" / "finding"

    if not graph_loop_dir.exists():
        print(f"Error: graph-loop dir not found at {graph_loop_dir}", file=sys.stderr)
        sys.exit(1)

    return graph_loop_dir, finding_dir


def collect_files(graph_loop_dir: Path) -> list[Path]:
    """Collect all files, skipping runtime dirs and empty files."""
    files: list[Path] = []

    for path in sorted(graph_loop_dir.rglob("*")):
        if not path.is_file():
            continue
        if path.name in SKIP_FILES:
            continue

        # Skip files inside runtime directories
        try:
            rel = path.relative_to(graph_loop_dir)
        except ValueError:
            continue

        if rel.parts and rel.parts[0] in SKIP_DIRS:
            continue

        # Skip empty files
        if path.stat().st_size == 0:
            continue

        files.append(path)

    return files


def _language_for(path: Path) -> str:
    ext_map = {
        ".py": "python",
        ".sh": "bash",
        ".yaml": "yaml",
        ".yml": "yaml",
        ".json": "json",
        ".js": "javascript",
        ".ts": "typescript",
        ".md": "markdown",
        ".toml": "toml",
        ".txt": "text",
        ".service": "ini",
    }
    return ext_map.get(path.suffix, "")


def write_markdown(
    output_path: Path,
    files: list[Path],
    graph_loop_dir: Path,
) -> None:
    with open(output_path, "w", encoding="utf-8") as out:
        out.write("# Graph Loop — Exported Source\n\n")
        out.write(
            "This document contains all source files from the `.agents/graph-loop/` directory.\n\n"
        )

        out.write("## File List\n\n")
        for f in files:
            rel = f.relative_to(graph_loop_dir)
            out.write(f"- `{rel}`\n")
        out.write("\n---\n\n")

        for f in files:
            rel = f.relative_to(graph_loop_dir)
            lang = _language_for(f)
            out.write(f"## File: {rel}\n\n")
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


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Export .agents/graph-loop/ into a single Markdown file."
    )
    parser.add_argument(
        "--output", "-o",
        help="Output file path (default: .agents/finding/graph-loop.md).",
    )
    args = parser.parse_args()

    graph_loop_dir, finding_dir = resolve_workspace()
    files = collect_files(graph_loop_dir)

    if not files:
        print("Error: No files found in graph-loop directory.", file=sys.stderr)
        sys.exit(1)

    print(f"Found {len(files)} files in .agents/graph-loop/")

    if args.output:
        output_path = Path(args.output)
    else:
        finding_dir.mkdir(parents=True, exist_ok=True)
        output_path = finding_dir / "graph-loop.md"

    write_markdown(output_path, files, graph_loop_dir)
    print(f"Exported to: {output_path}")


if __name__ == "__main__":
    main()
