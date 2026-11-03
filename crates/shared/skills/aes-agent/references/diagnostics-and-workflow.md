# Diagnostics and Workflow

Triage an agent file in the fixed order, walk the workflow, then verify by machine and reader.

## Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is this orchestration only — or does it compute / touch I/O / hold domain data?**
   - *Computes/I-O/data* → move to capabilities / taxonomy / utility.
2. **Does an `_aggregate` contract exist?**
   - *No* → create it first with `aes-contract`.
3. **Block 2 only aggregate methods? ≥1 aggregate? ≤3 types?**
   - *No* → restructure to 1→2→3; extract free functions to utility.
4. **Any silently discarded errors or raw primitives in signatures?**
   - *Yes* → propagate/`Result`; wrap primitives in VOs.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.

## Workflow

1. Confirm orchestration only — computation → capabilities, domain data → taxonomy, pure helpers → utility.
2. Ensure an `_aggregate` contract exists — if missing, run `aes-contract` first.
3. Resolve the feature agent dir. Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
4. Implement the 3 blocks from the language HOW-TO § Template / § Section Contract; inject the aggregate via DI.
5. Check imports, I/O, computation, error handling, and primitives against HOW-TO § Rules.
6. Register in the shared barrel, verify with `lint-arwaky-cli scan`, then compose via `aes-root` / consume via `aes-surface`.

## Verification

### Machine Checks

```bash
lint-arwaky-cli scan <layer-path>   # AES101/102, AES201–205, AES401–406 → must be 0
# Fallback only: language compile (python -c import / cargo check / npx tsc --noEmit)
```

A pass means naming, imports, primitives, and roles are clean. Structural judgement
(tier choice, block order, helper-vs-utility, "orchestration only") is **manual** — see HOW-TO § Rules.

### Human Checks

A machine pass does not mean the file is right. Layer purpose, structural order, and
"only what this layer may do" still need a reader (HOW-TO § Rules).

## Pre-flight Checklist

- [ ] `lint-arwaky-cli scan <layer-path>` exits 0.
- [ ] Every touched HOW-TO's `Verify` block was executed.
- [ ] File registered in `__init__.py` / `mod.rs` / `index.ts`.
- [ ] Language fallback compile clean if the HOW-TO lists it.
- [ ] Related skills considered for the next layer up/down.

## Common Mistakes (Anti-Patterns)

The linter covers naming, imports, and primitives. These need a reader (HOW-TO § Rules):

- **Computation or I/O in the agent**: arithmetic/parsing/FS/network belong below the agent layer.
- **Silently discarded errors**: no `or ""` / `unwrap_or_default()` / `?? ""` on results.
- **Agent implements protocol instead of aggregate (or both)**: exactly one aggregate, 3-block order.
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

---

## Completion

Work is done when:

- All 6 Workflow steps were executed in order.
- Pre-flight Checklist is fully ticked.
- `lint-arwaky-cli scan <layer-path>` exits 0 with every finding from the work list fixed and re-scanned.
