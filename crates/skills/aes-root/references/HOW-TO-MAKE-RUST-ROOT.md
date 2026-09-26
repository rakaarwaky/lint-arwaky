# HOW TO MAKE ROOT RUST

> **Purpose**: Compose the system: containers wire capabilities to contracts; entries bootstrap and start the process.
>
> **Audience**: Agents and engineers scaffolding AES composition roots.
>
> **Scope**: Python, Rust, and TypeScript `root_<concept>_<container|entry>` files (plus documented barrel/entry exceptions).
>
> **Location**: Feature or app package top; the only layer that may construct implementations.
>
> **Length**: One container per feature; one (or few) entry files; no business logic, no orchestration policy.

---

## Rules

1. **Suffix is strictly `_container` or `_entry`.** File: `root_<concept>_<suffix>.rs`.
   Documented exceptions: `main.rs`, `lib.rs`, `mod.rs`, `root_cli_main_entry.rs`,
   `root_mcp_main_entry.rs`, `root_tui_main_entry.rs`, `root_composition_container.rs`
   (AES101/AES102).
2. **Container = wire one feature only.** `Arc::new(impl)` for capabilities; expose
   `Arc<dyn Trait>` (aggregate/protocol contracts). Never business logic.
3. **Entry = bootstrap the app.** Parse CLI args (if needed), compose feature container
   factories, start the surface/CLI loop. Never construct capabilities directly.
4. **Allowed imports: everything below root** — agent, capabilities, surface, contract,
   taxonomy, utility. Nothing below root may import root (AES201–AES205).
5. **No business logic, no orchestration policy, no technical parsing, no UI behaviour** —
   those live in capabilities / agent / utility / surface.
6. **Signatures use shared VOs** where domain values appear (AES402). `bool` for
   semantic toggles only.
7. **Register** in crate `mod.rs`.

### Workflow

1. **Determine role** — Container (wire one feature) or Entry (bootstrap all)?
2. **Create file** → `root_<concept>_<suffix>.rs`.
3. **Wire deps** → `Arc::new(impl)`; expose `Arc<dyn Trait>`.
4. **Register** → update `mod.rs`.
5. **Verify** → `lint-arwaky-cli scan <layer-path>` then `cargo check -p <crate-name>`.

---

## Template

Copy, fill, delete nothing. File: `root_<concept>_<suffix>.rs` where
`<suffix>` is `_container` or `_entry`.

### Container — wire one feature

```rust
// PURPOSE: <Concept>Container — wiring for <feature> feature (root layer, wiring only)
use crate::agent_<concept>_orchestrator::<Concept>Orchestrator;
use crate::capabilities_<concept>_<role>::<Capability>;
use shared::<concept>::contract_<concept>_aggregate::I<Concept>Aggregate;
use shared::<concept>::contract_<concept>_protocol::I<Concept>Protocol;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct <Concept>Container {
    orchestrator: Arc<dyn I<Concept>Aggregate>,
}

// ─── Block 2: Wiring & Factory ────────────────────────────

impl <Concept>Container {
    pub fn new(/* shared deps */) -> Self {
        let runners: Vec<Arc<dyn I<Concept>Protocol>> = vec![
            Arc::new(<Capability>::new(/* deps */)),
        ];
        let orchestrator = Arc::new(<Concept>Orchestrator::new(runners));
        Self { orchestrator }
    }

    pub fn aggregate(&self) -> Arc<dyn I<Concept>Aggregate> {
        self.orchestrator.clone()
    }
}
```

**Container rules:** instantiate capabilities with `Arc::new(impl)`; expose
`Arc<dyn Trait>` (aggregate/protocol contracts) so agents and surfaces never
see concretions; construction and binding only — no business logic.

### Entry — bootstrap the application

```rust
// PURPOSE: <concept> CLI entry — compose feature containers and start the surface
use <feature>::root_<concept>_container::<Concept>Container;
use <feature>::surface_<concept>_command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let container = <Concept>Container::new(/* shared deps */);
    let code = surface_main::run(container.aggregate(), &args);
    std::process::exit(code);
}
```

**Entry rules:** parse process args (bootstrap only) and compose containers;
start the surface/CLI loop; never construct capabilities directly — go through
a container; no business logic.

---

## Section Contract

| Section | Why it belongs here |
| ------- | ------------------- |
| Module docstring / PURPOSE | Names the role: composition container or app entry. |
| Correct filename + suffix | AES101/AES102 resolve the layer and role from the name. |
| Layer-legal imports | Keeps the dependency arrow pointed down (AES201–AES205). |
| `Arc<dyn Trait>` in signatures | Expose contracts, not concretions (AES402). |
| Block 1 struct / Block 2 wiring | Matches the standard 3-block root layout. |
| Container: `new` + accessors only | Wiring is the whole job; logic belongs below. |
| Entry: compose containers + `main` | Bootstrap only; never construct capabilities. |
| Registered in `mod.rs` | Dead code otherwise; composition needs the export. |

---

## Anti-Patterns

- **Business / orchestration / parsing / UI logic in root** — move down a layer.
- **Entry constructing capabilities directly** — go through a container `new`.
- **Container returning concrete types instead of `Arc<dyn Trait>`** — agents/surfaces must see contracts.
- **Wrong suffix or undocumented name** — only `_container` / `_entry` (or listed exceptions).

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Manual (not machine-checked): correct role suffix; no business/orchestration/parsing/UI logic; nothing below root imports root.
# Fallback compile gate: cargo check -p <crate-name>
```
