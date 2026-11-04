# Diagnostic tree

Ask these questions in order. The first "No" dictates your next action.

1. **Container or Entry?**
   - *Wire one feature* → `_container`; *bootstrap all* → `_entry`.
2. **Does the file contain business / orchestration / parsing / UI logic?**
   - *Yes* → move down to capabilities / agent / utility / surface.
3. **Are bindings exposed as contracts (not concretions)?**
   - *Concretions leak* → bind to protocol/aggregate types (`Arc<dyn Trait>` in Rust).
4. **Does anything below root import from root?**
   - *Yes* → invert: root depends downward only.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.
