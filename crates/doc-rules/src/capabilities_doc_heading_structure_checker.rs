// PURPOSE: HeadingStructureChecker — AES605: the H1/H2 heading structure each
// recognised document must hold.
//
// The one capability that answers AES605. Every document with a registered H2
// contract must open with exactly one level-1 heading, carry every required
// level-2 section, and hold no level-2 heading outside the agreed set. The
// document set is this capability's own decision: a document with no
// registered contract has no heading shape to hold, so it is skipped rather
// than reported.
use std::collections::HashSet;

use shared_doc_rules::contract_doc_protocol::IDocHeadingProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;

use shared_doc_rules::utility_markdown_scanner::{
    blank_fenced, doc_h2_contract, heading_re, normalize_heading,
};

// ─── Block 1: Struct Definition ────────────────────────────

/// The AES605 auditor: H1/H2 heading structure plus verbatim line enforcement.
pub struct HeadingStructureChecker {}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl IDocHeadingProtocol for HeadingStructureChecker {
    /// Report every heading-count and heading-contract violation in the
    /// documents of *context*.
    fn audit_doc_heading(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            let name = context.document_name(document);
            // Only a document with a registered H2 contract has a heading
            // shape to hold; anything else is outside the document chain.
            let Some((required, allowed)) = doc_h2_contract(name) else {
                continue;
            };
            let mut own = Vec::new();
            self.check_doc_heading(&document.text, name, required, allowed, &mut own);
            if name == consts::AGENTS_DOC {
                self.check_agents_line_drift(
                    &document.text,
                    name,
                    &agents_template_lines(),
                    &mut own,
                );
            }
            context.stamp(document, &mut own);
            findings.extend(own);
        }
        findings
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl Default for HeadingStructureChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadingStructureChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// A document must carry exactly one H1, every required H2 from its
    /// template, and no H2 outside the agreed set. Level-3 headings are free.
    fn check_doc_heading(
        &self,
        text: &str,
        name: &str,
        required: &[&str],
        allowed: &[&str],
        findings: &mut Vec<DocFinding>,
    ) {
        let Some(re) = heading_re() else {
            return;
        };
        // Headings inside fenced code blocks are ignored: a shell comment
        // such as `# Tests (matches CI "Tests" job)` must not read as a heading.
        let prose = blank_fenced(text);
        let captures: Vec<_> = re.captures_iter(&prose).collect();
        let h1_count: usize = captures
            .iter()
            .filter(|c| c.get(1).is_some_and(|m| m.as_str().len() == 1))
            .count();
        if h1_count != 1 {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    0,
                    consts::RULE_CODE_DOC_STRUCTURE,
                    consts::DOC_STRUCTURE_VIOLATION_H1_COUNT,
                    format!("{name} opens with {h1_count} level-1 headings"),
                )
                .with_reason(
                    "The template names exactly one H1 at the top of the file, so a reader knows where the document starts.",
                    "Reduce the document to a single level-1 heading at the top.",
                ),
            );
        }
        let h2: Vec<String> = captures
            .iter()
            .filter(|c| c.get(1).is_some_and(|m| m.as_str().len() == 2))
            .map(|c| normalize_heading(c.get(2).map_or("", |m| m.as_str())))
            .collect();
        let missing: Vec<&str> = required
            .iter()
            .copied()
            .filter(|want| {
                let want = normalize_heading(want);
                !h2.iter()
                    .any(|title| title == &want || title.starts_with(&want))
            })
            .collect();
        if !missing.is_empty() {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    0,
                    consts::RULE_CODE_DOC_STRUCTURE,
                    consts::DOC_STRUCTURE_VIOLATION_H2_MISSING,
                    format!("{name} has no H2 heading for {}", missing.join(", ")),
                )
                .with_reason(
                    "Each of these level-2 sections is mandatory in the document's registered template, so the reader cannot find the section the reader expects.",
                    format!("Add the missing level-2 heading(s) {} to the document.", missing.join(", ")),
                ),
            );
        }
        // The H2 set is closed: a heading at level 2 that is not in the
        // required + allowed union must be demoted to a level-3 heading or removed.
        let permitted: Vec<String> = required
            .iter()
            .copied()
            .chain(allowed.iter().copied())
            .map(normalize_heading)
            .collect();
        let unexpected_h2: Vec<String> = h2
            .into_iter()
            .filter(|title| !permitted.iter().any(|a| title == a || title.starts_with(a)))
            .collect();
        if !unexpected_h2.is_empty() {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    0,
                    consts::RULE_CODE_DOC_STRUCTURE,
                    consts::DOC_STRUCTURE_VIOLATION_H2_UNEXPECTED,
                    format!(
                        "{name} carries H2 heading(s) outside the template: {}",
                        unexpected_h2.join(", ")
                    ),
                )
                .with_reason(
                    "The H2 set is closed: a heading at level 2 that is not in the required + allowed union breaks the document's registered shape.",
                    format!("Demote each unexpected H2 in {} to a level-3 heading or remove it.", unexpected_h2.join(", ")),
                ),
            );
        }
    }

    /// Check that AGENTS.md matches the template line for line, allowing only
    /// `{...}` placeholder slots to carry project-specific content.
    ///
    /// Strict enforcement: every line the template fixes must appear verbatim
    /// and in order. A `{...}` placeholder slot is the only place a project may
    /// write its own text. Anything else — a renamed heading, a reworded
    /// bullet, a reordered section, a dropped rule — is drift.
    fn check_agents_line_drift(
        &self,
        text: &str,
        doc_name: &str,
        template: &[&str],
        findings: &mut Vec<DocFinding>,
    ) {
        let doc_lines: Vec<&str> = text.lines().collect();

        let has_frontmatter = doc_lines.first().is_some_and(|l| l.trim() == "---");
        if !has_frontmatter {
            findings.push(
                DocFinding::new_with_line(
                    doc_name,
                    1,
                    consts::RULE_CODE_DOC_STRUCTURE,
                    consts::DOC_STRUCTURE_VIOLATION_FRONTMATTER_MISSING,
                    "AGENTS.md must have YAML frontmatter (---) at the start".to_string(),
                )
                .with_reason(
                    "The frontmatter declares the tool contract (trigger, description) that consumers rely on to decide when the file applies.",
                    "Add YAML frontmatter (---) at the top of AGENTS.md.",
                ),
            );
        }

        let first_line = doc_lines.first().map_or("", |v| *v);
        let looks_like_agents = first_line.trim() == "---" || first_line.starts_with("# ");
        if !looks_like_agents {
            return;
        }

        if template.is_empty() {
            return;
        }

        // A placeholder is a `{...}` slot. That is the convention the
        // reference file uses, and it is the only place a project may write
        // its own text. Every other non-blank line is fixed text that must
        // appear verbatim, in order.
        //
        // The comparison is scoped per section. A template section holds
        // placeholder lines and their wrapped continuations interleaved with
        // fixed lines; comparing the whole document as one sequence makes a
        // legitimate fill desynchronise every later line. Section by section,
        // a fill only consumes its own slot.
        let doc_sections = split_sections(&doc_lines);
        let template_sections = split_sections(template);

        // Every section the document declares must exist in the template, so a
        // rename cannot slip through a section that holds no fixed lines.
        for (name, _) in &doc_sections {
            if !template_sections.iter().any(|(key, _)| key == name) {
                findings.push(
                    DocFinding::new_with_line(
                        doc_name,
                        0,
                        consts::RULE_CODE_DOC_STRUCTURE,
                        consts::DOC_STRUCTURE_VIOLATION_LINE_DRIFT,
                        format!(
                            "AGENTS.md has the section `## {name}`, which the template does not declare"
                        ),
                    )
                    .with_reason(
                        "The template fixes the section set, so an undeclared section is drift from the reference shape.",
                        "Remove or rename the section `## {name}` to one the template declares.",
                    ),
                );
            }
        }

        for (name, want_lines) in &template_sections {
            let Some((_, have_lines)) = doc_sections.iter().find(|(key, _)| key == name) else {
                // The template fixes every heading, so a section the document
                // does not carry under this exact name has been renamed or
                // dropped. `check_doc_heading` allows a title suffix for other
                // documents, so state the rename here where it is drift.
                findings.push(
                    DocFinding::new_with_line(
                        doc_name,
                        0,
                        consts::RULE_CODE_DOC_STRUCTURE,
                        consts::DOC_STRUCTURE_VIOLATION_LINE_DRIFT,
                        format!(
                            "AGENTS.md is missing the template section `## {name}`, or renamed it"
                        ),
                    )
                    .with_reason(
                        "The template fixes this heading, so a renamed or dropped section drifts from the reference shape.",
                        "Restore the template section `## {name}` with its exact heading.",
                    ),
                );
                continue;
            };
            // A placeholder may wrap onto the next line. When a line opens a
            // brace it does not close, the following lines up to the closer are
            // part of the slot, so they carry no fixed text to match.
            let want_placeholders = placeholder_lines(want_lines);
            let want_strict: Vec<&str> = want_lines
                .iter()
                .enumerate()
                .filter(|(idx, l)| !l.trim().is_empty() && !want_placeholders.contains(idx))
                .map(|(_, l)| *l)
                .collect();
            let mut cursor = 0usize;
            for want in want_strict {
                let start = cursor.min(have_lines.len());
                match have_lines[start..].iter().position(|have| *have == want) {
                    Some(offset) => cursor = start + offset + 1,
                    None => {
                        let offending = have_lines
                            .get(start)
                            .copied()
                            .or_else(|| have_lines.last().copied())
                            .unwrap_or("");
                        findings.push(
                            DocFinding::new_with_line(
                                doc_name,
                                0,
                                consts::RULE_CODE_DOC_STRUCTURE,
                                consts::DOC_STRUCTURE_VIOLATION_LINE_DRIFT,
                                format!(
                                    "AGENTS.md section `{name}` does not match the template: expected the line '{want}' (or a {{...}} placeholder), but found '{offending}'"
                                ),
                            )
                            .with_reason(
                                "Every line the template fixes must appear verbatim and in order; only {{...}} placeholder slots may differ.",
                                "Replace '{offending}' in section `{name}` with the template line '{want}' or a {{...}} placeholder.",
                            ),
                        );
                        cursor = start + 1;
                    }
                }
            }
        }

        if !doc_lines.iter().any(|l| l.trim_start().starts_with("# ")) {
            findings.push(
                DocFinding::new_with_line(
                    doc_name,
                    0,
                    consts::RULE_CODE_DOC_STRUCTURE,
                    consts::DOC_STRUCTURE_VIOLATION_LINE_DRIFT,
                    "AGENTS.md must open with a level-1 heading naming the project".to_string(),
                )
                .with_reason(
                    "The H1 names the project and anchors the document for a reader at the top of the file.",
                    "Add a level-1 heading naming the project at the top of AGENTS.md.",
                ),
            );
        }
        // A project may name itself freely, so only the shape is fixed: one
        // H1, non-empty, carrying no `{...}` slot left unfilled.
        for line in doc_lines
            .iter()
            .filter(|l| l.trim_start().starts_with("# "))
        {
            let title = line.trim_start().trim_start_matches('#').trim();
            if title.is_empty() {
                findings.push(
                    DocFinding::new_with_line(
                        doc_name,
                        0,
                        consts::RULE_CODE_DOC_STRUCTURE,
                        consts::DOC_STRUCTURE_VIOLATION_LINE_DRIFT,
                        "AGENTS.md H1 has no title".to_string(),
                    )
                    .with_reason(
                        "A level-1 heading with no title leaves the document unnamed at the top.",
                        "Fill in the H1 title of AGENTS.md.",
                    ),
                );
            }
        }
    }
}

// ─── Block 3: Module Helpers ───────────────────────────────

/// The AGENTS.md template, pasted verbatim from
/// `crates/shared/skills/aes-docs/references/HOW-TO-MAKE-AGENTS.md`.
///
/// It lives here rather than being read at runtime so the contract cannot go
/// missing when the skill is absent, and so this file and the reference stay
/// diffable against each other. Every `{...}` slot is the one place a project
/// writes its own text; every other line must appear verbatim.
pub const AGENTS_TEMPLATE: &str = r#"---
trigger: always
description: "{Project} operational guide."
---
# {Project Name}

## User Context

- Preferences: {response style, e.g., concise, technical, direct}

### Autonomy

Operate as a YOLO-reversible agent. Allowed without asking: read code,
logs, status, diffs, test output, CI output; create or update isolated
branches and worktrees; install or sync dependencies from lockfiles;
run tests, linters, type checkers, builds, documentation checks; apply
reversible fixes to code, tests, docs, formatting, imports, comments;
commit, push to feature branches, open or update PRs when required
checks pass; poll PR status, CI status, merge queue, and failed check
logs; fix CI failures caused by the change, push follow-ups, repeat
validation; merge the PR after required checks pass.

### Scope and Precedence

- Local project guide controls project-specific conventions.
- Global guide (this `User Context` block, mirrored in
  {global agent guide path, e.g., ~/.qwen/QWEN.md}) controls
  autonomy, safety, and general behavior.
- Destructive or out-of-scope actions — force-push, `git reset
  --hard`, `rm -rf` on something outside the working tree, dropping
  a database, messaging a third party — still need explicit approval.

### Work Loop

1. Read the local project guide.
2. Identify the smallest correct change.
3. Work in an isolated worktree under {worktree-dir}/, never on
   {default branch}.
4. Run the project validation commands from `Commands`.
5. Fix failures caused by the change: read error output, reproduce
   locally when possible, apply the smallest fix, rerun the failed
   command, run related checks before pushing.
6. Commit with the convention defined by `Git Workflow`.
7. Push and open or update the PR.
8. Poll PR and CI status until terminal state. Treat pending checks
   as active work.
9. If CI fails: read failed output, fix, commit, push, poll again.
   Repeat until checks pass or a blocker requires human action.
10. If a merge conflict appears: resolve it in the isolated worktree
    when reversible and within scope.
11. If the repository uses a merge queue: poll the queue and fix
    rejections the same way as a CI failure.
12. When required checks pass, merge using the method from `Git
    Workflow`. If undefined and squash is allowed, use squash.
13. Verify the merged state from GitHub.

### Command Policy

Read-only inspection and PR/CI polling commands are always allowed:

```bash
git diff
git log --oneline -5
gh pr status
gh pr checks
gh pr checks --watch
gh pr view --json state,mergeStateStatus,mergeable,statusCheckRollup
gh run list --limit 5
```

## Session Start

Check state:

```bash
git status
git branch --show-current
git worktree list
git fetch origin {default branch}
```

Continue only from the correct {worktree-dir}/<branch-name>. If state
is missing or stale, ask before destructive changes. If the active
worktree is dirty, preserve relevant changes and stash unrelated
ones with `git stash push -u -m "agent-auto-stash"`.

## Runtime

- Language: {Language + pinned version}.
- Environment: {Where it lives and how it is created}.
- Artifacts: {Where they go. Never use /tmp for build output a
  reviewer must find}.

```bash
{version probe, e.g., python --version}
{env setup, e.g., export UV_PROJECT_ENVIRONMENT="$HOME/.local/share/<project>/venv" && uv sync}
```

## Quick Facts

INPUT  = {artifact + what it carries}
OUTPUT = {artifact + locked spec values, e.g., format, size, rate}

## Pipeline

{A} → {B} → {C} → {D}
{one word per stage}

## Git Workflow

Every change must use a worktree under
{worktree-dir}/<branch-name>. Do not work directly on {default-branch}.
Do not switch branches (no `git checkout`/`git switch`); use `git worktree add`
so each branch lives in its own directory. PR to {default-branch} when done.
Exceptions require explicit user approval.

Branch prefixes: `<type>/`, ...

```bash
git worktree add -b {branch-name} {worktree-dir}/{branch-name} origin/{default branch}
cd {worktree-dir}/{branch-name}

# Run the checks under Commands, then:
git add .
git commit -m "{type}: {short description}"
git push -u origin {branch-name}

gh pr create --base {default branch} --head {branch-name} \
  --title "{type}: {short description}" \
  --body "$(cat <<'PRBODY'
What changed:
PRBODY
)"
```

After merge:

```bash
cd ../..
git worktree remove {worktree-dir}/{branch-name}
git branch -d {branch-name}
```

Merge strategy: {which prefixes squash, which rebase onto }.

Merge and verification commands:

```bash
gh pr merge --{merge method}
gh pr view --json state,mergedAt,mergeCommit
```

Do not bypass branch protection, required checks, review requirements,
or merge restrictions. Do not force-push to protected branches. Do
not switch branches between active tasks; use separate worktrees.

## Commands

```bash
# Tests
{whole-workspace test command}                      # what it covers
{single-package test command}                       # one unit
{single-file test command}                          # one file

# Lint / types / architecture —
{formatter/linter}                                  # matches ci.yml {job name}
{type checker, exact config-file flags}
{architecture scanner}
{dry-run variant, if the fixer is destructive}
```

## Guided Skills

Use `.agents/skills/` when a task matches a guided workflow. Read the
matching skill before generating structural code.

## Definition of Done

A change is done when:

- Work happened inside the correct {worktree-dir}/<branch-name>.
- Tests pass for touched units.
- Linter, type checker, and architecture scanner pass for touched paths.
- PR title and body follow conventions.
- A PR that merges a fix updates every invalidated backlog row in the
  same PR, then merges to {default branch}.
- Required CI checks pass and the PR is merged into {default branch}.
- Merged state is verified from GitHub (e.g. `gh pr view --json
  state,mergedAt,mergeCommit`).

A task is not complete at commit, local checks, or PR creation.

When a failure is unrelated to the current task, note it and continue
if the change remains safe. When a failure requires destructive
cleanup, secret access, production access, or out-of-scope behavior,
stop and ask.

## Writing Style

Use this section when editing prose, docs, PR descriptions, or release
notes. Do not apply it to code identifiers, commands, or config keys.

- Lead with the point. Use plain, active verbs. Keep concrete facts
  (names, dates, numbers, mechanisms).
- No invented claims, weasel attribution, throat-clearing openers,
  binary contrasts, or dramatic endings. Name the source or cut the claim.
- Use complete sentences, no emoji by default, code formatting for
  commands and variables. Vary rhythm only when it helps.

## Related Documents

- {link plus one line on what that document answers; repeat this bullet once per related document}.

---"#;

/// Split the template into its lines, dropping the fence that wraps it.
fn agents_template_lines() -> Vec<&'static str> {
    AGENTS_TEMPLATE.lines().collect()
}

/// Every line that belongs to a `{...}` slot, including slots that wrap across
/// physical lines.
///
/// The template wraps long placeholders, so a slot can start on one line and
/// close on the next. A line inside such a slot fixes no text: it is entirely
/// the project's to write. Walking the lines once and tracking brace depth
/// keeps the caller a plain set lookup.
fn placeholder_lines(lines: &[&str]) -> HashSet<usize> {
    let mut inside = HashSet::new();
    let mut depth = 0usize;
    for (idx, line) in lines.iter().enumerate() {
        let opens = line.matches('{').count();
        let closes = line.matches('}').count();
        if opens > closes {
            // The slot is open before this line, so this line starts inside it.
            inside.insert(idx);
            depth += opens - closes;
        } else if depth > 0 {
            // Still open: this line only closes it.
            inside.insert(idx);
            depth = depth.saturating_sub(closes);
            depth = depth.saturating_sub(opens);
        } else if closes > opens || (opens > 0 && closes > 0) {
            // Self-contained slot.
            inside.insert(idx);
        }
        if depth == 0 {
            // Any extra closes beyond the open one end the slot; nothing to do.
        }
    }
    inside
}

/// Split a document into `H2 heading -> body lines` sections.
///
/// The H1 and any preamble before the first H2 become the section named `""`,
/// so frontmatter and the title are compared too.
fn split_sections<'a>(lines: &'a [&'a str]) -> Vec<(String, Vec<&'a str>)> {
    let mut sections: Vec<(String, Vec<&str>)> = Vec::new();
    for line in lines {
        if let Some(title) = line.strip_prefix("## ") {
            sections.push((title.trim().to_string(), Vec::new()));
            continue;
        }
        match sections.last_mut() {
            Some((_, body)) => body.push(line),
            None => sections.push((String::new(), vec![line])),
        }
    }
    sections
}
