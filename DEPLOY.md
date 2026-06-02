# Deployment Guide - MCP Lint-Arwaky

**Version**: 1.9.4
**Python**: 3.12+
**Status**: PRODUCTION-READY — all acceptance criteria PASS.

---

## Prerequisites

| Requirement | Minimum               | Recommended                 |
| ----------- | --------------------- | --------------------------- |
| Python      | 3.12                  | 3.13+                       |
| RAM         | 512 MB                | 2 GB+ (for large codebases) |
| Disk        | 100 MB                | -                           |
| OS          | Linux, macOS, Windows | Linux                       |

No external services required. Runs entirely locally.

---

## Installation

### Option 1: pip

    pip install lint-arwaky

### Option 2: uv (recommended)

    uv tool install lint-arwaky
    # Or run without installing: uvx auto-lint check ./src/

### Option 3: From source

    git clone https://github.com/rakaarwaky/lint-arwaky.git
    cd lint-arwaky
    python -m venv .venv && source .venv/bin/activate
    pip install -e ".[dev]"

### Option 4: Installer scripts

    Linux/macOS: curl -sSL INSTALL_URL | bash
    Windows PowerShell: Invoke-WebRequest -Uri INSTALL_URL | Invoke-Expression

### Verify installation

    auto-lint version
    auto-lint setup doctor

---

## MCP Server Setup

### Configure for Claude Desktop

    auto-lint setup mcp-config --client claude

Manual config in claude_desktop_config.json:

    json
    {
      "mcpServers": {
        "lint-arwaky": { "command": "lint-arwaky" }
      }
    }

### Configure for VS Code

    auto-lint setup mcp-config --client vscode

### Configure for Hermes Agent

    pip install lint-arwaky && auto-lint setup hermes

---

## Health Check Commands

    auto-lint version          # Version check
    auto-lint setup doctor     # Self-diagnose
    auto-lint check ./src/ --dry-run  # Adapter health

Expected: version returns info, doctor reports no issues, health_check tool returns all 4 adapters healthy.

---

## Usage

    auto-lint check ./src/              # Full audit
    auto-lint security ./src/           # Bandit scan
    auto-lint complexity ./src/         # Radon analysis
    auto-lint ci ./src/                 # CI mode with exit codes
    auto-lint fix ./src/                # Auto-apply safe fixes
    auto-lint report ./src/ --output-format sarif -o report.sarif

---

## Configuration

Config file: lint-arwaky.config.python.yaml

    thresholds:
      score_target: 100.0
      max_complexity: 10
    adapters:
      ruff: { enabled: true, weight: 1.0 }
      mypy: { enabled: true, weight: 1.0 }
      bandit: { enabled: true, weight: 1.0 }
      radon: { enabled: true, weight: 1.0 }
      architecture: { enabled: true, weight: 3.0 }

---

## Production Deployment Checklist

### Before Deploy

- [ ] Container.execute() implemented - CLI boots without crash
- [ ] Zero bypass annotations - no noqa or type: ignore in codebase
- [ ] E501 rule active - all line-length violations fixed
- [ ] Self-lint score = 100% - auto-lint check ./src/ passes clean
- [ ] MCP tools respond correctly - end-to-end MCP client test passes
- [ ] Health check green - all 4 adapters report healthy
- [ ] Security scan clean - auto-lint security ./src/ no issues
- [ ] Type safety confirmed - mypy src/ = 0 errors
- [ ] Architecture rules pass - 0 AES violations

### Deploy

- [ ] Package version bumped
- [ ] Changelog updated (CHANGELOG.md)
- [ ] Tests pass: pytest test/
- [ ] Build succeeds: python -m build
- [ ] Upload to PyPI: twine upload dist/*

### Post-Deploy

- [ ] pip install lint-arwaky==version succeeds
- [ ] MCP server starts without errors
- [ ] Health check tool responds within 5 seconds
- [ ] Sample lint run completes on a known-good project

---

## Rollback Plan

    pip install lint-arwaky==previous_version

Restart MCP server / Claude Desktop / VS Code.

---

## Support

- Repository: https://github.com/rakaarwaky/lint-arwaky
- Issues: https://github.com/rakaarwaky/lint-arwaky/issues
- Documentation: SKILL.md, AES_RULES.md, README.md
