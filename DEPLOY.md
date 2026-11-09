# Deployment Guide — Lint Arwaky

**Status**: RELEASED (v3.8.0). Tracked in `ROADMAP.md`.

> **Superseded versions:** 3.7.1 and 3.7.2 were declared in `Cargo.toml` and
> `CHANGELOG.md` but never tagged (PE-3-01 / #623). The previous actual tag was
> `v3.7.0`; everything since first ships as 3.8.0. Closed by that release.

---

## Release Sign-off Checklist

The "Deploy" checklist below requires "Product, Engineering, QA, Documentation, and
Operations sign-offs" verbatim. The table maps each of those five named domains to the
role that actually owns the evidence on this project today (PE-5-01 / #629 — this table
previously used role names that did not match the Deploy checklist's literal text):

| Domain | Role / Owner | Sign-off Criterion | Status | Evidence (commit / CI run) | Date |
|---|---|---|---|---|---|
| Product | Business analyst (`@raka`) | Requirements and scope audit closed with verifiable evidence; outcome metrics recorded in `PRD.md` | Pending — re-collect at tag time | must name the release commit + CI run | — |
| Engineering | Architect + tech lead (`@raka`) | 41 AES rules, 0 self-lint violations on the release tag; `cargo build --release` succeeds | Pending — re-collect at tag time | must name the release commit + CI run | — |
| QA | **Independent reviewer — not yet named** | Full suite and negative-test matrix green on the release tag | **Blocked** | no reviewer other than the release author exists today | — |
| Documentation | Business analyst (`@raka`) | `lint-arwaky-cli docs .` reports 0 document-invariant violations on the release tag | Pending — re-collect at tag time | must name the release commit + CI run | — |
| Operations | Security + DevOps engineer (`@raka`) | `SECURITY.md` supported versions and cargo-audit clean; binaries built and checksums recorded | Pending — re-collect at tag time | must name the release commit + CI run | — |

**Rules for this table.** These replace the withdrawn record that approved all five domains with one
handle against `8d4342a` (2026-09-30): that commit does not exist in this repository, the row set
predates eight minor releases, and a signature from the release author is not review.

- Evidence must be a commit reachable from the release tag plus the CI run URL for that commit. A
  hash that is not an ancestor of the tag is not evidence.
- Product and QA are signed by someone other than the release author. While QA has no independent
  reviewer, QA stays **Blocked** and the release is not eligible.
- Re-collect the table for every release. A stale row is worse than an empty one.

---

## Prerequisites

| Requirement    | Minimum                  | Recommended                |
| -------------- | ------------------------ | ---------------------------|
| Rust toolchain | 1.85 (edition 2024)      | 1.85+ (stable)             |
| RAM            | 256 MB                   | 1 GB+ (for large codebases)|
| Disk           | 50 MB (release binaries) | -                          |
| OS             | Linux                    | Linux x86_64               |

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
# Expected: lint-arwaky $(cat VERSION)   e.g. lint-arwaky 3.16.1

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

The MCP surface is five tools — `execute_command`, `list_commands`, `health_check`,
`get_config`, `docs_scaffold` — at protocol version `2024-11-05`. No automated test exercises any of them today
(PE-2-02 / #621), so this table is a manual verification record, not a passing
CI gate:

| Client | Protocol Version Verified | Last Verified | Notes|
|---|---|---|---|
| Claude Desktop | 2024-11-05 | unsupported — no owner assigned | Config emitted by `mcp-config --client claude`; not validated |
| VS Code MCP extension | 2024-11-05 | unsupported — no owner assigned | Config emitted by `mcp-config --client vscode`; not validated |
| Hermes Agent | 2024-11-05 | unsupported — no owner assigned | Config emitted by `mcp-config --client hermes`; ownership of this integration is undocumented |

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

### Current Quality Status (applies to 3.16.1, staged)

- **Release recommendation:** **NOT ELIGIBLE.** Two prerequisites are unmet: the sign-off table above has no reachable-commit evidence and no independent QA reviewer, and the persona acceptance record in `TEST.md` Section 4 has not been re-run for 3.16.1.
- **Open defects:** read them from the tracker (`is:open label:severity-critical`) instead of a count restated here. The previous "90+ open across #522-#650" figure described a backlog that is fully closed and has been removed — a stale count is worse than a query.
- **Gate status:** self-lint, workspace tests, fmt/clippy, doctests, coverage visibility, external-tool end-to-end checks and the 41-code AES floor are enforced by `.github/workflows/ci.yml`; see `TEST.md` Sections 3.1-3.5 for the thresholds they enforce.
- **Known process gaps:** no 10k-file scan benchmark is recorded yet, and the rollback record below has not been rehearsed.

### Before Deploy

- [ ] `cargo build --release` succeeds
- [ ] `cargo test --workspace` passes
- [ ] `cargo run --bin lint-arwaky-cli -- check .` reports 0 CRITICAL findings
- [ ] `cargo fmt --all` and `cargo clippy --all-targets -- -D warnings` clean
- [ ] `lint-arwaky-cli version` returns the candidate version recorded in `Cargo.toml`
- [ ] `lint-arwaky-cli doctor` reports no issues
- [ ] `lint-arwaky-mcp` responds to `tools/list` with the five contracted tools (`execute_command`, `list_commands`, `health_check`, `get_config`, `docs_scaffold`)
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

- **Owner:** Operations lead (`@raka`).
- **Trigger threshold:** any P0/CRITICAL regression reported within 24 h of a release, or a confirmed case of `ci` passing a workspace it should have failed.
- **Retained artifact:** the previous stable tag's release assets (`lint-arwaky-cli`, `lint-arwaky-mcp`, `lint-arwaky-tui`) plus the installer, retained 90 days.
- **Post-rollback verification:** `lint-arwaky-cli version` reports the restored tag; `doctor` exits 0; the MCP `tools/list` smoke test returns the five contracted tools.
- **Rehearsal:** not yet performed. The rehearsal (rollback 3.16.1 → 3.16.0 on a clean Linux x86_64 host, recording duration and verification output) is a release blocker and is recorded here with its date and result before the candidate is marked Released.

Or rebuild from a specific tag:

```bash
git checkout vX.Y.Z
cargo build --release
```

---

## Support

- Repository: <https://github.com/rakaarwaky/lint-arwaky>
- Issues: <https://github.com/rakaarwaky/lint-arwaky/issues>
- Documentation: [README.md](README.md), [ARCHITECTURE.md](ARCHITECTURE.md)
