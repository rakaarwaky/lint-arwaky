# Test Specification & Verification Matrix — Lint Arwaky

## 1. Test Projects

There are 3 test workspaces with 2 variants each:

- **`workspaces-bad/`** — files with intentional violations (linter SHOULD detect them)
- **`workspaces-good/`** — clean files (linter should NOT flag them = false positive test)

> **Demo note (PE-4-02 / #627):** until #606 (every non-`scan` TUI action blocks the main
> render thread with no progress indicator) is resolved, scope any live/stakeholder TUI
> demo to these fixture workspaces — do not run non-`scan` TUI actions (`fix`, `ci`,
> `orphan`, `security`, `dependencies`, and the gated actions below) against a large
> real-world project in front of an audience, since the UI will appear to hang with no
> on-screen feedback. The CLI path is unaffected by #606 and is preferred for demos
> against larger codebases. Remove this note once #606 closes.

| Category         | Bad Path                     | Good Path                     | Purpose                                    |
| ---------------- | ---------------------------- | ----------------------------- | -------------------------------------------|
| Rust (crates)    | `workspaces-bad/crates/`   | `workspaces-good/crates/`   | AES Rust rules + Clippy/Rustfmt/cargo-audit|
| Python (modules) | `workspaces-bad/modules/`  | `workspaces-good/modules/`  | AES Python rules + Ruff/MyPy/Bandit        |
| JS/TS (packages) | `workspaces-bad/packages/` | `workspaces-good/packages/` | AES JS/TS rules + ESLint/Prettier/tsc      |

### Workspace Structure

```text
workspaces-bad/                   
├── crates/                       
│   ├── shared_common/src/        
│   ├── naming_violations/src/     
│   ├── quality-rules/src/       
│   └── ...
├── modules/                      
├── packages/                      
├── Cargo.toml, pyproject.toml, package.json, ...

workspaces-good/                   
├── crates/                        
│   └── ...
├── modules/                       
├── packages/                     
├── Cargo.toml, pyproject.toml, package.json, ...
```

### Expected Violation Counts

| Workspace | Language | Files | Violations | False Positives | Expected AES Codes|
| --------- | -------- | ----- | ---------- | --------------- | ------------------|
| bad       | Rust     | 156   | ≥ 100 (non-regressing floor) | — | 41 unique codes|
| bad       | Python   | 160   | ≥ 100 (non-regressing floor) | — | 41 unique codes|
| bad       | JS/TS    | 114   | ≥ 100 (non-regressing floor) | — | 41 unique codes|
| good      | Rust     | 35    | 0          | 0               | —                 |
| good      | Python   | 42    | 0          | 0               | —                 |
| good      | JS/TS    | 33    | 0          | 0               | —                 |

> **Note**: workspaces-good must produce exactly 0 violations. If any violation
> appears, it is a false positive that must be fixed. Bad workspaces must not
> regress below the documented ≥ 100 violation floor per language.
>
> **Reconciled rule count (single source of truth — §3.2):** the product
> enforces **41 AES rules** total (per the ARCHITECTURE.md Rule Index table
> and the §3.2 Per-Rule Detection Matrix). `scan` surfaces **all 41** in a
> whole-workspace scan. Each member folder (`crates`, `modules`, `packages`)
> carries its own root-doc set so that a per-member scan also surfaces all
> 41 codes — the linter's doc-audit walk roots at the scan target, so a
> per-member scan collects root docs from the member directory itself. The
> CI "AES Codes Check" gate enforces all 41 unique codes per scan — no
> safety margin (QA #636).
>
> **External tools prerequisite**: Python & JS/TS scans require external tools
> installed (ruff, mypy, bandit, eslint, prettier, tsc) for
> external lint violations. **CI satisfies this**: the `self-lint` job in
> `.github/workflows/ci.yml` installs all seven tools and asserts tool-native
> (non-AES) violation codes appear in the bad-workspace scan output before the
> job can pass (QA #637).
>
> **Gitleaks secret-scanning gate**: the `self-lint` job also installs
> Gitleaks (pinned to `v8.18.4`) and runs the Gitleaks adapter end-to-end on
> the fixture dirs `workspaces-bad/fixtures/gitleaks/` and
> `workspaces-good/fixtures/gitleaks/`. The bad-workspace fixture files
> carry Gitleaks' own public test vectors and the bad workspace intentionally
> ships no `.gitleaks.toml` (so Gitleaks' built-in default rule set applies);
> the good-workspace file is
> allowlisted in `workspaces-good/.gitleaks.toml` so the scan reports zero
> findings there. The CI step asserts at least one `RuleID` appears in the
> bad-workspace JSON report and none in the good-workspace report.

See [README.md](README.md) for CLI reference and
[ARCHITECTURE.md](ARCHITECTURE.md) for AES background.

---

## 2. How to Run Tests

### 2.0 Regression Test Convention

Every CRITICAL or WARNING defect fix MUST add or extend a
`regression_<short-name>.rs` file in the `tests/` directory of the crate where
the defect was found, asserting the previously-broken behavior is now
correct. The canonical example is
[`crates/dispatcher/tests/regression_scan_modes.rs`](crates/dispatcher/tests/regression_scan_modes.rs).
Where a fix spans behaviors, name the test after the tracking issue
(e.g. `regression_640_existing_custom_hook_is_backed_up_on_install`), so the
guard is greppably tied to the defect it protects. A defect fix is not
complete until its regression guard lands (QA #639).

### 2.1 Self-Lint (must be clean)

```bash
cd <repo-root>
cargo run --bin lint-arwaky-cli -- scan .
```

> `scan .` runs ALL 7 code linters (naming, import, quality, role, orphan, structure, external) on the lint-arwaky codebase itself. Expected: **0 violations**. Document invariants (AES601–AES610) are audited separately with `docs .`.

### 2.2 Scan Test Projects

```bash
# Scan bad workspace (should find violations)
cargo run --bin lint-arwaky-cli -- scan workspaces-bad/crates
cargo run --bin lint-arwaky-cli -- scan workspaces-bad/modules
cargo run --bin lint-arwaky-cli -- scan workspaces-bad/packages

# Scan good workspace (should find 0 violations = false positive test)
cargo run --bin lint-arwaky-cli -- scan workspaces-good/crates
cargo run --bin lint-arwaky-cli -- scan workspaces-good/modules
cargo run --bin lint-arwaky-cli -- scan workspaces-good/packages
```

> Language is auto-detected from file extensions. No language flag needed.
> Python and JS/TS scans require external tools installed (ruff,
> mypy, bandit, eslint, prettier, tsc) for external lint
> violations — see the prerequisite note in §1, which the CI `self-lint` job
> satisfies (QA #637).
>
> Rule policy lives in `ignored_rules:` in `lint_arwaky.config.yaml`, beside the
> `adapters:` list.

### 2.3 Individual Rule Surface

```bash
# Run only naming rules on Rust test workspace
cargo run --bin lint-arwaky-cli -- naming workspaces-bad/crates

# Run only import rules
cargo run --bin lint-arwaky-cli -- import workspaces-bad/crates

# Run only orphan detection
cargo run --bin lint-arwaky-cli -- orphan workspaces-bad/crates
```

### 2.3.1 Layer-Scoped Surface

Each AES layer has its own subcommand: it runs **every** rule group over the
target and reports only the violations whose file belongs to that layer.

```bash
# Every rule group, taxonomy-layer findings only
cargo run --bin lint-arwaky-cli -- taxonomy workspaces-bad/crates

# Same for the other five layers (json so the codes can be tallied)
cargo run --bin lint-arwaky-cli -- contract workspaces-bad/crates --format json
cargo run --bin lint-arwaky-cli -- capabilities workspaces-bad/crates --format json
cargo run --bin lint-arwaky-cli -- utility workspaces-bad/crates --format json
cargo run --bin lint-arwaky-cli -- agents workspaces-bad/crates --format json
cargo run --bin lint-arwaky-cli -- surface workspaces-bad/crates --format json
```

Pass/fail criteria:

| Check | Expected|
| --- | ---|
| Every file a layer command reports carries that layer's prefix (`taxonomy_`, `contract_`, `capabilities_`, `utility_`, `agent_`, `surface_`) | no foreign layer in any output|
| `taxonomy` on `workspaces-bad/crates` | > 0 violations, all in `taxonomy_*` files|
| Each of the six layer commands on `workspaces-good/crates` | **0** violations|
| `root_*` files | reported by `scan` / `check` only — `root` has no subcommand|
| A folder-level structure finding (AES702) or a doc invariant (AES601–606) | absent from all six layer commands; reported by `structure` / `docs`|

```bash
# Assert no layer command reports a foreign layer on the good workspace.
for layer in taxonomy contract capabilities utility agents surface; do
  echo "== $layer"
  cargo run --bin lint-arwaky-cli -- "$layer" workspaces-good/crates
done
```

### 2.4 Unit, Integration, and Doc Tests

```bash
# Unit + integration tests (~2000 tests workspace-wide)
cargo nextest run --workspace --lib --tests

# Doctests (~25 runnable examples across 11 source files)
cargo test --doc --workspace
```

> **Doctests are in documented test scope** (QA #638): public-API doc examples
> are real tests and must stay green. `cargo-nextest` **cannot execute
> doctests at all** (known upstream limitation) — `nextest run` and
> `cargo test --doc` are complementary, not substitutable, and run as two
> separate steps in CI. Do **not** assume `--doc` can be appended to the
> nextest invocation (QA #644).

### 2.5 Code Coverage

```bash
# Statement/branch coverage summary (requires llvm-tools-preview component)
cargo llvm-cov nextest --workspace --lib --tests --summary-only
```

> The CI `coverage` job prints this summary on every run so the workspace
> coverage percentage is always visible without manual computation (QA #643).

---

## 3. Pass / Fail Criteria

### 3.1 Thresholds

Thresholds match the §1 Expected Violation Counts table (≥ 100 violations floor per language; 0 in good workspaces).

| Criteria                       | PASS   | FAIL      |
| ------------------------------ | ------ | ----------|
| Total violations (Rust scan)   | >= 100 | < 100 or 0|
| Total violations (Python scan) | >= 100 | < 100 or 0|
| Total violations (JS/TS scan)  | >= 100 | < 100 or 0|
| Unique AES codes (crates)      | >= 41  | < 41      |
| Unique AES codes (modules)     | >= 41  | < 41      |
| Unique AES codes (packages)    | >= 41  | < 41      |
| Self-lint violations           | 0      | > 0       |
| Doc-consistency gate failures  | 0      | > 0       |

> The "Unique AES codes" thresholds are the reconciled scan-visible counts
> from §3.2 (41 AES rules total). Each member folder carries its own
> root-doc set so that a per-member scan (`scan workspaces-bad/crates`,
> `scan workspaces-bad/modules`, `scan workspaces-bad/packages`) surfaces
> all 41 codes. The CI "AES Codes Check" step enforces the 41-code floor
> with no lower safety margin, so a detection regression in any one of the
> 41 rules fails CI (QA #636). When rules are added/removed, update §3.2,
> this table, §1, and the `ci.yml` threshold in lockstep.

### 3.2 Per-Rule Detection Matrix

Every AES rule MUST produce at least 1 violation in the test workspaces.
If any rule produces 0 violations, the test project is missing a trigger file.

| Rule   | Description                            | Rust | Python | JS/TS| Whole Tree¹|
| ------ | -------------------------------------- | ---- | ------ | ---- | -----------|
| AES101 | Naming convention                      | ✓   | ✓     | ✓   | ✓          |
| AES102 | Suffix/prefix validation               | ✓   | ✓     | ✓   | ✓          |
| AES103 | Test/bench file prefix                 | ✓   | ✓     | ✓   | ✓          |
| AES201 | Layer dependency violation             | ✓   | ✓     | ✓   | ✓          |
| AES202 | Mandatory import missing               | ✓   | ✓     | ✓   | ✓          |
| AES203 | Unused import                          | ✓   | ✓     | ✓   | ✓          |
| AES204 | Dummy import / function                | ✓   | ✓     | ✓   | ✓          |
| AES205 | Circular dependency                    | ✓   | ✓     | ✓   | ✓          |
| AES301 | Max line count exceeded                | ✓   | ✓     | ✓   | ✓          |
| AES302 | Min line count below                   | ✓   | ✓     | ✓   | ✓          |
| AES303 | Missing definitions / dead inheritance | ✓   | ✓     | ✓   | ✓²         |
| AES304 | Bypass detection                       | ✓   | ✓     | ✓   | ✓          |
| AES305 | Duplicate code                         | ✓   | ✓     | ✓   | ✓          |
| AES401 | Taxonomy purity                        | ✓   | ✓     | ✓   | ✓          |
| AES402 | Contract primitives                    | ✓   | ✓     | ✓   | ✓          |
| AES403 | Capability implementation              | ✓   | ✓     | ✓   | ✓          |
| AES404 | Utility purity                         | ✓   | ✓     | ✓   | ✓          |
| AES405 | Agent composition, delegation, blocks   | ✓   | ✓     | ✓   | ✓          |
| AES406 | Surface passive role                   | ✓   | ✓     | ✓   | ✓          |
| AES501 | Taxonomy orphan                        | ✓   | ✓     | ✓   | ✓          |
| AES502 | Contract orphan                        | ✓   | ✓     | ✓   | ✓          |
| AES503 | Capabilities orphan                    | ✓   | ✓     | ✓   | ✓          |
| AES504 | Utility orphan                         | ✓   | ✓     | ✓   | ✓          |
| AES505 | Agent orphan                           | ✓   | ✓     | ✓   | ✓          |
| AES506 | Surface orphan                         | ✓   | ✓     | ✓   | ✓          |
| AES601 | AGENTS doc invariants                  |  |   |  | ✓          |
| AES602 | README doc invariants                  |  |   |  | ✓          |
| AES603 | ARCHITECTURE doc invariants            |  |   |  | ✓          |
| AES604 | CONTRIBUTING doc invariants            |  |   |  | ✓          |
| AES605 | PRD doc invariants                     |  |   |  | ✓          |
| AES606 | ROADMAP doc invariants                 |  |   |  | ✓          |
| AES607 | BACKLOG doc invariants                 |  |   |  | ✓          |
| AES608 | DATA doc invariants                    |  |   |  | ✓          |
| AES609 | FRD doc invariants                     |  |   |  | ✓          |
| AES610 | DESIGN doc invariants                  |  |   |  | ✓          |
| AES701 | Shared folder purity                   | ✓   | ✓     | ✓   | ✓          |
| AES702 | Feature folder health                  | ✓   | ✓     | ✓   | ✓          |
| AES703 | Surface folder purity                  | ✓   | ✓     | ✓   | ✓          |
| AES704 | Test-suite category coverage           | ✓   | ✓     | ✓   | ✓          |
| AES705 | Member-root placement                  | ✓   | ✓     | ✓   | ✓          |
| AES706 | PR ID folder mapping                   | ✓   | ✓     | ✓   | ✓          |

¹ The AES6xx doc-invariant codes fire on the root spec documents (AGENTS.md,
README.md, ARCHITECTURE.md, CONTRIBUTING.md, PRD.md, ROADMAP.md, DATA.md,
BACKLOG.md, FRD.md, DESIGN.md) via the doc-audit pass folded into `scan`.
Each member folder carries its own copy of the root docs plus a
`surface_probe` folder with a DESIGN.md, so a per-member scan
(`scan workspaces-bad/crates`, `scan workspaces-bad/modules`,
`scan workspaces-bad/packages`) surfaces all 41 codes individually.

² AES303 mandatory-definition is skipped for the `utility` and `surface`
layers (stateless pure functions and function dispatchers), while remaining
active for taxonomy, contract, capabilities, agent, and root.

### 3.3 Negative Tests (must produce 0 violations)

| #  | Scenario                                                | Expected                                       |
| -- | ------------------------------------------------------- | -----------------------------------------------|
| 1  | Barrel file (`mod.rs`, `__init__.py`, `index.ts`) | 0 violations (skipped)                         |
| 2  | File in`exceptions` list                              | 0 violations (skipped)                         |
| 3  | Config`architecture.enabled: false`                   | 0 violations (all rules disabled)              |
| 4  | Rule`AES201.enabled: false`                           | 0 AES201 violations (other rules still run)    |
| 5  | Clean, well-structured file                             | 0 violations                                   |
| 6  | `pub use` re-export in barrel file                    | 0 violations (not flagged as unused/dummy)     |
| 7  | `unwrap_or_default()` usage                           | 0 AES304 violations (safe variant)             |
| 8  | Import inside`#[cfg(test)]` block                     | 0 violations (conditional skip)                |
| 9  | Root layer file (`root_*`)                            | 0 role-rule violations (skipped)               |
| 10 | File with`parse_ok = false`                           | PARSE_WARN emitted, file skipped for AES checks|

### 3.4 Exit Code Tests

The contract itself lives in `PRD.md` → `Project Requirements` → `Workspace Exit Code Contract`.
This table is the executable proof of that contract; it must not introduce outcomes the contract
does not define.

| #  | Scenario | Expected Exit Code |
|----|----------|--------------------|
| 1  | `scan` on clean project | 0 |
| 2  | `scan` on workspaces-bad (violations found) | 1 |
| 3  | `scan` on nonexistent path | 2 |
| 4  | `scan` with invalid arguments | 2 |
| 5  | `security` without cargo-audit installed | 3 + visible scanner-missing warning |
| 6  | `security` on pure Python without `pip-audit` | 3 + visible `pip-audit` warning; never silent `findings: []` |
| 7  | `security` on JS/TS lockfile without npm | 3 + visible `npm-audit` warning; never a clean result |
| 8  | `ci --threshold 0` with violations | 1 |
| 9  | `ci --threshold 100` with few violations | 0 |
| 10 | `ci --threshold N` on a workspace with exactly N findings | 0 (boundary; N+1 findings → 1) |
| 11 | `fix --dry-run` with violations | 0 (preview only) |
| 12 | `fix` with every finding applied | 0 |
| 13 | `fix` leaving at least one finding un-applied | 1 |
| 14 | `docs` on a workspace with a document violation | 1 |
| 15 | `docs` on a conformant workspace | 0 |
| 16 | `doctor` with all tools installed | 0 |
| 17 | `doctor` with missing tools | 0 (missing tools listed in body) |
| 18 | `report` with no previous snapshot | 2 |
| 19 | MCP `execute_command` on a path outside the workspace | `exit_code: 2` in the JSON result |

### 3.5 Test Suite Quality Thresholds

§3.1 gates the AES self-lint surface; this section gates the test suite itself.
Meaningful targets for the quality metrics reporting required by the QA cycle
(QA #650), cross-referenced from the "Current Quality Status" section in
[DEPLOY.md](DEPLOY.md):

| Criteria                                        | PASS | FAIL           |
| ----------------------------------------------- | ---- | ---------------|
| `cargo nextest run --workspace` pass rate       | 100% | any failing test|
| `cargo test --doc --workspace` pass rate        | 100% | any failing test|
| `#[ignore]`d tests without a linked issue + expiry date | 0 | >= 1        |
| Open CRITICAL defects before release             | 0    | >= 1           |

> **Baseline to preserve**: the workspace currently has **zero `#[ignore]`d
> tests** — a test may only be quarantined with a linked tracking issue and an
> expiry date; anything else must be fixed or deleted, not ignored.
>
> **Tracking caveat (QA #645)**: any severity or defect-density metric derived
> from GitHub Issue *labels* is unreliable until #618 (label write permission)
> is resolved. Use issue *title* text — `[ROLE][SEVERITY] …` — as the source
> of truth for severity counts until then, and caveat any quality dashboard
> accordingly.

### 3.6 Assertion-Depth Guideline for High-Risk Crates

Test-file *structure* (the acceptance/unit/contract/integration/smoke/e2e
taxonomy) is not sufficient on crates with outsized blast radius — tests must
also have *depth* (QA #641):

- Any crate whose public surface is reachable by an untrusted external caller
  (e.g. `crates/mcp-server`, called by out-of-repo MCP clients) or that fronts
  destructive user actions (e.g. `crates/tui`) SHOULD average **≥ 3 tests per
  taxonomy category file**, each asserting behavior (inputs → outputs, exact
  error envelopes, exit codes) rather than type existence.
- Vacuous checks — e.g. asserting only `std::any::type_name::<T>()`, module
  importability, or struct size — do not count toward a category's test depth
  and SHOULD be replaced with behavioral assertions over time.
- The path/boundary reaching untrusted input (e.g. the MCP `execute_command`
  path argument) MUST have negative-path tests: invalid, empty, and hostile
  inputs each with exact expected envelopes.

This guideline was introduced while remediating QA #641 (`crates/mcp-server`)
and is cross-linked with FE #612 (`crates/tui`) so both surfaces are
remediated under one shared "thin high-risk test suite" theme.

---

## 4. User Acceptance Testing (UAT)

Persona-level end-to-end acceptance verifies business outcomes, not synthetic rule fixtures. Every
scenario below is executed against a release candidate, and its evidence is a replayable transcript
stored under `docs/uat/<version>/<UAT-ID>.log` (or a CI run URL) plus a commit that is reachable from
the release tag.

### 4.1 Acceptance Matrix

Scope: Linux x86_64 and macOS. The Windows MSVC target is build-only and explicitly out of scope
for UAT until the runtime support decision lands.

Status as of 3.16.1: **not run**. The previous "Verified" record cited `8d4342a`, a commit that does
not exist in this repository, so it has been withdrawn rather than carried forward.

| ID | Persona | Scenario | Test data / workflow | Expected business outcome | Status | Evidence |
|----|---------|----------|----------------------|---------------------------|--------|----------|
| UAT-001 | Developer | Onboarding: `init` → `doctor` → `check .` | Clean clone, no config file | Config written, toolchain reported, `check .` exits 0 | Not run | `docs/uat/3.16.1/UAT-001.log` |
| UAT-002 | Developer | Negative: invalid config | Config with an unknown threshold type | Exit 2, the offending key is named, no stack trace | Not run | `docs/uat/3.16.1/UAT-002.log` |
| UAT-003 | Developer | Boundary: toggle one rule off, rescan | `workspaces-bad/crates` and one rule disabled | That rule's findings disappear; every other rule still fires | Not run | `docs/uat/3.16.1/UAT-003.log` |
| UAT-004 | DevOps / CI | Happy: clean workspace gated | `workspaces-good/crates`, `ci --threshold 0` | Exit 0, pipeline passes | Not run | `docs/uat/3.16.1/UAT-004.log` |
| UAT-005 | DevOps / CI | Negative: violating workspace gated | `workspaces-bad/crates`, `ci --threshold 0` | Exit 1, every offending file and rule listed | Not run | `docs/uat/3.16.1/UAT-005.log` |
| UAT-006 | DevOps / CI | Boundary: threshold equals finding count | Workspace with exactly N findings | Exit 0 at `--threshold N`, exit 1 at `--threshold N-1` | Not run | `docs/uat/3.16.1/UAT-006.log` |
| UAT-007 | AI agent | Autonomous fix through MCP | `tools/list`, then `execute_command fix` on a known AES203 finding | Five tools listed; fix applied once; follow-up `scan` exits 0 | Not run | `docs/uat/3.16.1/UAT-007.log` |
| UAT-008 | AI agent | Negative: path outside the workspace | `execute_command` with `../` target | `exit_code: 2`, nothing written, workspace untouched | Not run | `docs/uat/3.16.1/UAT-008.log` |
| UAT-009 | AI agent | Health check with a missing adapter | `health_check` on a host without `ruff` | Adapter reported `not_installed`; result never claims a clean state | Not run | `docs/uat/3.16.1/UAT-009.log` |
| UAT-010 | Release manager | Document gate on a mutated document copy | Workspace copy with one required H2 deleted | `docs .` exits 1 and names the document, code, and line | Not run | `docs/uat/3.16.1/UAT-010.log` |
| UAT-011 | Maintainer | Report and delta | `scan .` then `report .` | By-code and by-member tables render; delta reflects the previous run | Not run | `docs/uat/3.16.1/UAT-011.log` |
| UAT-012 | Maintainer | Preview then apply | `fix . --dry-run`, then `fix .` | Dry run changes zero bytes; apply rewrites each fixable finding once | Not run | `docs/uat/3.16.1/UAT-012.log` |

### 4.2 Evidence Rules

- Every row records tester, date, OS, exact commands, and captured output. A row whose evidence cannot be replayed is `Not run`, not `Verified`.
- The evidence commit must be reachable from the release tag, and the signing date must be on or after the release-candidate date.
- A scenario is `Verified` only when a reviewer other than the one who ran it can reproduce the stated outcome from the transcript.

### 4.3 Exit Criteria

UAT passes, and a release may proceed, only when all of the following hold:

1. Every scenario in 4.1 is `Verified` with replayable evidence.
2. Open CRITICAL defects are 0, counted from the `severity-critical` label (label writes must stay functional; if they regress, a milestone-based severity queue replaces this counter).
3. Every deferred defect appears in the waiver log with an approver, a justification, and an expiry date.
4. Each persona row is signed by name, role, and date, and QA is signed by someone other than the release author.

### 4.4 Waiver Log

| Defect | Scenario | Justification | Approved by | Expires |
|--------|----------|---------------|-------------|---------|
| _none recorded_ | — | — | — | — |

## 5. Release Eligibility Checklist

The product-engineering release gate lives in this checklist (5.1-5.8); the dependency
map and risk tracking live in `ROADMAP.md`'s Risk Register section; per-feature
dependency detail lives in each feature's BACKLOG.md;
role sign-offs and the rollback record live in `DEPLOY.md`'s Release Sign-off Checklist
and Rollback Record. This section cross-references all three rather than restating them.

Before releasing the binary to production or deploying to a client,
complete all verification tasks below.

### 5.1 Architecture Compliance (Self-Lint)

The base codebase must be clean of internal architecture rule violations.

- [ ] Run self-lint audit:

  ```bash
  cargo run --bin lint-arwaky-cli -- check .
  ```

- [ ] **Criteria**: Output must show **`Total violations: 0`**.
- [ ] **Safety net**: No inline bypasses (`#[allow(...)]`, `unwrap()`, `todo!()`,
  `FIXME`, `HACK`). If an external module strictly requires an exception,
  register it in `lint_arwaky.config.yaml` under the `exceptions`
  block — never use inline bypass comments.

### 5.2 Cross-Language Functional Verification

- [ ] Build a clean release:

  ```bash
  bash scripts/install.local.sh
  ```

- [ ] Run scan on bad workspaces (should find violations):

  ```bash
  lint-arwaky-cli scan workspaces-bad/crates
  lint-arwaky-cli scan workspaces-bad/modules
  lint-arwaky-cli scan workspaces-bad/packages
  ```

- [ ] Run scan on good workspaces (should find 0 violations):

  ```bash
  lint-arwaky-cli scan workspaces-good/crates
  lint-arwaky-cli scan workspaces-good/modules
  lint-arwaky-cli scan workspaces-good/packages
  ```

- [ ] **Criteria**: Bad workspaces meet aggregate thresholds (Section 3.1).
- [ ] **Criteria**: Good workspaces produce 0 violations (false positive test).
- [ ] **Criteria**: All 41 unique AES codes detected per language (Section 3.2).
- [ ] **Criteria**: All negative tests pass (Section 3.3).
- [ ] **Criteria**: All exit code tests pass (Section 3.4).

### 5.3 System & MCP Protocol Verification

- [ ] Run workspace unit tests:

  ```bash
  cargo test --workspace
  ```

- [ ] Run binary health diagnostics:

  ```bash
  lint-arwaky-cli doctor
  ```

- [ ] Run MCP protocol smoke test:

  ```bash
  echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | lint-arwaky-mcp
  ```

  **Criteria**: Responds in < 2 seconds with the complete list of 5 registered
  MCP tools (`execute_command`, `list_commands`, `health_check`,
  `get_config`, `docs_scaffold`).

### 5.4 Report Format Verification

- [ ] JSON output:

  ```bash
  lint-arwaky-cli scan workspaces-bad/crates --format json
  ```

- [ ] SARIF output:

  ```bash
  lint-arwaky-cli scan workspaces-bad/crates --format sarif
  ```

- [ ] JUnit XML output:

  ```bash
  lint-arwaky-cli scan workspaces-bad/crates --format junit
  ```

  **Criteria**: All 3 formats produce valid, parseable output with correct
  rule codes, file paths, line numbers, and severity levels.

### 5.5 Demo Walkthrough

A safe, presenter-ready sequence using the existing fixtures (PE-4-03 / #628), consolidating
the PE-4-01 and PE-4-02 caveats into one positive script rather than leaving them scattered:

- [ ] `lint-arwaky-cli scan workspaces-bad/crates` — show violations found.
- [ ] Edit `lint_arwaky.config.yaml`, set a rule's `enabled` flag to `false` (e.g. an
      AES2xx import rule from the negative-test table in Section 3.3).
- [ ] Re-run the scan from the first step — show that rule's violations no longer
      reported (the toggle mechanism works end-to-end).
- [ ] `lint-arwaky-cli fix workspaces-bad/crates --dry-run` — show proposed fixes without
      writing anything.
- [ ] `lint-arwaky-cli ci workspaces-bad/crates --threshold 0` — show the non-zero exit
      code a CI pipeline would gate on.

Caveats a presenter must know before going live:

- The TUI confirm-gate defect (#552) and the synchronous main-thread defect (#606) are closed, so
  gated actions may be demoed — but only after a dry run of the exact sequence on the demo host,
  because no UAT scenario currently covers the TUI (see UAT-012 and Section 4.1).
- Scope any TUI portion of the demo to `workspaces-good`/`workspaces-bad`; do not point a live demo
  at a customer tree.
- `README.md`'s Quick Start does not link this script; the canonical entry point is this section.

### 5.5.1 Demo Evidence

Record the presenter, host, date, and terminal size with the demo, and link the transcript from
`docs/uat/<version>/` so a future release can replay it.

### 5.6 Artifact Manifest

Per-release artifact inventory, populated at tag time and retained for rollback
(cross-referenced from `DEPLOY.md`'s Deploy checklist and Rollback Record):

| Release | Artifact | Platform | Checksum File | Provenance Attestation | Retained Until|
|---|---|---|---|---|---|
| v3.8.0 | `lint-arwaky-cli`, `lint-arwaky-mcp` | linux x86_64 | Not published (see `DEPLOY.md` installation notes / #631) | Generated by `release.yml` (`provenance` job) | — (not yet recorded)|
| v3.7.1 / v3.7.2 | Never tagged — superseded by v3.8.0 (PE-3-01 / #623, closed) | — | — | — | —|

### 5.7 Role Sign-offs

Role sign-offs are tracked in `DEPLOY.md`'s Release Sign-off Checklist (Product,
Engineering, QA, Documentation, Operations domains), not duplicated here. See PE-5-01
(#629) for the record and its caveats.

### 5.8 Dependency Map

The crate-to-crate and crate-to-external-client dependency map is tracked in
each feature's `BACKLOG.md` Dependencies section, not duplicated here. See PE-2-01 (#620) and
PE-2-03 (#622).

---

## 6. Instructions for AI Agents

1. **Automated verification**: Every time you modify code, rebuild with
   `scripts/install.local.sh` and run `check .` locally.
2. **Fix the root cause, do not bypass**: Never use inline bypasses
   (`unwrap`, `expect`, `panic!`, `noqa`, `#[allow(...)]`, `FIXME`, `HACK`)
   to suppress architecture warnings.
3. **Readiness report**: Upon completing work, report the status of every
   item in the Section 5 checklist transparently to the user.
4. **Test project maintenance**: If a new AES rule is added, add
   corresponding trigger files to all 3 test workspaces and update the
   per-rule detection matrix (Section 3.2).
5. **No false positives**: If a clean file produces a violation, it is a
   bug in the rule implementation — fix the rule, do not modify the test
   project to accommodate the bug.
6. **Full test execution**: run both `cargo nextest run --workspace --lib
   --tests` and `cargo test --doc --workspace` — nextest alone silently skips
   all doctests (Sections 2.4, QA #644).
7. **Coverage**: to compute the workspace coverage percentage locally, run
   `cargo llvm-cov nextest --workspace --lib --tests --summary-only`
   (Section 2.5); CI's `coverage` job publishes the same summary every run
   (QA #643).

---

## Reference

- PRD: [PRD.md](PRD.md)
- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md)
- CLI Reference: [README.md](README.md)
- MCP Deployment: [DEPLOY.md](DEPLOY.md)
