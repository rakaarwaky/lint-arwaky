# Deployment Guide — Lint Arwaky v1.10.9

**Status**: PRODUCTION-READY — self-lint target ships clean.

---

## Prerequisites

| Requirement    | Minimum                  | Recommended                 |
| -------------- | ------------------------ | --------------------------- |
| Rust toolchain | 1.70 (edition 2021)      | 1.78+ (stable)              |
| RAM            | 256 MB                   | 1 GB+ (for large codebases) |
| Disk           | 50 MB (release binaries) | -                           |
| OS             | Linux                    | Linux x86_64                |

No external services required. The MCP server speaks JSON-RPC 2.0 over stdin/stdout and has no network dependencies.

---

## Installation

### Option 1: Installer script

```bash
# Linux / macOS
curl -sSL https://raw.githubusercontent.com/rakaarwaky/lint-arwaky/main/install.remote.sh | bash
```

The installer places `lint-arwaky-cli` and `lint-arwaky-mcp` in `~/.local/bin/` (or `/usr/local/bin/` when run as root).

### Option 2: From source (recommended for contributors)

```bash
git clone https://github.com/rakaarwaky/lint-arwaky.git
cd lint-arwaky
cargo build --release

# Binaries produced at:
#   target/release/lint-arwaky-cli
#   target/release/lint-arwaky-mcp

# Optionally symlink into PATH
ln -s "$PWD/target/release/lint-arwaky-cli" ~/.local/bin/
ln -s "$PWD/target/release/lint-arwaky-mcp" ~/.local/bin/
```

### Option 3: Cross-compile

```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin

# Windows MSVC
cargo build --release --target x86_64-pc-windows-msvc
```

### Verify installation

```bash
lint-arwaky-cli version
# Expected: Lint Arwaky v1.10.9 (AES Semantic Builder)

lint-arwaky-cli setup doctor
# Expected: cargo: OK (cargo X.Y.Z), binary: OK (/path/to/lint-arwaky-cli)
```

---

## MCP Server Setup

The MCP server is a self-contained binary that speaks JSON-RPC 2.0 over stdin/stdout using the `2024-11-05` protocol version.

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
lint-arwaky-cli setup mcp-config --client claude
```

### Configure for VS Code (MCP extension)

```bash
lint-arwaky-cli setup mcp-config --client vscode
```

### Configure for Hermes Agent

```bash
# Add lint-arwaky to ~/.hermes/config.toml
lint-arwaky-cli setup hermes

# To remove:
lint-arwaky-cli setup hermes --remove
```

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
lint-arwaky-cli setup doctor   # Self-diagnose (Rust toolchain, binary path)
lint-arwaky-cli adapters       # List active linter adapters
```

The `health_check` MCP tool reports on adapter health and system state.

---

## Usage

```bash
# Full self-lint
lint-arwaky-cli check .

# Targeted scans
lint-arwaky-cli security .       # vulnerability scan
lint-arwaky-cli complexity .     # complexity hotspots
lint-arwaky-cli duplicates .     # duplication detection
lint-arwaky-cli dependencies .   # Cargo.toml listing

# CI mode with exit codes
lint-arwaky-cli ci . --threshold 80

# Auto-fix (where safe)
lint-arwaky-cli fix .

# Reports
lint-arwaky-cli report . --output-format sarif
lint-arwaky-cli report . --output-format junit
lint-arwaky-cli report . --output-format json
```

---

## Configuration

Default configuration is hard-coded in `src-rust/capabilities/architecture_lint_handler::default_aes_config()`. To override, generate a YAML file:

```bash
lint-arwaky-cli setup init
# Creates lint_arwaky.config.yaml in the current directory
```

Key sections:

```yaml
thresholds:
  score_target: 100.0      # target self-lint score
  max_complexity: 10       # per-function branch cap

adapters:
  rust_linter:  { enabled: true, weight: 1.0 }
  python_ruff:  { enabled: true, weight: 1.0 }
  python_mypy:  { enabled: true, weight: 1.0 }
  python_bandit:{ enabled: true, weight: 1.0 }
  python_metrics:{ enabled: true, weight: 1.0 }
  javascript:   { enabled: true, weight: 1.0 }
  architecture: { enabled: true, weight: 3.0 }   # AES rules carry 3x weight
```

---

## Production Deployment Checklist

### Before Deploy

- [ ] `cargo build --release` succeeds
- [ ] `cargo test --workspace` passes
- [ ] `cargo run --bin lint-arwaky-cli -- check .` reports 0 CRITICAL findings
- [ ] `cargo fmt --all` and `cargo clippy --all-targets -- -D warnings` clean
- [ ] `lint-arwaky-cli version` returns `1.10.9`
- [ ] `lint-arwaky-cli setup doctor` reports no issues
- [ ] `lint-arwaky-mcp` responds to `tools/list` with the 5 expected tools
- [ ] `health_check` MCP tool returns all adapters healthy

### Deploy

- [ ] Bump version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Build release: `cargo build --release`
- [ ] Tag the release: `git tag v1.10.9`
- [ ] Push tag: `git push origin v1.10.9`
- [ ] Run installer smoke-test on a clean machine

### Post-Deploy

- [ ] `lint-arwaky-cli --version` succeeds on the target machine
- [ ] MCP server starts and responds to `tools/list` within 2 seconds
- [ ] Sample lint run on a known-good project completes without errors

---

## Rollback Plan

Reinstall the previous release:

```bash
cargo install --git https://github.com/rakaarwaky/lint-arwaky --tag v1.10.9
```

Or rebuild from a specific tag:

```bash
git checkout v1.10.9
cargo build --release
```

Restart any running MCP client (Claude Desktop, VS Code, Hermes).

---

## Support

- Repository: https://github.com/rakaarwaky/lint-arwaky
- Issues: https://github.com/rakaarwaky/lint-arwaky/issues
- Documentation: [README.md](README.md), [SKILL.md](SKILL.md), [RULES_AES.md](RULES_AES.md), [ARCHITECTURE.md](ARCHITECTURE.md)
