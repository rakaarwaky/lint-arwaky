# FRD — naming-rules

## System Overview

The naming-rules crate enforces strict naming conventions across the codebase to ensure consistency, readability, and adherence to the 7-layer architecture. By validating that files and identifiers conform to structural and semantic naming patterns, it prevents naming chaos.

## Functional Requirements

### FR-001: Naming Convention Consistency (AES101)

- **Description**: Every file stem must be snake_case with at least 3 words.
- **Input**: File path
- **Output**: AES101 diagnostic if invalid
- **Business Rules**:
  - Must be snake_case (lowercase ASCII + underscores)
  - Must follow prefix_concept_suffix pattern
  - Minimum 3 words (prefix + concept + suffix)
  - Exceptions: main.rs, lib.rs, mod.rs, __init__.py, index.ts, index.js
- **Edge Cases**: Abbreviations, acronyms
- **Error Handling**: Emit AES101 with invalid filename

### FR-002: Suffix/Prefix Layer Alignment (AES102)

- **Description**: File suffix must align with its layer prefix.
- **Input**: File path
- **Output**: AES102 diagnostic if mismatched
- **Business Rules**:
  - taxonomy_: _vo, _entity, _error, _event, _constant
  - contract_: _protocol, _aggregate
  - utility_: any role suffix (flexible)
  - capabilities_: any role suffix (flexible)
  - agent_: _orchestrator
  - surface_: _command, _controller, _page, _view, _component, _router, _layout, _hook, _store, _action, _screen
  - root_: _container, _entry
- **Edge Cases**: Multiple valid suffixes, custom roles
- **Error Handling**: Emit AES102 with expected suffixes

## Data Model / Entity Relationship

```
NamingRuleVO {
    layer_prefix: String
    allowed_suffixes: Vec<String>
    forbidden_suffixes: Vec<String>
}

LayerSuffixPolicy {
    taxonomy: Vec<String>
    contract: Vec<String>
    utility: Vec<String>
    capabilities: Vec<String>
    agent: Vec<String>
    surface: Vec<String>
    root: Vec<String>
}
```

## API Contract

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| `check_naming_convention()` | File path | Option<Diagnostic> | Check AES101 |
| `check_layer_alignment()` | File path | Option<Diagnostic> | Check AES102 |

## Integration Points

- **Internal**: config-system (YAML rules), shared (taxonomy VOs)
- **External**: None

## Non-functional Requirements (Detailed)

- Performance: Check 1000 files in < 1 second
- Memory: O(1) per file
- Accuracy: Zero false positives for valid names

## Test Scenarios / QA Checklist

- [ ] Valid snake_case name passes
- [ ] Non-snake_case name fails with AES101
- [ ] Name with < 3 words fails with AES101
- [ ] Correct layer suffix passes
- [ ] Wrong layer suffix fails with AES102
- [ ] Exception files (main.rs, lib.rs) pass

## Assumptions & Constraints

- Layer hierarchy is defined in config YAML
- File naming follows AES conventions
- Exceptions are configurable

## Glossary

- **AES**: Agentic Engineering System
- **Layer**: Architectural boundary (taxonomy, contract, utility, capabilities, agent, surface, root)
- **Suffix**: File name ending indicating role (_vo, _protocol, _orchestrator, etc.)

## Reference

- PRD: [PRD.md](../../PRD.md)
