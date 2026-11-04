# Diagnostic tree

Ask these questions in order. The first "No" dictates your next action.

1. **What tier is this — Smart / Utility / Passive?**
   - *Smart* → inject aggregate; *Utility/Passive* → taxonomy-only imports.
2. **Is the suffix in the surface allow-list (never `_entry`)?**
   - *No* → rename to a legal surface suffix (or `root_*_entry` if it bootstraps).
3. **Any business logic, computation, or direct capability calls?**
   - *Yes* → move logic down; surface only maps and delegates.
4. **Any silently discarded errors or primitive state fields?**
   - *Yes* → propagate errors; wrap state in VOs.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.
