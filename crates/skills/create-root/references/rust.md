# Root — Rust

File: `root_<concept>_<suffix>.rs` where `<suffix>` is `_container` or `_entry`.

## Definition of Done

1. Correct suffix: `_container` or `_entry`.
2. Container: wires Capabilities to Contract traits/aggregates (via `Arc<dyn Trait>`).
3. Entry: bootstraps application and composes feature containers.
4. May instantiate and wire components.
5. No business logic.
6. No orchestration policy.
7. No technical parsing or UI behavior.
8. `cargo check -p <crate-name>` passes.

## Workflow

1. **Determine role** — Container (wire one feature) or Entry (bootstrap all)?
2. **Create file** → `root_<concept>_<suffix>.rs`.
3. **Wire deps** → Connect Capabilities to Contract traits via `Arc::new(impl)`.
4. **Register** → update `mod.rs`.
5. **Verify** → `cargo check -p <crate-name>`.
