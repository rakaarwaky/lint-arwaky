# Deployment Guide — Lint Arwaky

**Status**: RELEASE CANDIDATE (v3.7.2) — pending final sign-offs in checklist below. Tracked in `ROADMAP.md`.

> **Known gap (PE-3-01 / #623):** `Cargo.toml` and `CHANGELOG.md` already declare 3.7.2,
> but no `v3.7.2` GitHub tag or Release exists yet. The prior 3.7.1 candidate was also
> never tagged. The sign-off table below evidences pipeline-gate health; it is not,
> by itself, authorization to tag 3.7.2 — see `ROADMAP.md` Risk Register before cutting
> the tag.

---

## Release Sign-off Checklist

The "Deploy" checklist below requires "Product, Engineering, QA, Documentation, and
Operations sign-offs" verbatim. The table maps each of those five named domains to the
role that actually owns the evidence on this project today (PE-5-01 / #629 — this table
previously used role names that did not match the Deploy checklist's literal text):

| Domain | Role / Owner | Sign-off Criterion | Status | Evidence (Commit / Artifact) | Date |
|---|---|---|---|---|---|
| Product | Business Analyst (`@raka`) | Requirements/scope audit issues resolved with verifiable evidence; business outcome metrics current | Approved | `8d4342a` (PR #520, PR #567) | 2026-09-30 |
| Engineering | Architect + Tech Lead (`@raka`) | 34 AES rules pass with 0 internal violations on self-lint; `cargo build --release` succeeds | Approved | `8d4342a` (`lint-arwaky-cli check .`) | 2026-09-30 |
| QA | Tech Lead (`@raka`) | Full test suite & negative test matrix green (`TEST.md` Sections 3-4) | Approved | `8d4342a` (`cargo test --workspace`) | 2026-09-30 |
| Documentation | Business Analyst (`@raka`) | `lint-arwaky-cli docs .` reports 0 document invariant violations | Approved | `8d4342a` (`lint-arwaky-cli docs .`) | 2026-09-30 |
| Operations | Security Engineer + DevOps Engineer (`@raka`) | `SECURITY.md` supported versions & cargo-audit clean; binaries built, checksums registered | Approved | `8d4342a` (`cargo-audit`, PR #516; `target/release/lint-arwaky-*`) | 2026-09-30 |

**Caveat (PE-5-01 / #629, PE-3-01 / #623):** this record was captured at commit `8d4342a`
before the BA/SA/UX/ARCH/BE/FE/PE audit backlog (#522-#631) existed and before `v3.7.1`'s
tag/Release status was confirmed absent. It is not release authorization for `v3.7.1` as
it stands today. Re-collect all five sign-offs against the actual release commit once the
tag ambiguity (Risk Register, PE-3-01) and the CRITICAL findings cited against FR-DISP,
FR-CONF, FR-GITH, FR-TUIC, and FR-MCPP in `ROADMAP.md`'s Feature Roll-up are resolved.

---

## Prerequisites

| Requirement    | Minimum                  | Recommended                 |
| -------------- | ------------------------ | --------------------------- |
| Rust toolchain | 1.85 (edition 2024)      | 1.85+ (stable)              |
| RAM            | 256 MB                   | 1 GB+ (for large codebases) |
| Disk           | 50 MB (release binaries) | -                           |
| OS             | Linux                    | Linux x86_64                |

No external services required. The MCP server speaks JSON-RPC 2.0 over stdin/stdout and has no network dependencies.

---

## Installation

### Option 1: Installer script

```bash
# Linux
bash scripts/install.sh
```

The installer builds from source and places binaries in `target/release/`.

**Remote mode (download pre-built binary):** `bash scripts/install.sh --remote` downloads a pre-built archive from the GitHub release. The release workflow does not currently publish a checksum file, so the installer prints the archive's SHA-256 after download. Verify it manually:

```bash
# After the remote install prints "SHA-256: <digest>", compare against the
# checksum published in the GitHub release notes/assets for the same tag.
sha256sum /tmp/lint-arwaky.tar.gz
```

Only proceed to extraction/execution after the digest matches. Until a checksum file is published in the release, this manual verification step is the integrity control for the remote install path.

Release CI already generates SLSA build-provenance attestations. When the GitHub
CLI is available, optionally verify that a downloaded binary was built by this
repository's CI (a stronger guarantee than checksum equality alone):

```bash
gh attestation verify lint-arwaky-cli --repo rakaarwaky/lint-arwaky
```

This consumes the attestation already produced by the release workflow; it does
not require additional CI infrastructure. Authentication may be required by the
GitHub CLI, so checksum verification remains the minimum mandatory check.

### Option 2: From source (recommended for contributors)

```bash
git clone https://github.com/rakaarwaky/lint-arwaky.git
cd lint-arwaky
cargo build --release

# Binaries produced at:
#   target/release/lint-arwaky-cli
#   target/release/lint-arwaky-mcp
#   target/release/lint-arwaky-tui

# Optionally symlink into PATH. `lint-arwaky` and `la` are name aliases of the
# single compiled CLI binary (not separate [[bin]] targets), so install them as
# symlinks too — scripts/install.sh does this automatically via
# `install_alias_symlinks`.
ln -s "$PWD/target/release/lint-arwaky-cli" ~/.local/bin/
ln -s "$PWD/target/release/lint-arwaky-cli" ~/.local/bin/lint-arwaky
ln -s "$PWD/target/release/lint-arwaky-cli" ~/.local/bin/la
ln -s "$PWD/target/release/lint-arwaky-mcp" ~/.local/bin/
```

### Option 3: Cross-compile

```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin

# Windows MSVC (experimental build-only target; unsupported runtime until WS-10 closes)
cargo build --release --target x86_64-pc-windows-msvc
```

### Verify installation

```bash
lint-arwaky-cli version
# Expected: lint-arwaky 3.7.1

lint-arwaky-cli doctor
# Expected: cargo: OK (cargo X.Y.Z), binary: OK (/path/to/lint-arwaky-cli)
```

---

## MCP Server Setup

The MCP server is a self-contained binary that speaks JSON-RPC 2.0 over stdin/stdout using the `2024-11-05` protocol version. Its clients and tool-call arguments are governed by the local-process trust boundary documented in [SECURITY.md](SECURITY.md#scope-and-mcp-trust-model); in particular, paths supplied to `execute_command` are untrusted even when the client is local.

### Configure for Claude Desktop

Edit `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "lint-arwaky": {
      "command": "lint-arwaky-mcp",
      "args": []
    }
  }
}
```

Or print the config snippet from the CLI:

```bash
lint-arwaky-cli mcp-config --client claude
```

### Configure for VS Code (MCP extension)

```bash
lint-arwaky-cli mcp-config --client vscode
```

### Configure for Hermes Agent

```bash
# Print configuration snippet for Hermes:
lint-arwaky-cli mcp-config --client hermes
```

### External Client Compatibility

Three external, out-of-repo clients integrate against the 5-tool MCP surface
(`execute_command`, `list_commands`, `read_skill`, `health_check`, `get_config`) at
protocol version `2024-11-05`. No automated test exercises any of them today
(PE-2-02 / #621), so this table is a manual verification record, not a passing
CI gate:

| Client | Protocol Version Verified | Last Verified | Notes |
|---|---|---|---|
| Claude Desktop | 2024-11-05 | — (unverified) | Config via `mcp-config --client claude` |
| VS Code MCP extension | 2024-11-05 | — (unverified) | Config via `mcp-config --client vscode` |
| Hermes Agent | 2024-11-05 | — (unverified) | Config via `mcp-config --client hermes`; ownership of this integration is not documented — confirm with whoever maintains it before depending on this table |

**Process note:** whoever changes the MCP tool surface (adds, removes, or renames a tool,
or bumps the protocol version) updates this table in the same PR and states in the
`CHANGELOG.md` entry which of the three clients above were re-validated, if any.

### Smoke-test the MCP server manually

```bash
# tools/list
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | lint-arwaky-mcp

# health_check
echo '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"health_check","arguments":{}}}' \
  | lint-arwaky-mcp
```

---

## Health Check Commands

```bash
lint-arwaky-cli version        # Version check
lint-arwaky-cli doctor   # Self-diagnose (Rust toolchain, binary path)
lint-arwaky-cli adapters       # List active linter adapters
```

See [TEST.md](TEST.md) for the full production checklist and [README.md](README.md) for user-facing usage.

The `health_check` MCP tool reports on adapter health and system state.

---

## Usage

```bash
# Full self-lint
lint-arwaky-cli scan .

# Deep directory scan
lint-arwaky-cli scan <path>

# CI mode with exit codes
lint-arwaky-cli ci

# Auto-fix (where safe)
lint-arwaky-cli fix

# Orphan check
lint-arwaky-cli orphan <path>

# File watching
lint-arwaky-cli watch .
```

---

## Configuration

```bash
lint-arwaky-cli init
# Creates lint_arwaky.config.yaml in the current directory
```

## Production Deployment Checklist

### Current Quality Status (update before each release attempt)

Last updated: 2026-10-01

- **Open defects:** 90+ logged across audit cycles (#522–#650), including 11+ confirmed CRITICAL — see PE #617. Severity counts are derived from issue **titles** (`[ROLE][SEVERITY] …`), not labels, until #618 is resolved (label writes fail with `Resource not accessible by integration`).
- **CI detection-threshold gap — CLOSED:** the AES codes gate now enforces the reconciled scan-visible count (29 of 34 rules, per `TEST.md` §3.1/§3.2) with no safety margin — see QA #636.
- **Self-lint gate integrity — CLOSED:** the `check .` step now fails loudly on unparseable output or non-zero exit instead of substituting `0` violations — see QA #642.
- **Coverage measurement:** now computed in CI via `cargo-llvm-cov` (`coverage` job summary) — see QA #643; no minimum percentage is enforced yet.
- **External-tool end-to-end coverage — CLOSED:** CI installs ruff/mypy/bandit/eslint/prettier/tsc and asserts tool-native findings — see QA #637.
- **Doctest execution — CLOSED:** `cargo test --doc --workspace` runs as a dedicated CI step — see QA #638/#644.
- **Release recommendation:** **NOT READY** — open CRITICAL defects and the missing release sign-off record (PE #629) block any release-ready verdict regardless of the above gate improvements.

### Before Deploy

- [ ] `cargo build --release` succeeds
- [ ] `cargo test --workspace` passes
- [ ] `cargo run --bin lint-arwaky-cli -- check .` reports 0 CRITICAL findings
- [ ] `cargo fmt --all` and `cargo clippy --all-targets -- -D warnings` clean
- [ ] `lint-arwaky-cli version` returns the candidate version recorded in `Cargo.toml`
- [ ] `lint-arwaky-cli doctor` reports no issues
- [ ] `lint-arwaky-mcp` responds to `tools/list` with all 5 expected tools
- [ ] `health_check` MCP tool reports adapter availability, including missing optional tools
- [ ] Bad Rust, Python, and TypeScript fixtures produce findings; good fixtures produce 0 findings
- [ ] Demo configuration, external-tool availability, and expected exit codes are recorded

### Deploy

- [ ] Bump version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Build release: `cargo build --release`
- [ ] Tag the release: `git tag vX.Y.Z`
- [ ] Push tag: `git push origin vX.Y.Z`
- [ ] Run installer smoke-test on a clean machine
- [ ] Record candidate tag, binary inventory, platform, checksum, and provenance
- [ ] Obtain Product, Engineering, QA, Documentation, and Operations sign-offs (see the Release Sign-off Checklist above; re-collect against the actual release commit)
- [ ] Record owner, trigger threshold, retained artifact, and post-rollback verification in the Rollback Record below

### Post-Deploy

- [ ] `lint-arwaky-cli --version` succeeds on the target machine
- [ ] MCP server starts and responds to `tools/list` within 2 seconds
- [ ] Sample lint run on a known-good project completes without errors

---

## Rollback Plan

Reinstall the previous known-good release tag recorded in the release checklist:

```bash
cargo install --git https://github.com/rakaarwaky/lint-arwaky --tag <previous-stable-tag>
```

Rollback owner, trigger threshold, retained artifact, and post-rollback verification must be recorded before the candidate is marked Released.

### Rollback Record (current candidate)

Populated before the current candidate (resolving PE-3-01's tag ambiguity first) is
marked Released (PE-5-02 / #630):

- **Owner:** _(unassigned — fill before release; whoever holds release sign-off authority per the Release Sign-off Checklist above)_
- **Trigger threshold:** _(unset — e.g. "P0/CRITICAL regression reported within 24h of release")_
- **Retained artifact:** _(unset — e.g. "previous stable tag's binaries + checksum file kept available for N days", tracked alongside the Artifact Manifest in `TEST.md` Section 5.6)_
- **Post-rollback verification:** _(unset — e.g. "`lint-arwaky-cli version` and `doctor` pass on the reinstalled tag; MCP `tools/list` smoke test passes")_

This record is a prerequisite for the "Deploy" checklist's sign-off item above, not just
the procedure in isolation.

Or rebuild from a specific tag:

```bash
git checkout vX.Y.Z
cargo build --release
```

---

## Support

- Repository: <https://github.com/rakaarwaky/lint-arwaky>
- Issues: <https://github.com/rakaarwaky/lint-arwaky/issues>
- Documentation: [README.md](README.md), [RULES_AES.md](RULES_AES.md), [ARCHITECTURE.md](ARCHITECTURE.md)
