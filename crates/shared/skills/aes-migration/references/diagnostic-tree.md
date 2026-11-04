# Diagnostic tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is this a legacy codebase without AES layers?**
   - *No* → verify compliance with `lint-arwaky-cli scan`; fix findings per layer skill.
   - *Yes* → proceed to Phase 0.
2. **Has the baseline audit been run?**
   - *No* → `lint-arwaky-cli scan .`; record violation count to choose strategy (< 10: full, 10–50: phased, > 50: taxonomy-first).
   - *Yes* → proceed to Phase 1.
3. **Does the taxonomy layer expose only domain values (no I/O, no upward imports)?**
   - *No* → strip I/O to capabilities; fix primitives to VOs (AES401); re-scan.
   - *Yes* → proceed to Phase 2.
4. **Is every protocol file one trait per capability seam, and every aggregate one `execute()` method?**
   - *No* → split protocols per seam (each trait owns its capability's methods); collapse aggregate to a single `execute(request) -> response` (AES402/AES403).
   - *Yes* → proceed to Phase 3.
5. **Does `lint-arwaky-cli scan <project-root>` exit 0?**
   - *No* → fix findings in order: taxonomy → contract → utility → capabilities → agent → surface → root.
   - *Yes* → migration complete; run tests and format.
