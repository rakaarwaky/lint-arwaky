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

| Category         | Bad Path                     | Good Path                     | Purpose                                     |
| ---------------- | ---------------------------- | ----------------------------- | ------------------------------------------- |
| Rust (crates)    | `workspaces-bad/crates/`   | `workspaces-good/crates/`   | AES Rust rules + Clippy/Rustfmt/cargo-audit |
| Python (modules) | `workspaces-bad/modules/`  | `workspaces-good/modules/`  | AES Python rules + Ruff/MyPy/Bandit         |
| JS/TS (packages) | `workspaces-bad/packages/` | `workspaces-good/packages/` | AES JS/TS rules + ESLint/Prettier/tsc       |

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

| Workspace | Language | Files | Violations | False Positives | Expected AES Codes |
| --------- | -------- | ----- | ---------- | --------------- | ------------------ |
| bad       | Rust     | 156   | ≥ 100 (non-regressing floor) | — | 27 unique codes |
| bad       | Python   | 160   | ≥ 100 (non-regressing floor) | — | 27 unique codes |
| bad       | JS/TS    | 150   | ≥ 100 (non-regressing floor) | — | 27 unique codes |
| good      | Rust     | 35    | 0          | 0               | —                  |
| good      | Python   | 42    | 0          | 0               | —                  |
| good      | JS/TS    | 33    | 0          | 0               | —                  |

> **Note**: workspaces-good must produce exactly 0 violations. If any violation
> appears, it is a false positive that must be fixed. Bad workspaces must not regress
> below the documented ≥ 100 violation floor per language.
>
> **Reconciled rule count (single source of truth — §3.2):** the product
> enforces **32 AES rules** total (per README and the §3.2 Per-Rule Detection
> Matrix). `scan` can only ever surface **27** of them: AES601–AES605 are
> doc-only invariants audited by the separate `docs` command. So
> *27 unique scan codes + 5 doc-only codes = 32 rules*. This is the documented
> exception explaining why the aggregate `workspaces-bad` scan expectation
> (27) is below the full catalog (32); the CI "AES Codes Check" gate enforces
> exactly 27 — no lower safety margin (QA #636).
>
> **External tools prerequisite**: Python & JS/TS scans require external tools
> installed (ruff, mypy, bandit, eslint, prettier, tsc, markdownlint-cli2) for
> external lint violations. **CI satisfies this**: the `self-lint` job in
> `.github/workflows/ci.yml` installs all seven tools and asserts tool-native
> (non-AES) violation codes appear in the bad-workspace scan output before the
> job can pass (QA #637).

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

> `scan .` runs ALL 7 code linters (naming, import, quality, role, orphan, structure, external) on the lint-arwaky codebase itself. Expected: **0 violations**. Document invariants (AES601–AES605) are audited separately with `docs .`.

Cross-document consistency — the drifts that span two artefacts and so fall outside AES601–AES605 — is a separate gate, run in CI as **Doc Consistency** and locally with:

```bash
python3 tools/check_doc_consistency.py
```

It fails when a rule-code range in a `DESIGN.md`/`FRD.md` names a code `RULES_AES.md` no longer publishes, an in-repo Markdown link points at a missing file or heading anchor, `crates/shared/DATA.md`'s attribute tables disagree with the shared value objects, the auto-fix Reason Code Reference disagrees with the enumerated reasons, the performance NFR states different numbers in `PRD.md`, `README.md`, and `crates/filesystem/FRD.md`, the AES605 `DESIGN.md` H2 contract in `crates/shared/src/doc_rules/taxonomy_doc_rules_constant.rs` disagrees with the template fenced in `HOW-TO-MAKE-DESIGN.md`, or the AES602 FRD level-3 contract (`FRD_H3_TITLES`) in the same file disagrees with the template fenced in `HOW-TO-MAKE-FRD.md`. Expected: **0 failures**.

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
> Python, JS/TS, and Markdown scans require external tools installed (ruff,
> mypy, bandit, eslint, prettier, tsc, markdownlint-cli2) for external lint
> violations — see the prerequisite note in §1, which the CI `self-lint` job
> satisfies (QA #637).
>
> A single Markdown file is a scan target too, and the `markdownlint` adapter
> runs on it:
>
> ```bash
> # Run markdownlint over one file (adapter selected from the .md extension)
> cargo run --bin lint-arwaky-cli -- scan workspaces-good/crates/calculator/BACKLOG.md
> ```
>
> Rule policy lives in `ignored_rules:` in `lint_arwaky.config.yaml`, beside the
> `adapters:` list. A listed code is dropped from the report when it matches as
> a case-insensitive substring, so `markdownlint::MD013` silences exactly that
> one rule and `MD013` works too. No per-tool companion config file is needed:
> `markdownlint-cli2` reads its policy only from the directory it runs in and
> never walks up, so a tool-specific file would have to be duplicated into
> every scan root.

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

| Check | Expected |
| --- | --- |
| Every file a layer command reports carries that layer's prefix (`taxonomy_`, `contract_`, `capabilities_`, `utility_`, `agent_`, `surface_`) | no foreign layer in any output |
| `taxonomy` on `workspaces-bad/crates` | > 0 violations, all in `taxonomy_*` files |
| Each of the six layer commands on `workspaces-good/crates` | **0** violations |
| `root_*` files | reported by `scan` / `check` only — `root` has no subcommand |
| A folder-level structure finding (AES702) or a doc invariant (AES601–605) | absent from all six layer commands; reported by `structure` / `docs` |

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

| Criteria                       | PASS   | FAIL       |
| ------------------------------ | ------ | ---------- |
| Total violations (Rust scan)   | >= 100 | < 100 or 0 |
| Total violations (Python scan) | >= 100 | < 100 or 0 |
| Total violations (JS/TS scan)  | >= 100 | < 100 or 0 |
| Unique AES codes (Rust)        | >= 27  | < 27       |
| Unique AES codes (Python)      | >= 27  | < 27       |
| Unique AES codes (JS/TS)       | >= 27  | < 27       |
| Self-lint violations           | 0      | > 0        |
| Doc-consistency gate failures  | 0      | > 0        |

> The "Unique AES codes" threshold is the reconciled scan-visible count from
> §1/§3.2 (27 of the 32 total AES rules; AES601–AES605 are doc-only). The CI
> "AES Codes Check" step enforces exactly this value with no lower safety
> margin, so a detection regression in any one of the 27 scan-visible rules
> fails CI (QA #636). When rules are added/removed, update §3.2, this table,
> §1, and the `ci.yml` threshold in lockstep.

### 3.2 Per-Rule Detection Matrix

Every AES rule MUST produce at least 1 violation in the test workspaces.
If any rule produces 0 violations, the test project is missing a trigger file.

| Rule   | Description                            | Rust | Python | JS/TS |
| ------ | -------------------------------------- | ---- | ------ | ----- |
| AES101 | Naming convention                      | ✓   | ✓     | ✓    |
| AES102 | Suffix/prefix validation               | ✓   | ✓     | ✓    |
| AES201 | Layer dependency violation             | ✓   | ✓     | ✓    |
| AES202 | Mandatory import missing               | ✓   | ✓     | ✓    |
| AES203 | Unused import                          | ✓   | ✓     | ✓    |
| AES204 | Dummy import / function                | ✓   | ✓     | ✓    |
| AES205 | Circular dependency                    | ✓   | ✓     | ✓    |
| AES301 | Max line count exceeded                | ✓   | ✓     | ✓    |
| AES302 | Min line count below                   | ✓   | ✓     | ✓    |
| AES303 | Missing definitions / dead inheritance | ✓   | ✓     | ✓    |
| AES304 | Bypass detection                       | ✓   | ✓     | ✓    |
| AES305 | Duplicate code                         | ✓   | ✓     | ✓    |
| AES401 | Taxonomy purity                        | ✓   | ✓     | ✓    |
| AES402 | Contract primitives                    | ✓   | ✓     | ✓    |
| AES403 | Capability implementation              | ✓   | ✓     | ✓    |
| AES404 | Utility purity                         | ✓   | ✓     | ✓    |
| AES405 | Agent composition, delegation, blocks   | ✓   | ✓     | ✓    |
| AES406 | Surface passive role                   | ✓   | ✓     | ✓    |
| AES501 | Taxonomy orphan                        | ✓   | ✓     | ✓    |
| AES502 | Contract orphan                        | ✓   | ✓     | ✓    |
| AES503 | Capabilities orphan                    | ✓   | ✓     | ✓    |
| AES504 | Utility orphan                         | ✓   | ✓     | ✓    |
| AES505 | Agent orphan                           | ✓   | ✓     | ✓    |
| AES506 | Surface orphan                         | ✓   | ✓     | ✓    |
| AES601 | FR Format                              | ✓   |       |      |
| AES602 | Section Structure                      | ✓   |       |      |
| AES602 | Section Structure — level-3 template parity (`h3_off_template`) | ✓ |  |  |
| AES603 | Spec Purity                            | ✓   |       |      |
| AES604 | Crosslinks                             | ✓   |       |      |
| AES605 | Doc Heading Structure                  | ✓   |       |      |
| AES701 | Shared folder purity                   | ✓   | ✓     | ✓    |
| AES702 | Feature folder health                  | ✓   | ✓     | ✓    |
| AES703 | Surface folder purity                  | ✓   | ✓     | ✓    |

### 3.3 Negative Tests (must produce 0 violations)

| #  | Scenario                                                | Expected                                        |
| -- | ------------------------------------------------------- | ----------------------------------------------- |
| 1  | Barrel file (`mod.rs`, `__init__.py`, `index.ts`) | 0 violations (skipped)                          |
| 2  | File in`exceptions` list                              | 0 violations (skipped)                          |
| 3  | Config`architecture.enabled: false`                   | 0 violations (all rules disabled)               |
| 4  | Rule`AES201.enabled: false`                           | 0 AES201 violations (other rules still run)     |
| 5  | Clean, well-structured file                             | 0 violations                                    |
| 6  | `pub use` re-export in barrel file                    | 0 violations (not flagged as unused/dummy)      |
| 7  | `unwrap_or_default()` usage                           | 0 AES304 violations (safe variant)              |
| 8  | Import inside`#[cfg(test)]` block                     | 0 violations (conditional skip)                 |
| 9  | Root layer file (`root_*`)                            | 0 role-rule violations (skipped)                |
| 10 | File with`parse_ok = false`                           | PARSE_WARN emitted, file skipped for AES checks |

### 3.4 Exit Code Tests

| #  | Scenario                                      | Expected Exit Code               |
| -- | --------------------------------------------- | -------------------------------- |
| 1  | `scan` on clean project                     | 0                                |
| 2  | `scan` on workspaces-bad (violations found) | 1                                |
| 3  | `scan` on nonexistent path                  | 2                                |
| 4  | `scan` with invalid arguments               | 2                                |
| 5  | `security` without cargo-audit installed    | 3 + visible scanner-missing warning |
| 6  | `security` on pure Python without `pip-audit` | 3 + visible `pip-audit` warning; never silent `findings: []` |
| 7  | `security` on JS/TS lockfile without npm      | 3 + visible `npm-audit` warning; never a clean result |
| 8  | `ci --threshold 0` with violations          | 1                                |
| 9  | `ci --threshold 100` with few violations    | 0                                |
| 10 | `fix --dry-run` with violations             | 0 (preview only)                 |
| 11 | `doctor` with all tools installed           | 0                                |
| 12 | `doctor` with missing tools                 | 0 (missing tools listed in body) |

### 3.5 Test Suite Quality Thresholds

§3.1 gates the AES self-lint surface; this section gates the test suite itself.
Meaningful targets for the quality metrics reporting required by the QA cycle
(QA #650), cross-referenced from the "Current Quality Status" section in
[DEPLOY.md](DEPLOY.md):

| Criteria                                        | PASS | FAIL            |
| ----------------------------------------------- | ---- | --------------- |
| `cargo nextest run --workspace` pass rate       | 100% | any failing test |
| `cargo test --doc --workspace` pass rate        | 100% | any failing test |
| `#[ignore]`d tests without a linked issue + expiry date | 0 | >= 1         |
| Open CRITICAL defects before release             | 0    | >= 1            |

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

Persona-level end-to-end acceptance scenarios verifying business outcomes beyond synthetic rule fixtures:

| Persona | Scenario | Test Data / Workflow | Expected Business Outcome | Sign-off Status | Verification Evidence |
|---|---|---|---|---|---|
| **Developer** | Local onboarding: clone repo, initialize config, verify toolchain, and run first clean architecture scan | Non-fixture real monorepo (`lint-arwaky` root) | Working CI gate & clean local scan reached in < 5 minutes | Verified | `8d4342a` (`init` → `doctor` → `check .`: 0 violations) |
| **DevOps / CI Engineer** | Pipeline gating: integrate `ci --threshold` into CI workflow | Monorepo with mixed severity violations | Pipeline blocks below threshold (exit 1), allows clean passes (exit 0) | Verified | `8d4342a` (`ci . --threshold 0`: exit 0; bad workspaces: exit 1) |
| **AI Agent** | Autonomous self-healing: detect AES violations and invoke auto-fix via MCP tools | Mixed workspace with known AES203 unused imports & AES304 bypasses | Violations fixed automatically via `execute_command`; `health_check` confirms clean state | Verified | `8d4342a` (MCP `execute_command fix` + `scan`: clean outcome) |

---

## 5. Release Eligibility Checklist

The product-engineering release gate lives in this checklist (5.1-5.8); the dependency
map and risk tracking live in `ROADMAP.md`'s Dependencies and Risk Register sections;
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
- [ ] **Criteria**: All 27 unique scan-visible AES codes detected per language (Section 3.2).
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

  **Criteria**: Responds in < 2 seconds with complete list of 5 registered
  MCP tools (`execute_command`, `list_commands`, `read_skill`, `health_check`,
  `get_config`).

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

- Do not demo the TUI's gated destructive actions (`F` fix-live, `H` install-hook,
  `U` uninstall-hook, `install`, `init`) until #552 closes — the confirm-gate currently
  re-arms instead of executing (PE-4-01 / #626).
- Scope any TUI portion of the demo to `workspaces-good`/`workspaces-bad` until #606
  closes — see the Section 1 demo note (PE-4-02 / #627).
- This script is also linked from `README.md`'s Quick Start section.

### 5.6 Artifact Manifest

Per-release artifact inventory, populated at tag time and retained for rollback
(cross-referenced from `DEPLOY.md`'s Deploy checklist and Rollback Record):

| Release | Artifact | Platform | Checksum File | Provenance Attestation | Retained Until |
|---|---|---|---|---|---|
| v3.7.0 | `lint-arwaky-cli`, `lint-arwaky-mcp` | linux x86_64 | Not published (see `DEPLOY.md` installation notes / #631) | Generated by `release.yml` (`provenance` job) | — (not yet recorded) |
| v3.7.1 | *(pending — see `ROADMAP.md` Risk Register, PE-3-01 / #623)* | — | — | — | — |

### 5.7 Role Sign-offs

Role sign-offs are tracked in `DEPLOY.md`'s Release Sign-off Checklist (Product,
Engineering, QA, Documentation, Operations domains), not duplicated here. See PE-5-01
(#629) for the record and its caveats.

### 5.8 Dependency Map

The crate-to-crate and crate-to-external-client dependency map is tracked in
`ROADMAP.md`'s Dependencies section, not duplicated here. See PE-2-01 (#620) and
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
