# Contributing a SKILL.md to the hermes-agent repo

Load when the skill being written is **committed source in
`github.com/NousResearch/hermes-agent`** (`skills/…` or `optional-skills/…`), not a
personal skill. Everything generic lives in `SKILL.md`; this file is only what the
upstream repo additionally demands. Source of truth for the standards is the repo's
`AGENTS.md` § "Skill authoring standards (HARDLINE)" — reviewers reject violations, so
meeting them up front is cheaper than a salvage pass.

## Two trees, two mechanisms

| Target | Path | Tool |
|---|---|---|
| User-local (personal, not shared) | `~/.hermes/skills/<category>/<name>/SKILL.md` | `skill_manage(action='create')` |
| In-repo (committed, shipped) | `skills/<category>/<name>/` or `optional-skills/<category>/<name>/` | `write_file` + `git add` |

`skill_manage(action='create')` writes to `~/.hermes/skills/`, NEVER the repo tree — using
it for an in-repo skill is the classic mistake. `skill_manage(action='patch')` does work on
in-repo skills. In this pack, `agents-arwaky/skills/` is the analogous committed tree: same
rule, `write_file` + git, and the category-dir requirement is the same.

## Decide the tier first: bundled vs optional

- **Bundled (`skills/<category>/`)** — daily-driver behavior, broadly useful across many user
  types, low footprint. Hard bar: you can say "a user will load this in 5+ sessions per month"
  with a straight face.
- **Optional (`optional-skills/<category>/`)** — niche, vertical-specific (blockchain, gaming,
  finance, one app), recurring-job/task skills, or anything heavy. Installed via
  `hermes skills install official/<category>/<skill>`.

**When in doubt, optional.** Promoting later is easy; demoting is churn. "Would be useful to
anyone who ever needs this" is an optional-tier argument, not a bundled one.

Pick the category by what the tool IS, not what it feels like (an AI-agent CLI goes in
`autonomous-ai-agents/` even if it "feels productivity"). Confirm existing categories with
`search_files(pattern='*', target='files', path='skills')` and don't invent new top-level
categories casually.

**No router / index / hub skills.** A skill whose core content is a routing table pointing at
sibling skills adds an indirection hop and duplicates the siblings' own `When to Use` triggers.
If the skill would be empty without "load skill X instead" pointers, don't write it.

## Required frontmatter (repo shape)

Validator source of truth: `tools/skill_manager_tool.py::_validate_frontmatter`. It enforces:
`---` as the first bytes (no leading blank line or BOM), a closing `\n---\n`, a YAML mapping,
`name` present, `description` present, non-empty body. The validator is NOT the standard —
review is stricter:

```yaml
---
name: my-skill-name               # lowercase, hyphens, ≤64 chars (MAX_NAME_LENGTH)
description: Concise capability statement, under sixty chars.
version: 0.1.0                    # semver; new skills start at 0.1.0
author: Real Name (github-handle), Hermes Agent
license: MIT
platforms: [linux, macos, windows]   # audit, don't guess — see below
metadata:
  hermes:
    tags: [Short, Descriptive, Tags]
    related_skills: [other-in-repo-skill]
---
```

- **`description` ≤ 60 characters.** One sentence, ends with a period. State the capability,
  not the implementation; don't repeat the skill name; no marketing words. The system prompt's
  skill index truncates at 57 chars + "…", so the trigger must be self-contained in that
  window. Wrap in double quotes if it contains a `:` (otherwise YAML parses a mapping and the
  docs generator crashes); quotes don't count toward the 60.
  Good: `Track named companies for material news with cited digests.`
  Bad: `Use when a user asks to monitor named competitors or companies for product launches,
  pricing changes, funding, …` (240 chars — rejected in review).
- **`author` credits the human first**, with `Hermes Agent` as secondary collaborator:
  `Ben Barclay (benbarclay), Hermes Agent`. Never `author: Hermes Agent` alone for a
  contributed skill — even (especially) when an agent drafted the text.
  Maintainer-authored skills: `Teknium (teknium1), Hermes Agent`.
- **`related_skills`** — every entry must resolve to an existing in-repo skill at the same tree
  state as the PR (not planned, not another PR, not `~/.hermes/skills/`). Verify each with
  `search_files(pattern='<name>', target='files', path='skills')` and `optional-skills/`.

## Platform gating: audit, don't trust

`platforms:` gates loading by host OS. Set it from what the prose and scripts actually invoke:

| Skill uses only… | `platforms:` |
|---|---|
| Hermes tools + stdlib Python + cross-platform CLIs | `[linux, macos, windows]` |
| bash pipelines, `grep`/`awk`/`sed` chains, heredocs | `[linux, macos]` |
| `osascript`, `defaults`, `pmset` | `[macos]` |
| `apt`/`systemctl`/`/proc` | `[linux]` |

POSIX-only signals to search for in `scripts/`: `fcntl`, `termios`, `pty`, `os.fork`,
`os.killpg`, `signal.SIGKILL`, `os.kill(pid, 0)` liveness checks, hardcoded `/tmp`, `/proc`,
`/etc`. Default posture: fix cross-platform first (`tempfile.gettempdir()`, `pathlib.Path`,
`psutil.pid_exists`); gate narrower only when the dependency is genuinely platform-bound, and
say why in `## Pitfalls`.

## Size and structure

- Full SKILL.md ≤ 100,000 chars is *enforced* (`MAX_SKILL_CONTENT_CHARS`); the review target is
  ~100 lines for a simple skill, ~200 for a complex one. Peer skills sit at 8–14k chars.
- Section order: intro (what it does, what it doesn't), `## When to Use` (triggers +
  "Don't use for:"), `## Prerequisites`, `## How to Run`, `## Quick Reference`,
  `## Procedure` (numbered, each step with a checkable completion criterion), `## Pitfalls`,
  `## Verification`. When to Use + actionable body + Pitfalls + Verification are the minimum.
- **Reference Hermes tools, not raw shell.** Name `terminal`, `read_file`, `write_file`,
  `patch`, `search_files`, `web_search`, `web_extract`, `browser_navigate`, `vision_analyze`,
  `delegate_task`, `cronjob` in backticks. Do not name utilities the agent already wraps
  (`grep` → `search_files`, `cat` → `read_file`, `sed`/`awk` → `patch`, `find`/`ls` →
  `search_files target='files'`). A CLI-wrapper skill frames invocations as
  `terminal(command="…", timeout=…)`; bare shell prose is a review-blocking non-conformance.
- **Never machine-local paths.** Repo-relative only (`skills/…`, `tools/skill_manager_tool.py`).
  A `/home/<you>/…` in a committed skill breaks every other user and is an instant flag.
- Cut marketing intros, "Setup Check" no-ops, and re-explanations of env vars already listed in
  Prerequisites. When adding a rule, delete the old wording it replaces (no sediment).

## Tests and docs (required for repo skills)

1. **Tests** at `tests/skills/test_<skill>_skill.py` — stdlib + pytest + `unittest.mock` only,
   no live network. Run `scripts/run_tests.sh tests/skills/test_<skill>_skill.py -q`. The
   generic `tests/tools/test_skill_manager_tool.py` passing proves nothing about YOUR skill.
2. **Docs regen:** `python website/scripts/generate-skill-docs.py`, then apply scope discipline
   — the generator rewrites EVERY auto-gen page. `git checkout --` everything that isn't yours;
   the final diff must show only your SKILL.md, your one per-skill docs page, a one-line
   catalog row, and a one-line `website/sidebars.ts` insertion (verify with
   `search_files(pattern='<your-slug>', path='website/sidebars.ts')` — exactly one hit, or the
   page is an orphan). No regen = orphan skill with no docs page; blind regen = a ballooned
   diff full of other skills' drift. Both directions are wrong.
3. **`.env.example`** (only if the skill needs new env vars): one clearly delimited commented
   block; touch nothing else in the file.

## Workflow

1. Survey peers in the target category (`search_files(target='files')`), read 2–3 peer
   SKILL.md files to match tone and structure. Prefer extending an existing skill over creating
   a narrow sibling.
2. Decide tier and category. When in doubt, optional — and ask before pushing rather than
   defaulting.
3. Draft with `write_file` to `skills/<category>/<name>/SKILL.md` (or `optional-skills/…`).
4. Validate locally:
   ```python
   import yaml, re, pathlib
   content = pathlib.Path("skills/<category>/<name>/SKILL.md").read_text()
   assert content.startswith("---")
   m = re.search(r'\n---\s*\n', content[3:])
   fm = yaml.safe_load(content[3:m.start()+3])
   assert "name" in fm and "description" in fm
   assert len(fm["description"]) <= 60, f"description {len(fm['description'])} chars — hardline is 60"
   assert fm["description"].endswith(".")
   assert "platforms" in fm
   assert len(content) <= 100_000
   ```
   Also verify every `related_skills` entry exists in-repo.
5. Add tests + regen docs (previous section).
6. `git add` + commit on the active branch; open a PR. Re-run the docs generator whenever
   frontmatter changed — in-repo skills are source, not runtime state.
7. **The current session's skill loader is cached:** `skill_view` / `skills_list` will not see
   the new skill until a new session. Expected, not a bug.

## Pitfalls (repo-specific)

1. Using `skill_manage(action='create')` for an in-repo skill (writes to `~/.hermes/skills/`).
2. Trusting validator limits as the standard: it allows 1024-char descriptions and ignores
   `platforms:`, author format, tests, and docs. Review rejects >60 and flags the rest.
3. `author: Hermes Agent` alone on a contributed skill.
4. Leading whitespace/BOM before `---` — validation fails.
5. Generic description, or the trigger buried past char 57.
6. `related_skills` pointing at skills that don't exist in-repo.
7. Duplicating a peer instead of extending it — survey the category first.
8. Expecting the current session to see the new skill.

## Verification checklist (repo additions only)

- [ ] Tier decided deliberately (bundled bar: 5+ sessions/month; else `optional-skills/`)
- [ ] File at `skills/<category>/<name>/SKILL.md` or `optional-skills/<category>/<name>/SKILL.md`
- [ ] `name`, `description`, `version`, `author`, `license`, `platforms`,
      `metadata.hermes.{tags, related_skills}` all present
- [ ] Description ≤ 60 chars, one sentence, period-terminated, no marketing words
- [ ] `author` credits the human contributor first
- [ ] `platforms:` audited against actual prose/scripts, not copied from a sibling
- [ ] Every `related_skills` entry resolves in-repo
- [ ] Commands framed through Hermes tools; no machine-local paths anywhere
- [ ] Tests at `tests/skills/test_<skill>_skill.py` pass under `scripts/run_tests.sh`
- [ ] Docs regenerated with scope discipline; sidebar has exactly one entry for the slug
- [ ] `git add` + commit on the intended branch; PR opened
