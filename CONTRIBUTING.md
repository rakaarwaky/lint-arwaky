# Contributing to Auto Linter

> This guide covers everything you need to start contributing effectively.

## Why Contribute

| Aspect                         | Benefit                                     |
| ------------------------------- | ------------------------------------------- |
| **Real-world impact**     | Your code supports teams using automated quality tooling |
| **Skill development**     | Practice with pytest, async, MCP, and 6-domain architecture |
| **Open source experience**      | Build portfolio and gain collaborative development experience |
| **Community** | Join an active open-source project with regular contributors |
| **Learning opportunity**  | Study a well-architected codebase with clear domain boundaries |

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Setup](#setup)
- [Architecture](#architecture)
- [How to Add an Adapter](#how-to-add-an-adapter)
- [How to Add a CLI Command](#how-to-add-a-cli-command)
- [How to Add an MCP Tool](#how-to-add-an-mcp-tool)
- [Testing](#testing)
- [Code Style](#code-style)
- [Pull Request Process](#pull-request-process)

---

## Prerequisites

- Python >= 3.12
- uv (recommended) or pip
- Git
- Familiarity with:
  - Python asyncio
  - Click (CLI framework)
  - mcp (MCP protocol library)
  - pytest

---

## Setup

```bash
# Clone
git clone https://github.com/rakaarwaky/auto_linter.git
cd auto_linter

# Install with uv (recommended)
uv sync

# Or with pip
pip install -e ".[dev]"

# Verify installation
python3 -m pytest tests/ -q
# Expected: 1500+ passed

# Check version
python3 -m surfaces.cli_main_entry version
# Expected: 1.7.0
```

---

## Architecture

### 6-Domain Model

```
src/
  agent/              Wiring layer -- DI container, managers, orchestrators
  capabilities/       Thinking layer -- analysis logic, processors, evaluators
  contract/           Interface layer -- ports, protocols, aggregates
  infrastructure/     Toolbox layer -- linter adapters, providers, clients
  surfaces/           Interface layer -- CLI commands, MCP handlers
  taxonomy/           Language layer -- Value Objects (VOs), entities, events
```

### Dependency Rules

Imports must follow AES layer rules:

```
agent          --> taxonomy, contract, infrastructure, capabilities  OK
surfaces       --> taxonomy, contract(io), agent                     OK
surfaces       --> infrastructure, capabilities                      NO
capabilities   --> taxonomy, contract(protocol)                      OK
capabilities   --> infrastructure, surfaces, agent                   NO
infrastructure --> taxonomy, contract(port)                          OK
infrastructure --> capabilities, surfaces, agent                     NO
contract       --> taxonomy                                          OK
contract       --> agent, capabilities, infrastructure, surfaces     NO
taxonomy       --> taxonomy                                          OK
taxonomy       --> agent, capabilities, infrastructure, surfaces, contract NO
```

The `ArchComplianceAdapter` enforces these rules automatically.
Run `auto-lint check src/` to verify no violations.

### Key Interfaces & Mandatory Inheritance

To prevent architectural bypasses, every logic file (except `__init__.py`) **must** define a class that inherits from its corresponding contract:

| Layer              | Suffix Rule | Base Contract   | Example File                 |
| ------------------ | ----------- | --------------- | ---------------------------- |
| **Agent**          | Strict      | `_aggregate.py`  | `analysis_orchestrator_aggregate.py` |
| **Capabilities**   | Flexible    | `_protocol.py`  | `arch_compliance_analyzer.py` |
| **Infrastructure** | Flexible    | `_port.py`      | `python_ruff_adapter.py`      |
| **Surfaces**       | Strict      | N/A             | `cli_check_command.py`        |
| **Taxonomy**       | Strict      | `_vo.py` etc.   | `adapter_name_vo.py`         |

---

## How to Add an Adapter

### 1. Create the adapter file

File: `src/infrastructure/python_mytool_adapter.py` (Must be exactly 3 words)

Implement the appropriate port from `contract/`.

```python
"""Adapter for MyTool."""
from contract import ILinterAdapterPort
from taxonomy import LintResultEntity

class PythonMytoolAdapter(ILinterAdapterPort):
    def name(self) -> str:
        return "my-tool"

    def scan(self, path: str) -> list[LintResultEntity]:
        # Implementation
        ...
```

### 2. Register in DI container

File: `src/agent/dependency_injection_container.py`

Add your adapter to the container initialization:

```python
from infrastructure.python_mytool_adapter import PythonMytoolAdapter

self.adapters = [
    ...
    PythonMytoolAdapter(),
]
```

### 3. Add tests

File: `tests/infrastructure/test_my_tool_adapter.py`

```python
from unittest.mock import patch, MagicMock
from infrastructure.python_mytool_adapter import PythonMytoolAdapter

def test_my_tool_name():
    assert PythonMytoolAdapter().name() == "my-tool"

@patch("subprocess.run")
def test_my_tool_scan(mock_run):
    mock_run.return_value = MagicMock(stdout="...", stderr="", returncode=0)
    results = PythonMytoolAdapter().scan("test.py")
    assert isinstance(results, list)
```

### 4. Run tests

```bash
python3 -m pytest tests/infrastructure/test_my_tool_adapter.py -v
python3 -m pytest tests/ --cov=src --cov-report=term-missing
```

---

## How to Add a CLI Command

### 1. Choose the right module

All CLI modules are in `src/surfaces/` and must follow the `cli_<name>_command.py` pattern.

| Module                    | Purpose                                         |
| ------------------------- | ----------------------------------------------- |
| `cli_core_command.py`     | check, scan, fix, report, version               |
| `cli_analysis_command.py` | complexity, duplicates, trends, dependencies    |
| `cli_dev_command.py`      | diff, suggest, ignore, config, export, import   |
| `cli_setup_command.py`    | setup init, setup hermes, setup doctor          |

### 2. Add the command

Commands must be wrapped in a Handler or Controller class in the surface layer.

```python
@click.command()
@click.argument('path')
def my_command(path):
    """Description."""
    container = AgentContainerRegistry.get_instance()
    # Delegate to Agent layer
    container.orchestrator.run_logic(path)
```

### 3. Register in catalog

File: `src/surfaces/mcp_command_catalog.py`

Add to `_COMMAND_CATALOG`:

```python
"my-command": {
    "description": "What it does",
    "example": "auto-lint my-command /path",
},
```

### 4. Add tests

Test via CliRunner (integration) or test the underlying
use case directly (unit):

```python
from click.testing import CliRunner
from surfaces.cli_core_command import cli

def test_my_command():
    runner = CliRunner()
    result = runner.invoke(cli, ['my-command', 'test.py'])
    assert result.exit_code == 0
```

---

## How to Add an MCP Tool

### 1. Add to registry

File: `src/surfaces/mcp_tools_store.py` (3 words)

```python
def register_my_tool(server):
    @server.tool()
    async def my_tool_handler(param: str) -> str:
        """Description."""
        # implementation
```

### 2. Add tests

```python
import json
import pytest

@pytest.mark.asyncio
async def test_my_tool():
    from surfaces.mcp_tools_store import register_my_tool
    # Setup test server/mock
    ...
```

---

## Testing

### Run all tests

```bash
python3 -m pytest tests/ -v --tb=short
```

### Run with coverage

```bash
python3 -m pytest tests/ --cov=src --cov-report=term-missing
```

### Run specific test file

```bash
python3 -m pytest tests/infrastructure/test_python_mytool_adapter.py -v
```

### Test structure

```
tests/
  agent/                  DI container tests
  capabilities/           Use case and formatter tests
  infrastructure/         Adapter tests (mock subprocess)
  surfaces/               CLI and MCP tool tests
  taxonomy/               Model and utility tests
  conftest.py             Shared fixtures
```

### Rules

- Every new function needs at least one test
- Mock external tools (subprocess, filesystem, network)
- Test both success and failure paths
- Use `@pytest.mark.asyncio` for async tests

---

## Code Style

### Formatting

```bash
# Auto-format
auto-lint fix src/
```

### Conventions

- **Naming**: Strict 3-word underscore-separated filenames (`word1_word2_word3.py`).
- **Classes**: Mandatory class definitions for all logic modules. No standalone functions at module level.
- **Lines**: Files must be 10-300 lines.
- **Score**: 100/100 architectural compliance required for all PRs.
- **Bypasses**: `noqa`, `type: ignore`, and `nosec` are strictly forbidden.

### AES006 Primitive Type Policy (v1.9.4)

The project applies a **layer-granular** primitive enforcement strategy:

| Layer | `no_primitives` | Policy |
|---|---|---|
| `contract` | `true` | All port/protocol/aggregate signatures must use taxonomy Value Objects |
| `taxonomy(entity\|error\|event)` | `true` | All entity/error/event attributes must use Value Objects |
| `taxonomy(vo)` | `false` | VO internals may use primitives as underlying storage |
| `infrastructure` | `false` | Adapters may use primitive types as supporting/local types |
| `capabilities` | `false` | Capability implementations may use primitive types internally |
| `surfaces` | `false` | Surface/CLI handlers may use primitive types for I/O parsing |

**Rationale**: Enforcing strict Value Objects in implementation adapter layers creates unnecessary boxing overhead and conflicts with third-party library APIs (e.g., FastMCP, Click, asyncio). Domain contracts and taxonomy definitions remain strictly typed to prevent boundary leakage.

---

## Pull Request Process

### Before Submitting

1. **Run tests**: `python3 -m pytest tests/`
2. **Run Architecture Audit**: `auto-lint check src/`
   **Score must be 100.0/100.0**.
3. **Update docs**: Ensure `README.md`, `SKILL.md`, and `PRD.md` match your changes.

### PR Description Template

```
## What
Brief description of what this PR does.

## Why
Why is this change needed?

## How
How does it work? Any design decisions?

## Testing
How was it tested? What test cases were added?

## Checklist
- [ ] Tests passing
- [ ] 100.0/100.0 architecture score
- [ ] Coverage not decreased
- [ ] Docs updated if needed
```

### Review Criteria

- Code follows architecture rules (no cross-layer violations)
- Tests cover both happy path and error cases
- No hardcoded paths or environment assumptions
- Subprocess calls use absolute paths to executables
- Error messages are actionable (tell the user what to do)

---

## Questions?

Open an issue on GitHub or contact the maintainer.
