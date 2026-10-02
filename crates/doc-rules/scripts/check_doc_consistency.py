#!/usr/bin/env python3
"""Doc-consistency gate.

Catches the classes of documentation drift that the AES doc rules (AES601–605)
do not cover, because each of them spans two artefacts rather than living
inside one document:

1. rule ranges   — a rule-code range in a DESIGN.md/FRD.md names a code that
                   RULES_AES.md no longer publishes (issue #537).
2. anchors       — an in-repo Markdown link points at a file or a heading
                   anchor that does not exist (issue #551).
3. data model    — crates/shared/DATA.md's attribute tables disagree with the
                   shared value objects they document (issue #540).
4. fix reasons   — crates/auto-fix/FRD.md's Reason Code Reference disagrees
                   with the enumerated FixOutcome reasons (issue #545).
5. performance   — the performance NFR states different numbers in PRD.md,
                   README.md, and crates/filesystem/FRD.md (issue #549).
6. DESIGN H2     — the AES605 DESIGN.md H2 contract in the doc-rules constants
                   disagrees with the DESIGN.md template in
                   HOW-TO-MAKE-DESIGN.md, so copying the shipped template
                   fires AES605 the moment the file is written.
7. FRD H3        — the AES602 FRD level-3 contract in the doc-rules constants
                   disagrees with the FRD.md template in HOW-TO-MAKE-FRD.md, so
                   copying the shipped template fires AES602 the moment the
                   file is written.

Exit code 0 when every check passes, 1 otherwise. No third-party dependencies:
this runs anywhere python3 does, including a CI job with no Rust toolchain.
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent.parent

# Fixture workspaces are deliberately broken; generated or vendored trees are
# not ours to police.
EXCLUDED_DIRS = {
    ".git",
    "target",
    "node_modules",
    ".worktree",
    "workspaces-bad",
    "workspaces-good",
    # Local scratch trees, already gitignored: `.qwen-web/input/` holds
    # timestamped snapshots of past scans whose links are relative to the
    # snapshot, not to this repo, so every link in them is a false failure.
    ".qwen-web",
    ".qwen",
    ".agents",
}


def _gitignored(rel: Path) -> bool:
    """True when git ignores this path, so a scratch tree cannot gate CI.

    `EXCLUDED_DIRS` is the floor, not the whole rule: any directory a developer
    has gitignored locally must not decide whether the doc gate passes, or the
    result depends on whose machine ran it. Falls back to False when git is
    unavailable so the explicit list still governs.
    """
    try:
        result = subprocess.run(
            ["git", "-C", str(ROOT), "check-ignore", "-q", str(rel)],
            capture_output=True,
            check=False,
        )
    except OSError:
        return False
    return result.returncode == 0


def markdown_files() -> list[Path]:
    out = []
    for path in ROOT.rglob("*.md"):
        rel = path.relative_to(ROOT)
        if any(part in EXCLUDED_DIRS for part in rel.parts):
            continue
        if _gitignored(rel):
            continue
        out.append(path)
    return sorted(out)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def strip_code_fences(text: str) -> str:
    """Blank out fenced blocks so examples are not mistaken for statements."""
    out, inside = [], False
    for line in text.splitlines():
        if line.lstrip().startswith("```"):
            inside = not inside
            out.append("")
            continue
        out.append("" if inside else line)
    return "\n".join(out)


# ─── 1. Rule-code ranges ────────────────────────────────────────────────────

RULES_DOC = ROOT / "RULES_AES.md"
RANGE_RE = re.compile(r"\bAES(\d{3})\s*[–—-]\s*(?:AES)?(\d{3})\b")


def published_rule_codes() -> set[str]:
    text = RULES_DOC.read_text(encoding="utf-8")
    # The summary table rows: `| AES101 | Name | ...`
    return set(re.findall(r"^\|\s*(AES\d{3})\s*\|", text, flags=re.MULTILINE))


def check_rule_ranges() -> list[str]:
    published = published_rule_codes()
    if not published:
        return [f"{rel(RULES_DOC)}: no rule codes found in the summary table"]
    errors = []
    for path in markdown_files():
        if path == RULES_DOC:
            continue
        if path.name not in {"DESIGN.md", "FRD.md", "DATA.md", "BACKLOG.md"}:
            continue
        text = strip_code_fences(path.read_text(encoding="utf-8"))
        for lineno, line in enumerate(text.splitlines(), start=1):
            for start, end in RANGE_RE.findall(line):
                if start[0] != end[0] or int(end) < int(start):
                    continue  # not a single-group ascending range
                for number in range(int(start), int(end) + 1):
                    code = f"AES{number}"
                    if code not in published:
                        errors.append(
                            f"{rel(path)}:{lineno}: range AES{start}–{end} names "
                            f"{code}, which RULES_AES.md does not publish"
                        )
    return errors


# ─── 2. In-repo Markdown anchors ────────────────────────────────────────────

LINK_RE = re.compile(r"\[(?:[^\]]*)\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")
HEADING_RE = re.compile(r"^(#{1,6})\s+(.*?)\s*$", flags=re.MULTILINE)


def slugify(heading: str) -> str:
    """GitHub's heading-anchor slug."""
    text = heading.strip().lower()
    text = re.sub(r"`([^`]*)`", r"\1", text)
    text = re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", text)
    text = re.sub(r"[*_~]", "", text)
    text = re.sub(r"[^\w\s-]", "", text, flags=re.UNICODE)
    # GitHub replaces each whitespace character with a hyphen; it does not
    # collapse runs, so "A & B" becomes "a--b".
    return re.sub(r"\s", "-", text.strip())


def anchors_of(path: Path) -> set[str]:
    text = strip_code_fences(path.read_text(encoding="utf-8"))
    found: set[str] = set()
    seen: dict[str, int] = {}
    for _, title in HEADING_RE.findall(text):
        slug = slugify(title)
        count = seen.get(slug, 0)
        seen[slug] = count + 1
        found.add(slug if count == 0 else f"{slug}-{count}")
    return found


def check_anchors() -> list[str]:
    errors = []
    cache: dict[Path, set[str]] = {}
    for path in markdown_files():
        text = strip_code_fences(path.read_text(encoding="utf-8"))
        for lineno, line in enumerate(text.splitlines(), start=1):
            for target in LINK_RE.findall(line):
                if target.startswith(("http://", "https://", "mailto:", "tel:")):
                    continue
                file_part, _, anchor = target.partition("#")
                if not anchor:
                    continue
                if file_part:
                    target_path = (path.parent / file_part).resolve()
                    if not target_path.is_file():
                        errors.append(
                            f"{rel(path)}:{lineno}: link target '{file_part}' does not exist"
                        )
                        continue
                    if target_path.suffix != ".md":
                        continue
                else:
                    target_path = path
                if target_path not in cache:
                    cache[target_path] = anchors_of(target_path)
                if anchor.lower() not in cache[target_path]:
                    errors.append(
                        f"{rel(path)}:{lineno}: anchor '#{anchor}' not found in "
                        f"{rel(target_path)}"
                    )
    return errors


# ─── 3. Shared data model ───────────────────────────────────────────────────

DATA_DOC = ROOT / "crates" / "shared" / "DATA.md"
SHARED_SRC = ROOT / "crates" / "shared" / "src"

# Documented object → (source module, Rust item name)
DOCUMENTED_OBJECTS = {
    "ViolationItem": ("common/taxonomy_violation_item_vo", "ViolationItem"),
    "StructureFinding": (
        "structure_rules/taxonomy_structure_rules_request",
        "StructureFinding",
    ),
    "DocFinding": ("doc_rules/taxonomy_doc_rules_request", "DocFinding"),
}


def documented_attributes(name: str) -> set[str] | None:
    text = DATA_DOC.read_text(encoding="utf-8")
    pattern = re.compile(
        r"^###\s+DO-\d+:\s*" + re.escape(name) + r"\b.*?$(.*?)(?=^###\s|\Z)",
        flags=re.MULTILINE | re.DOTALL,
    )
    match = pattern.search(text)
    if not match:
        return None
    fields = set()
    for row in match.group(1).splitlines():
        row = row.strip()
        if not row.startswith("|"):
            continue
        cells = [c.strip() for c in row.strip("|").split("|")]
        if not cells or cells[0] in {"Field", "Value", ""} or set(cells[0]) <= {"-", ":"}:
            continue
        fields.add(cells[0].strip("`"))
    return fields


def struct_fields(module: str, item: str) -> set[str] | None:
    source = SHARED_SRC / f"{module}.rs"
    if not source.is_file():
        return None
    text = source.read_text(encoding="utf-8")
    match = re.search(
        r"pub struct\s+" + re.escape(item) + r"\s*\{(.*?)^\}",
        text,
        flags=re.DOTALL | re.MULTILINE,
    )
    if not match:
        return None
    return set(re.findall(r"^\s*pub\s+(\w+)\s*:", match.group(1), flags=re.MULTILINE))


def enum_variants(module: str, item: str) -> set[str] | None:
    source = SHARED_SRC / f"{module}.rs"
    if not source.is_file():
        return None
    text = source.read_text(encoding="utf-8")
    match = re.search(
        r"pub enum\s+" + re.escape(item) + r"\s*\{(.*?)^\}",
        text,
        flags=re.DOTALL | re.MULTILINE,
    )
    if not match:
        return None
    body = re.sub(r"^\s*(///|//|#\[).*$", "", match.group(1), flags=re.MULTILINE)
    return set(re.findall(r"^\s*([A-Z]\w*)", body, flags=re.MULTILINE))


def check_data_model() -> list[str]:
    errors = []
    for name, (module, item) in DOCUMENTED_OBJECTS.items():
        documented = documented_attributes(name)
        actual = struct_fields(module, item)
        if documented is None:
            errors.append(f"{rel(DATA_DOC)}: no attribute table for {name}")
            continue
        if actual is None:
            errors.append(f"cannot locate struct {item} in shared/{module}")
            continue
        for missing in sorted(actual - documented):
            errors.append(
                f"{rel(DATA_DOC)}: {name} field '{missing}' exists in the shared "
                "kernel but is not documented"
            )
        for extra in sorted(documented - actual):
            errors.append(
                f"{rel(DATA_DOC)}: {name} documents field '{extra}', which the "
                "shared kernel does not define"
            )

    severity = enum_variants("common/taxonomy_severity_vo", "Severity")
    documented_severity = documented_attributes("Severity")
    if severity and documented_severity is not None:
        for missing in sorted(severity - documented_severity):
            errors.append(
                f"{rel(DATA_DOC)}: Severity value '{missing}' is undocumented"
            )
        for extra in sorted(documented_severity - severity):
            errors.append(
                f"{rel(DATA_DOC)}: Severity documents '{extra}', which the enum "
                "does not define"
            )
    return errors


# ─── 4. Auto-fix reason codes ───────────────────────────────────────────────

AUTOFIX_FRD = ROOT / "crates" / "auto-fix" / "FRD.md"


def documented_reasons() -> dict[str, str]:
    text = AUTOFIX_FRD.read_text(encoding="utf-8")
    # The block is detail inside Functional Requirements. AES602 closes the FRD
    # level-3 set to the template's three shapes, so this heading was relabelled
    # as a bold list item — markdownlint MD001 forbids an H4 directly under an
    # H2 and MD036 forbids bold-as-heading, leaving the list item as the only
    # form that keeps the label without inventing a section. Accept a heading
    # at any level or the list-item form, so this check tracks the reference
    # rather than the mark-up choice.
    match = re.search(
        r"^(?:#{2,4}\s+|- \*\*)Reason Code Reference(?:\*\*)?\s*$(.*?)(?=^##\s|\Z)",
        text,
        flags=re.MULTILINE | re.DOTALL,
    )
    if not match:
        return {}
    out = {}
    for row in match.group(1).splitlines():
        row = row.strip()
        if not row.startswith("|"):
            continue
        cells = [c.strip() for c in row.strip("|").split("|")]
        if len(cells) < 2 or cells[0] in {"Reason", ""} or set(cells[0]) <= {"-", ":"}:
            continue
        out[cells[0].strip("`")] = cells[1]
    return out


def check_fix_reasons() -> list[str]:
    documented = documented_reasons()
    if not documented:
        return [f"{rel(AUTOFIX_FRD)}: Reason Code Reference table not found"]
    errors = []
    expected = {}
    for item, outcome in (("SkipReason", "Skipped"), ("FailReason", "Failed")):
        variants = enum_variants("auto_fix/taxonomy_auto_fix_vo", item)
        if variants is None:
            errors.append(f"cannot locate enum {item} in the shared kernel")
            continue
        for variant in variants:
            expected[variant] = outcome
    for name, outcome in sorted(expected.items()):
        if name not in documented:
            errors.append(
                f"{rel(AUTOFIX_FRD)}: reason '{name}' is produced by the fix "
                "contract but missing from the Reason Code Reference"
            )
        elif documented[name] != outcome:
            errors.append(
                f"{rel(AUTOFIX_FRD)}: reason '{name}' is documented as "
                f"'{documented[name]}' but the contract makes it '{outcome}'"
            )
    for name in sorted(set(documented) - set(expected)):
        errors.append(
            f"{rel(AUTOFIX_FRD)}: reason '{name}' is documented but no longer "
            "exists in the fix contract"
        )
    return errors


# ─── 5. Performance NFR numbers ─────────────────────────────────────────────

PERF_DOCS = [
    ROOT / "PRD.md",
    ROOT / "README.md",
    ROOT / "crates" / "filesystem" / "FRD.md",
]
# file-count tier → the set of second-budgets that may be stated for it
PERF_BUDGETS = {"1000": {2, 5}, "10000": {10, 15}}
PERF_RE = re.compile(
    r"(1|10),?000 files\s*(?:in\s*)?(?:<|&lt;|under)\s*(\d+)\s*s", re.IGNORECASE
)


def check_performance() -> list[str]:
    errors = []
    for path in PERF_DOCS:
        text = path.read_text(encoding="utf-8")
        found: dict[str, set[int]] = {"1000": set(), "10000": set()}
        for tier, seconds in PERF_RE.findall(text):
            key = "1000" if tier == "1" else "10000"
            found[key].add(int(seconds))
        for tier, budgets in PERF_BUDGETS.items():
            if not found[tier]:
                continue  # the document does not state this tier at all
            unexpected = found[tier] - budgets
            if unexpected:
                errors.append(
                    f"{rel(path)}: {tier} files states "
                    f"{sorted(unexpected)}s, which matches neither the indexing "
                    f"nor the full-pipeline budget {sorted(budgets)}s"
                )
            if found[tier] != budgets:
                errors.append(
                    f"{rel(path)}: {tier} files states only "
                    f"{sorted(found[tier])}s; both scopes must be stated "
                    f"({sorted(budgets)}s: indexing, then full pipeline)"
                )
    return errors


# ─── 6. DESIGN.md H2 contract ───────────────────────────────────────────────

HOWTO_DESIGN = ROOT / "crates/shared/skills/aes-docs/references/HOW-TO-MAKE-DESIGN.md"
DOC_CONTRACT_FILE = ROOT / "crates/shared/src/doc_rules/taxonomy_doc_rules_constant.rs"


def normalize_h2(title: str) -> str:
    """Mirror the Rust `normalize_heading`: lowercase, drop punctuation, squeeze."""
    collapsed = re.sub(r"[^0-9a-zA-Z]+", " ", title.lower())
    words = collapsed.split()
    # Drop a leading list index, as the Rust side does, so "4. Colors" and
    # "Colors" compare equal.
    if words and words[0].isdigit():
        words = words[1:]
    return " ".join(words)


def howto_design_h2() -> set[str]:
    """H2 headings of the DESIGN.md template fenced in HOW-TO-MAKE-DESIGN.md."""
    text = HOWTO_DESIGN.read_text(encoding="utf-8")
    inside, body = False, []
    for line in text.splitlines():
        if line.lstrip().startswith("```"):
            if inside:
                break
            inside = True
            continue
        if inside:
            body.append(line)
    if not body:
        raise ValueError(f"{rel(HOWTO_DESIGN)} has no fenced DESIGN.md template")
    return {normalize_h2(m.group(1)) for m in re.finditer(r"^##\s+(.*?)\s*$", "\n".join(body), re.MULTILINE)}


def contract_design_h2() -> tuple[set[str], set[str]]:
    """(required, allowed) H2 sets of the DESIGN_DOC entry in DOC_HEADING_CONTRACTS."""
    text = DOC_CONTRACT_FILE.read_text(encoding="utf-8")
    entry = re.search(
        r"\(\s*DESIGN_DOC\s*,\s*&\[(.*?)\]\s*,\s*&\[(.*?)\]\s*,?\s*\)", text, re.DOTALL
    )
    if entry is None:
        raise ValueError(f"{rel(DOC_CONTRACT_FILE)} has no DESIGN_DOC contract entry")
    quoted = r'"([^"]*)"'
    return (
        {normalize_h2(s) for s in re.findall(quoted, entry.group(1))},
        {normalize_h2(s) for s in re.findall(quoted, entry.group(2))},
    )


def check_design_h2_contract() -> list[str]:
    """The enforced DESIGN.md H2 set must be exactly the shipped template's.

    AES605 treats the H2 set as closed, so a heading the contract names but the
    template does not is unreachable, and a heading the template emits but the
    contract does not name makes the shipped template fail its own linter.
    """
    try:
        template = howto_design_h2()
        required, allowed = contract_design_h2()
    except (OSError, ValueError) as exc:
        return [str(exc)]

    errors = []
    unreachable = sorted(required - template)
    if unreachable:
        errors.append(
            f"DESIGN_DOC requires H2 {unreachable}, absent from the "
            f"{rel(HOWTO_DESIGN)} template; a file copied from that template "
            f"cannot satisfy AES605"
        )
    off_template = sorted(template - required - allowed)
    if off_template:
        errors.append(
            f"DESIGN_DOC has no H2 entry for {off_template}, which the "
            f"{rel(HOWTO_DESIGN)} template emits; copying it fires AES605 h2_unexpected"
        )
    return errors


# ─── 7. FRD H3 contract ─────────────────────────────────────────────────────

HOWTO_FRD = ROOT / "crates/shared/skills/aes-docs/references/HOW-TO-MAKE-FRD.md"
# The requirement-heading shape, written with the template's own placeholders:
# `### FR-<FEATURENAME>-001: <Short imperative name>`. The number segment
# accepts `XXX` as well as digits because the template ships both the first
# numbered heading and four `-XXX` repeats.
FR_ID_H3 = re.compile(r"^###\s+FR-<[A-Za-z_]+>-(?:\d+|XXX):")


def howto_frd_h3() -> tuple[set[str], bool]:
    """(literal H3 titles, saw_requirement_heading) of the FRD template.

    The requirement-heading shape is a pattern, not a literal title, so it is
    reported as a flag rather than as a member of the literal set — the same
    split the Rust side makes in FRD_H3_TITLES plus fr_id_heading_re.
    """
    text = HOWTO_FRD.read_text(encoding="utf-8")
    inside, body = False, []
    for line in text.splitlines():
        if line.lstrip().startswith("```"):
            if inside:
                break
            inside = True
            continue
        if inside:
            body.append(line)
    if not body:
        raise ValueError(f"{rel(HOWTO_FRD)} has no fenced FRD.md template")
    titles, saw_fr = set(), False
    for m in re.finditer(r"^###\s+(.*?)\s*$", "\n".join(body), re.MULTILINE):
        title = m.group(1)
        if FR_ID_H3.match(m.group(0)):
            saw_fr = True
        else:
            titles.add(normalize_h2(title))
    return titles, saw_fr


def contract_frd_h3() -> tuple[set[str], bool]:
    """(FRD_H3_TITLES, saw_requirement_alternative) from the doc-rules constants."""
    text = DOC_CONTRACT_FILE.read_text(encoding="utf-8")
    entry = re.search(r"FRD_H3_TITLES[^=]*=\s*&\[(.*?)\]", text, re.DOTALL)
    if entry is None:
        raise ValueError(f"{rel(DOC_CONTRACT_FILE)} has no FRD_H3_TITLES constant")
    # A requirement heading is sanctioned by shape, not by title, so it is not
    # in FRD_H3_TITLES; the constants record that fact as a doc comment on the
    # constant. Read it rather than assuming, so deleting the comment is caught.
    comment = text[max(0, entry.start() - 1200) : entry.start()]
    saw_fr = "FR-<FEATURENAME>-NNN" in comment
    return {normalize_h2(s) for s in re.findall(r'"([^"]*)"', entry.group(1))}, saw_fr


def check_frd_h3_contract() -> list[str]:
    """The enforced FRD level-3 set must be exactly the shipped template's.

    AES602 closes the FRD level-3 set, so a heading the constants sanction but
    the template does not is unreachable, and a heading the template emits but
    the constants do not sanction makes the shipped template fail its own
    linter — the same defect the DESIGN H2 check above was added to catch.
    """
    try:
        template, template_saw_fr = howto_frd_h3()
        contract, contract_saw_fr = contract_frd_h3()
    except (OSError, ValueError) as exc:
        return [str(exc)]

    errors = []
    unreachable = sorted(contract - template)
    if unreachable:
        errors.append(
            f"FRD_H3_TITLES sanctions {unreachable}, absent from the "
            f"{rel(HOWTO_FRD)} template; a file copied from that template "
            f"cannot satisfy AES602"
        )
    off_template = sorted(template - contract)
    if off_template:
        errors.append(
            f"FRD_H3_TITLES has no entry for {off_template}, which the "
            f"{rel(HOWTO_FRD)} template emits; copying it fires AES602 h3_off_template"
        )
    if template_saw_fr and not contract_saw_fr:
        errors.append(
            f"the {rel(HOWTO_FRD)} template sanctions a requirement heading at "
            f"level 3 but FRD_H3_TITLES does not record it; copying it fires "
            f"AES602 h3_off_template"
        )
    return errors


CHECKS = (
    ("rule ranges", check_rule_ranges),
    ("markdown anchors", check_anchors),
    ("shared data model", check_data_model),
    ("auto-fix reason codes", check_fix_reasons),
    ("performance NFR", check_performance),
    ("DESIGN H2 contract", check_design_h2_contract),
    ("FRD H3 contract", check_frd_h3_contract),
)


def main() -> int:
    failed = 0
    for name, check in CHECKS:
        errors = check()
        if errors:
            failed += 1
            print(f"FAIL  {name}")
            for error in errors:
                print(f"      {error}")
        else:
            print(f"ok    {name}")
    if failed:
        print(f"\n{failed} doc-consistency check(s) failed")
        return 1
    print("\nall doc-consistency checks passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
