---
name: author-skill-md
description: Authors SKILL.md files with best practices. Use when creating, structuring, validating skills.
metadata:
  tags:
    [skill, agent, authoring, skill-writing, skill-creation, skill-validation]
  triggers:
    - "create skill"
    - "write skill"
    - "improve skill"
    - "validate skill"
    - "check skill"
    - "audit skill"
  dependencies: []
  related: [skill-manager]
---
# Authoring Skills

## When to Use

- Asked to create, write, structure, improve, validate, or audit a Skill / `SKILL.md` /
  agent skill instructions — in any harness.
- Deciding what goes in the body vs a `references/` file, or how to word a `description:`.
- Don't use for: installing/enabling/disabling existing skills (see `skill-manager`), or
  for Hermes profile skill filtering (see `hermes-profiles`).
- **Committing a skill into the upstream `hermes-agent` repo** (or any vendored repo with
  its own review standards): load `references/hermes-upstream-repo.md` first — it adds
  mandatory fields, tier rules, tests, and docs regeneration on top of the rules here.

## Quick start

Create a single SKILL.md with valid frontmatter, then add concise markdown body.

```yaml
---
name: your-skill-name
description: What it does and when to use it. Third person. Include trigger terms.
---
```

## Core rules

1. **Be concise** — The model is already smart. Only add what it doesn't know. Challenge every token.
2. **Set degrees of freedom** — Match specificity to task fragility:
   - High freedom (text steps): multiple valid approaches, context-dependent
   - Medium freedom (parameterized scripts): preferred pattern exists, some variation OK
   - Low freedom (exact commands): fragile operations, strict sequence required
3. **Progressive disclosure** — SKILL.md is a table of contents. Link to detail files. Keep references ONE level deep.
4. **Consistent terminology** — Pick one term per concept. Never mix synonyms.
5. **Model-agnostic** — Write instructions that work across models. Avoid over-explaining for powerful models or under-specifying for lighter ones.
6. **Budget for permanent cost** — every `description:` is injected into the skill list in
   every session, so the number of skills and the length of their descriptions are paid for on
   every turn; `references/*.md` cost nothing until they are opened. Move anything
   task-conditional out of the body. The body loads when the skill loads, so it is paid
   per-skill-use; references are paid per-section-read.

## Frontmatter

### Allowed fields (exhaustive for upload targets)

Only these keys pass the hosted Skills-API validator. Any other key causes upload failure.
Self-hosted harnesses are looser — e.g. in-repo Hermes skills *require* `version`,
`author`, `license`, `platforms` and `metadata.hermes.{tags, related_skills}` on top of
`name` + `description` (`references/hermes-upstream-repo.md`). Match the field set the
target's validator actually enforces; survey 2-3 sibling skills in the tree first.


| Field         | Required | Constraints                                                                                                                                 |
| --------------- | ---------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | Yes      | Max 64 chars. Lowercase`a-z`, `0-9`, `-` only. No leading/trailing hyphen. No `--`. No XML tags. No reserved words ("anthropic", "claude"). |
| `description` | Yes      | Max 1,024 chars. Non-empty. No`<` or `>` characters. No XML tags.                                                                           |
| `metadata`    | yes      | Arbitrary key-value pairs for your own tracking.                                                                                            |

### Naming

- Gerund form preferred: `processing-pdfs`, `analyzing-data`, `managing-deployments`
- Valid: `pdf-processing`, `process-pdfs`
- Invalid: `-leading`, `trailing-`, `double--hyphen`, `Helper`, `utils`, `tools`
- **`name:` must equal the directory name exactly.** Loaders key discovery on the folder and
  cross-check the frontmatter; a mismatch means the skill is either rejected or loaded under a
  name no `related_skills`/`skill_view` call can resolve. Renaming a skill means renaming the
  folder (`git mv`) and grepping the pack for inbound mentions — not editing one field.
- **Skills are direct children of a category dir:** `<root>/<category>/<skill>/SKILL.md`. The
  loader walks exactly one level, so a skill nested one dir deeper, or sitting at the root of
  the skills tree, is invisible rather than broken-looking.

### Description

- Third person: "Extracts text from PDFs" (not "I can help" / "You can use")
- Include WHAT + WHEN + trigger terms
- No angle brackets (`<`, `>`) anywhere in the string
- Be specific. Avoid "Helps with documents" or "Processes data"

## Structure

### One SKILL.md per skill

Each skill directory contains exactly ONE `SKILL.md` at the root. Nested `SKILL.md` files are rejected on upload (Skills API / claude.ai). The only exception is loading via filesystem in Claude Code.

### Simple skill

```
my-skill/
└── SKILL.md
```

### Complex skill (progressive disclosure)

```
my-skill/
├── SKILL.md              # Overview + links (under 500 lines)
├── reference.md          # API/method details
├── examples.md           # Input/output pairs
└── scripts/
    └── validate.py       # Executed, not loaded into context
```

### Domain-organized skill

```
my-skill/
├── SKILL.md
└── reference/
    ├── domain-a.md
    ├── domain-b.md
    └── domain-c.md
```

All reference files link directly from SKILL.md. Never nest references deeper than one level.

## Workflow pattern

For multi-step tasks, provide a checklist:

```
Task Progress:
- [ ] Step 1: [action]
- [ ] Step 2: [action]
- [ ] Step 3: [validate]
- [ ] Step 4: [execute]
- [ ] Step 5: [verify]
```

Add feedback loops: run validator → fix errors → repeat. Only proceed when validation passes.

## Content guidelines

- **No time-sensitive info** — Use "Current method" + collapsed "Old patterns" section
- **Provide defaults, not menus** — "Use pdfplumber" not "You can use pypdf, or pdfplumber, or PyMuPDF..."
- **Forward slashes only** — `scripts/helper.py` not `scripts\helper.py`
- **Examples over descriptions** — Show input/output pairs for style-dependent output
- **Conditional branching** — "Creating? → Workflow A. Editing? → Workflow B"

## Writing quality

A skill exists to make the agent's process predictable — it reliably follows the same useful
discipline. So:

1. **Optimize for process predictability** — if a line does not change behavior, cut it.
2. **Choose the right context load** — description is paid every turn; details go in the body or
   a linked reference (see core rule 6).
3. **End steps with completion criteria** — checkable and, when it matters, exhaustive: "every
   modified file accounted for" beats "summarize changes".
4. **Co-locate rules with the concept they govern** rather than collecting them in a preamble.
5. **Use strong leading words** ("root cause", "regression test", "tight loop") instead of long
   repeated explanations.
6. **Prune duplication and no-ops** — "be careful" and "use best practices" change nothing;
   replace them with a checkable criterion or delete them. When adding a rule, delete the old
   wording it replaces so the skill never accumulates sediment.

## Scripts and code

- Handle errors explicitly (don't defer to the model)
- Don't expect the model to re-write a parser or non-trivial logic on every call — ship
  `scripts/helper.py` and reference it by path.
- Justify all constants: `TIMEOUT = 30  # HTTP requests complete within 30s`
- State intent: "Run script.py" (execute) vs "See script.py" (read as reference)
- List dependencies explicitly before usage
- Use fully qualified MCP tool names: `ServerName:tool_name`

## Pre-publish checklist

- [ ]  Frontmatter uses ONLY allowed keys (name, description, metadata)
- [ ]  Name: lowercase, hyphens, no leading/trailing `-`, no `--`, no reserved words
- [ ]  Description: third person, specific, no `<` or `>`, includes triggers
- [ ]  Exactly one SKILL.md at skill root (no nested SKILL.md)
- [ ]  Body under 500 lines; overflow in linked files
- [ ]  References one level deep only
- [ ]  Consistent terminology throughout
- [ ]  Concrete examples (not abstract)
- [ ]  Workflows have clear sequential steps + feedback loops
- [ ]  Scripts handle errors; no magic numbers
- [ ]  Dependencies listed; paths use forward slashes
- [ ]  No time-sensitive content in main body
- [ ]  Tested with all target models and real scenarios

## References

- `references/hermes-upstream-repo.md` — **load whenever the skill is committed source in the
  upstream `hermes-agent` repo** (or a similarly reviewed tree). Adds on top of everything here:
  the bundled vs `optional-skills/` tier decision, the extra required frontmatter (`version`,
  `author: Real Name (handle), Hermes Agent`, `license`, `platforms:` gating table), the ≤60-char
  description rule (the skill index truncates at 57), the required `tests/skills/` test file, and
  the docs-generator step with its scope discipline.
