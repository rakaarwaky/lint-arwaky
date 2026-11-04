# Diagnostic Tree

Ask these questions in order. The first "No" dictates the next action.

1. **Is the function stateless, domain-agnostic, and used by ≥2 modules?**
   - *No* → keep as a private helper where it is used, or move to capabilities if it has business rules.
2. **Does it implement protocol behaviour with business rules or injected dependencies?**
   - *Yes* → belongs in capabilities, not utility. See the aes-capabilities skill.
3. **Does the file define a class/struct/impl/trait/type alias?**
   - *Yes* → strip to free functions only (AES404).
4. **Does it import above taxonomy?**
   - *Yes* → remove (AES201).
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.
