# Diagnostic tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is this protocol behaviour (not a stateless helper or pure function)?**
   - *No* → it is a utility, not a capability. See `aes-utility`.
2. **Does a `_protocol` contract exist for this behaviour?**
   - *No* → create it first with `aes-contract`.
3. **Block 2 contain only protocol methods? ≥1 implementor? ≤3 types?**
   - *No* → restructure to 1→2→3; split or extract helpers.
4. **Any forbidden imports or local domain models?**
   - *Yes* → strip; shared needs go through contract or root.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.
