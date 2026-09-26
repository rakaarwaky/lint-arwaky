# HOW TO USE LINT PYTHON

> **Purpose**: Scan, diagnose, and fix AES architecture violations in a Python codebase with
> `lint-arwaky-cli`, then re-verify to zero findings.
>
> **Audience**: Agents and engineers running or remediating aes-lint-arwaky on Python modules.
>
> **Scope**: Python workspace paths (`modules/`, `workspaces-bad/modules/`), Python adapters
> (Ruff, Mypy, Radon, Bandit), Python-specific fix routing and verify pipeline.
>
> **Location**: Run from the repo root (or the lint target root); reports default under
> `${XDG_DATA_HOME:-~/.local/share}/aes-lint-arwaky/reports/`.
>
> **Length**: One scan session = pre-flight → scan → triage → fix → re-scan to 0. Shared
> commands, MCP tools, exit codes, and AES101 naming live in
> `HOW-TO-USE-LINT-COMMANDS.md`; the full AES fix table lives in
> `HOW-TO-USE-LINT-ROUTING.md`.

---

## Rules

Seven rules. Each one prevents a specific failure mode.

1. **Pre-flight first.** Violations on broken code are unreliable — confirm `python3` and key
   deps import before you trust a scan.
2. **Shared command surface is not restated here.** Global flags, aliases (`lac`/`lat`/`lam`),
   MCP tools, exit codes, and AES101 naming live in `HOW-TO-USE-LINT-COMMANDS.md`;
   triage order and the fix table live in `HOW-TO-USE-LINT-ROUTING.md`. This file only holds Python deltas.
3. **Path and `--member` follow the Python layout.** Examples use `modules/`; `--member` targets
   a **module** name (not a crate or package).
4. **Triage CRITICAL before HIGH/MEDIUM.** 🔴 AES201, AES205, AES304 → 🟡 AES101–102, AES202,
   AES301–303, AES401–403, AES406, AES505–506 → 🟢 AES203–204, AES305, AES404–405, AES501–504.
5. **Auto-fix only what is auto-fixable.** AES101 rename, AES203 unused import, AES304 bypass
   → `fix` (try `--dry-run` first). AES201 wrong dependency → manual; route through
   `aes-contract`.
6. **After AES101 rename, update the barrel.** `git mv` then re-export in `__init__.py` —
   a file missing from the barrel reads as an orphan (AES501–506).
7. **Re-scan to 0 before declaring done.** Then run the Python verify pipeline below;
   language compile is part of Verify, not a substitute for the AES scan.

---

## Template

Copy, fill, delete nothing.

### Adapters & external tooling

`lint-arwaky-cli install` installs external adapters; `lint-arwaky-cli adapters` lists the
active ones (Ruff, Mypy, Radon, Bandit, …). `get_config` accepts `"language": "python"`.

| Adapter | Standalone command | What it covers |
| ------- | ------------------ | -------------- |
| Ruff | `ruff check src/` | lint + style gate |
| Mypy | `mypy src/` | static type checking |
| Radon | `lint-arwaky-cli quality modules/` | complexity / quality metrics |
| Bandit | `lint-arwaky-cli security modules/` | security issues |
| CVE check | `lint-arwaky-cli dependencies modules/` | library vulnerabilities |

`lint-arwaky-cli external modules/` runs only the external linters (ruff).

### Scan session

```bash
# Pre-flight
python3 -c "import sys; print(sys.version)"
python3 -c "import ruff; print('ruff ok')" 2>/dev/null || echo "ruff not installed"

# Scan (text / json / by rule / by member)
lint-arwaky-cli scan workspaces-bad/modules
lint-arwaky-cli scan workspaces-bad/modules --format json
lint-arwaky-cli scan workspaces-bad/modules --filter AES201
lint-arwaky-cli orphan modules/ --member animator
lint-arwaky-cli ci modules/ --threshold 80 --format junit

# Reports → XDG data dir
lint-arwaky-cli scan modules/ --format json \
  --output-dir ~/.local/share/aes-lint-arwaky/reports
lint-arwaky-cli scan modules/ --format sarif \
  > ~/.local/share/aes-lint-arwaky/reports/scan_python.sarif
```

### Python-specific fix routing

- **AES101 rename** → after `git mv`, update `__init__.py` imports (barrel is the registration
  point; missing re-export = orphan AES501–506).
- **AES201 forbidden import** → for `capabilities_*`, allowed: `taxonomy`, `contract`,
  `utility` (mandatory: `taxonomy` + `contract(*_protocol)`); forbidden: other `capabilities_*`,
  `agent`, `surface`, `root`. Break the dependency through a contract protocol/aggregate —
  `aes-contract`.
- **AES204 / AES304** → `fix-bypass`.
- **AES403** → every capability class MUST inherit its protocol ABC.
- **AES405** → every agent class MUST inherit its aggregate ABC, no direct `capabilities_*`
  import, no `: Any` on aggregate signatures.
- **AES404** → `utility_*` files: stateless functions only; import `taxonomy` and nothing else.
- **AES303** → add the missing `class` / `def`.

### Role boundaries

| Layer | Can contain | Cannot contain |
| ----- | ----------- | -------------- |
| capabilities | Pure computation, validation | I/O, network, database |
| utility | Stateless pure functions | State, classes with fields, non-taxonomy imports |
| agent | Orchestration flow | Computation, I/O, business |

### Quick fix recipes

```bash
# AES101/102 rename
git mv src/capabilities_scanner.py src/capabilities_file_scanner.py
# then update __init__.py imports

# AES203 unused import
lint-arwaky-cli fix src/file.py --filter AES203

# AES304 bypass
grep -rn 'noqa\|type: ignore\|pragma: no cover\|FIXME\|TODO' src/
# fix root cause, delete suppression

# AES501–506 orphan
lint-arwaky-cli orphan modules/ --format json
# truly dead → delete; should be wired → container / import chain

# AES401–406 role
lint-arwaky-cli role modules/ --filter AES403
# I/O out of capabilities → utility; business out of surface → capabilities;
# orchestration out of capabilities → agent
```

### Common issues

| Issue | Fix strategy |
| ----- | ------------ |
| Cross-layer imports | Contract protocols via DI (`aes-contract`) |
| Missing protocol inheritance | Create protocol ABC and inherit (`aes-contract` + `aes-capabilities`) |
| Mixed layer responsibilities | Move code to the correct layer (`aes-*`) |
| Magic constants | Extract to taxonomy constants (`aes-taxonomy`) |
| Surface importing capabilities | Use aggregate contracts instead (`aes-contract`) |

---

## Section Contract

Every Python lint session is required to carry the rows that apply. Each exists for one reason.

| Section | Why it belongs here |
| ------- | ------------------- |
| Pre-flight | A scan on a broken interpreter is noise, not signal. |
| Scan with path + format | Findings must be machine-triageable (json) or human-scannable (text). |
| Triage order | CRITICAL (AES201/205/304) blocks merge; fix before HIGH/MEDIUM cosmetics. |
| Fix routing by AES code | Wrong fix layer recreates the violation; route via `HOW-TO-USE-LINT-ROUTING.md`. |
| Barrel update after rename | AES101 without `__init__.py` re-export becomes AES501–506. |
| Role-boundary check | Capabilities/utility/agent rows above must hold after fixes. |
| Re-scan to 0 | Done means zero AES findings, not "mostly clean". |
| Language verify | `py_compile` / import / ruff / mypy still pass — AES clean ≠ code healthy. |

---

## Verify

```bash
# 1. Re-scan — must be 0 AES violations
lint-arwaky-cli scan <target-path>

# 2. Syntax
python3 -m py_compile src/<module>.py

# 3. Import
python3 -c "import <module>"

# 4. Ruff (if available)
ruff check src/

# 5. Mypy (if available)
mypy src/
```

Exit codes (shared, `HOW-TO-USE-LINT-COMMANDS.md`): `0` pass · `1` violations · `2` config/parse error.

Checklist:

- [ ] Layer imports follow AES201; `python3 -c "import <module>"` succeeds.
- [ ] Capability classes inherit their protocol ABC (AES403).
- [ ] Utility files are stateless and import `taxonomy` only (AES404).
- [ ] Agent classes inherit their aggregate ABC; no `Any` on aggregate signatures (AES405).
- [ ] Surface files follow role-based imports (AES406).
- [ ] `ruff check src/` and `mypy src/` clean.
- [ ] `lint-arwaky-cli scan modules/` reports 0 violations.
