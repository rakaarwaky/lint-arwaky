---
name: create-capabilities
description: AES capability implementation scaffolding for Python Rust TS. Use when creating capability files.
metadata:
  tags: [python, rust, typescript, aes, capabilities, protocol, role-naming, 3-block-structure, di, vo]
  triggers:
    - "create capabilities"
    - "create capabilities python"
    - "create capabilities rust"
    - "create capabilities typescript"
    - "add capabilities"
    - "add capabilities python"
    - "add capabilities rust"
    - "fix capabilities structure"
    - "create protocol"
    - "create protocol python"
    - "create protocol rust"
    - "create protocol typescript"
    - "capabilities missing protocol"
    - "validate capabilities logic"
    - "check capabilities"
    - "check capabilities python"
    - "audit capabilities"
    - "audit capabilities python"
    - "audit capabilities rust"
  dependencies: []
  related:
    - create-agent
    - create-taxonomy
    - create-contract
    - create-utility
---
# Create Capabilities (AES)

The **capabilities layer is the concrete implementation of behaviour**: domain rules
(validating, calculating, classifying…) and external adaptation (repositories, gateways,
clients). It implements contract **protocols** and is consumed by agents or the root container —
never by other capabilities.

Naming: `capabilities_<domain>_<role>.<ext>`. Suffixes are flexible but **forbidden** names apply
(AES102): no `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_constants`, `_protocol`,
`_aggregate`, `_utility` — those roles belong to other layers.

Read `references/python.md`, `references/rust.md`, or `references/typescript.md` for that
language's templates, structure-rule wording, and verify command.

## Role naming

- **Internal (domain logic):** validator, assessor, calculator, resolver, classifier, selector,
  mapper, transformer, policy, enricher, evaluator, analyzer, scorer, grader, ranker, filter,
  checker, reviewer, approver, rejector
- **External (adaptation):** repository, gateway, client, provider, fetcher, reader, writer,
  scanner, executor, publisher, subscriber, adapter, connector, uploader, downloader, sender,
  receiver, dispatcher, watcher, monitor

## Import rules (all languages)

**Allowed:** Taxonomy, Contract (`_protocol` only), Utility.
**Forbidden:** `agent_*`, other `capabilities_*`, `surface_*`, local domain models, magic constants.
Capabilities must not depend on each other; shared needs go through a contract or the root container.

## Structure rules (AES403)

1. Internal helper types with no contract implementation → allowed.
2. **≥1 type must implement a protocol contract** — otherwise flag `CapabilityNoProtocol`.
3. **At most 3 types per file** (Python classes; Rust struct+enum; TS class+interface+enum,
   excluding `type` aliases). A capability never implements a protocol *and* an aggregate.

## 3-Block structure (mandatory order)

1. **Block 1** — type definition & constructor / struct fields.
2. **Block 2** — protocol method implementations only.
3. **Block 3** — constructors/factories, standard-protocol methods (`__repr__`, `Display`,
   `toString`), private helpers.

## Helper vs Utility

Keep a function in Block 3 if **any** of: it uses instance state, is domain-specific, has a single
consumer, or acts as a constructor/factory.
Extract it to the Utility layer only if **all** of: no instance state, pure/deterministic (or
domain-agnostic I/O such as serialization), domain-agnostic, ≥2 consumers.

## Language split

| Language   | Contract form          | DI mechanism                | Verify                       |
| ------------ | ------------------------ | ----------------------------- | -------------------------------- |
| Python     | `I<Name>Protocol(ABC)` | Constructor param (ABC)     | `python -c "import <module>"`  |
| Rust       | `trait I<Name>Protocol` | `Arc<dyn I<Name>Protocol>`  | `cargo check -p <crate-name>`  |
| TypeScript | `interface I<Name>Protocol` | Constructor param (interface) | `npx tsc --noEmit`           |

Fields and signatures use shared VOs; constants live in `taxonomy_<domain>_constant`; low-level
reusable stateless operations move to Utility. Rust helpers must be private or `pub(crate)`.

## Workflow

1. Confirm the work is protocol behaviour (not orchestration, data, or mechanics).
2. Check the file references a `_protocol` contract — if missing, flag `CapabilityNoProtocol`.
3. Create the missing contract with `create-contract`.
4. Implement the 3 blocks from the language reference template.
5. Enforce AES403: ≥1 protocol implementor, ≤3 types, DI via protocols, shared VOs.
6. Remove forbidden imports, inter-capability dependencies, and local domain models.
7. Register in `__init__.py` / `mod.rs` / `index.ts` and run the verify command.

## Checklist

- [ ] File is `capabilities_<domain>_<role>` and the role comes from the naming lists.
- [ ] Block 1 → 2 → 3 order followed (Rust: with explicit `// Block N:` comments).
- [ ] Block 2 contains ONLY protocol contract method implementations.
- [ ] ≥1 type implements a protocol contract; ≤3 total types in the file.
- [ ] Imports only from taxonomy, `_protocol` contracts, and utility.
- [ ] No local domain models, no agent or sibling-capability imports.
- [ ] DI through protocol interfaces (`Arc<dyn Trait>` in Rust); shared VOs in fields/signatures.
- [ ] Constants moved to `taxonomy_<domain>_constant`; low-level ops moved to Utility.
- [ ] Rust only: helpers private or `pub(crate)`, not fully `pub` without justification.
- [ ] Registered in the shared barrel; verify command passes.
