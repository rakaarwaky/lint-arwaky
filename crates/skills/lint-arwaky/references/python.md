# Lint Arwaky — Python

The shared command reference, global options, MCP tools, workflow, exit codes and the full
AES101–AES506 fix table live in `../SKILL.md` and `rule-routing.md`. This file keeps only what
differs for Python.

## Adapters & external tooling

`lint-arwaky-cli install` installs the external adapters; `lint-arwaky-cli adapters` lists the
active ones (Ruff, Mypy, Radon, Bandit, etc.). `get_config` accepts `"language": "python"`.

| Adapter | Standalone command        | What it covers                  |
| --------- | --------------------------- | --------------------------------- |
| Ruff    | `ruff check src/`         | lint + style gate               |
| Mypy    | `mypy src/`               | static type checking            |
| Radon   | `lint-arwaky-cli quality modules/`  | complexity / quality metrics  |
| Bandit  | `lint-arwaky-cli security modules/` | security issues             |
| pip-audit-style CVE check | `lint-arwaky-cli dependencies modules/` | library vulnerabilities |

`lint-arwaky-cli external modules/` runs only the external linters (ruff).

## Workspace layout & `--member`

Python workspaces are laid out as `workspaces-bad/modules/<module>/src/…`, so path examples use
`modules/`, and `--member` targets a **module** name.

```bash
lint-arwaky-cli scan workspaces-bad/modules                  # basic scan (text)
lint-arwaky-cli scan workspaces-bad/modules --format json
lint-arwaky-cli scan workspaces-bad/modules --filter AES201  # by rule code
lint-arwaky-cli orphan modules/ --member animator            # single member
lint-arwaky-cli ci modules/ --threshold 80 --format junit
lint-arwaky-cli scan modules/ --format json --output-dir ~/.local/share/lint-arwaky/reports
lint-arwaky-cli scan modules/ --format sarif > ~/.local/share/lint-arwaky/reports/scan_python.sarif
```

## Python-specific fix routing

- AES101 rename → after `git mv`, update `__init__.py` imports (the module barrel is the
  registration point; a file not re-exported there reads as an orphan, AES501–506).
- AES201 forbidden import → for a `capabilities_*` file the allowed set is `taxonomy`,
  `contract`, `utility` (mandatory: `taxonomy` + `contract(*_protocol)`); forbidden: other
  `capabilities_*`, `agent`, `surface`, `root`. Break the dependency through a contract
  protocol/aggregate — see `create-contract`.
- AES204 dummy import / AES304 bypass → `fix-bypass` (see the grep pattern below).
- AES403: every capability class MUST inherit from its protocol ABC.
- AES405 (Agent Role): every agent class MUST inherit from its aggregate ABC, must not import
  `capabilities_*` directly, and must avoid `: Any` annotations.
- AES404 (Utility Role): `utility_*` files hold stateless standalone functions only and may
  import `taxonomy` and nothing else.
- AES303 mandatory definition → add the missing `class` / `def`.

## Role boundaries

| Layer        | Can Contain                  | Cannot Contain             |
| :----------- | :--------------------------- | :------------------------- |
| capabilities | Pure computation, validation | I/O, network, database     |
| utility      | Stateless pure functions     | State, classes with fields, non-taxonomy imports |
| agent        | Orchestration flow           | Computation, I/O, business |

## Pre-flight (Python)

```bash
# Verify Python environment works
python3 -c "import sys; print(sys.version)"

# Check that key dependencies are importable
python3 -c "import ruff; print('ruff ok')" 2>/dev/null || echo "ruff not installed"
```

## Verify pipeline (Python)

```bash
# 1. Re-scan — should show 0 violations
lint-arwaky-cli scan <target-path>

# 2. Syntax check
python3 -m py_compile src/<module>.py

# 3. Import check
python3 -c "import <module>"

# 4. Run ruff (if available)
ruff check src/

# 5. Run mypy (if available)
mypy src/
```

## Quick fix recipes (Python)

### Rename file (AES101/102)

```bash
# Before: capabilities_scanner.py (wrong — missing concern + suffix mismatch)
# After:  capabilities_file_scanner.py
git mv src/capabilities_scanner.py src/capabilities_file_scanner.py
# Update __init__.py imports
```

### Remove unused import (AES203)

```bash
lint-arwaky-cli fix src/file.py --filter AES203
# Or manually: remove the unused import line
```

### Fix bypass comments (AES304)

```bash
# Find all bypass patterns
grep -rn 'noqa\|type: ignore\|pragma: no cover\|FIXME\|TODO' src/

# Fix each one:
# noqa → fix the underlying lint issue
# type: ignore → add proper type annotation
# pragma: no cover → ensure code is testable
```

### Remove dead code (AES501–506)

```bash
# Find orphan files
lint-arwaky-cli orphan modules/ --format json

# If orphan is truly dead → delete it
# If orphan should be wired → add to container or import chain
```

### Fix layer role violation (AES401–406)

```bash
# Identify which role rule is violated
lint-arwaky-cli role modules/ --filter AES403

# Common fixes:
# - Move I/O code from capabilities to utility
# - Move business logic from surface to capabilities
# - Move orchestration from capabilities to agent
```

## Common Issues & Fix Strategies

| Issue                          | Fix Strategy                        |
| :----------------------------- | :---------------------------------- |
| Cross-layer imports            | Use contract layer protocols via DI |
| Missing protocol inheritance   | Create protocol ABC and inherit     |
| Mixed layer responsibilities   | Move code to appropriate layer      |
| Magic constants                | Extract to taxonomy constants       |
| Surface importing capabilities | Use aggregate contracts instead     |

## Verification Checklist (Python)

- [ ] All layer imports follow AES201 rules (`python3 -c "import <module>"` still succeeds).
- [ ] All capability classes inherit their protocol ABC (AES403).
- [ ] Utility files are stateless and import `taxonomy` only (AES404).
- [ ] Agent classes inherit their aggregate ABC and use concrete types, not `Any` (AES405).
- [ ] Surface files follow role-based imports (AES406).
- [ ] `ruff check src/` and `mypy src/` clean.
- [ ] `lint-arwaky-cli scan modules/` reports 0 violations.
