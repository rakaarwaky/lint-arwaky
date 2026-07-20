# FRD — naming-rules

## Feature Goal
The naming-rules crate enforces strict naming conventions across the codebase to ensure consistency, readability, and adherence to the 7-layer architecture (Taxonomy → Contract → Utility → Capabilities → Agent → Surface → Root). By validating that files and identifiers conform to structural and semantic naming patterns, it prevents naming chaos and lets developers recognize a component's architectural role from its name alone.

## Requirements & Scope
- In scope:
  - AES101 Naming Convention Consistency — every file stem must be snake_case, follow the prefix_concept_suffix pattern, and contain at least 2 words (prefix + suffix). Exceptions: main.rs, lib.rs, mod.rs, root_*_entry.rs, root_composition_container.rs, __init__.py, index.ts/index.js, barrel/entry files.
  - AES102 Suffix/Prefix Layer Alignment — a file's layer is identified by its prefix_; its suffix must align with that layer's suffix policy:

    | Layer prefix      | Policy   | Allowed suffixes (non-exhaustive)                                                                                 | Forbidden suffixes                                                                                                    |
    | ----------------- | -------- | ----------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
    | taxonomy_         | strict   | _vo, _entity, _error, _event, _constant, _utility, _helper                                                        | —                                                                                                                     |
    | contract_         | strict   | _protocol, _aggregate                                                                                             | —                                                                                                                     |
    | utility_          | flexible | any role suffix describing the technical responsibility (_reader, _writer, _parser, _formatter, …)               | _vo, _entity, _error, _event, _constant, _protocol, _aggregate                                                        |
    | capabilities_     | flexible | _checker, _analyzer, _processor, _validator, _resolver, _calculator, _extractor, _reporter, … (role-based)       | _vo, _entity, _error, _event, _constant, _utility, _helper, _protocol, _aggregate                                     |
    | agent_            | strict   | _orchestrator                                                                                                     | —                                                                                                                     |
    | surface_          | strict   | _command, _controller, _page, _view, _component, _router, _layout, _hook, _store, _action, _screen               | —                                                                                                                     |
    | root_             | strict   | _container, _entry                                                                                                | —                                                                                                                     |
- Out of scope:
  - Dependency-flow / reachability checks (import-rules, orphan-detector).
  - Layer role/behavior checks (role-rules).

## Success Indicators
- [ ] Accuracy — zero false positives: valid snake_case stems and correct layer suffixes are never flagged, invalid ones caught 100% of the time.
- [ ] Coverage — Rust, Python, JavaScript, and TypeScript files all checked per configuration.
- [ ] Layer completeness — every canonical layer prefix is validated, with utility_ covering the former infrastructure_ concerns.
- [ ] Reporting — violations reported with precise location mappings consumable by the central CLI/MCP runner.
