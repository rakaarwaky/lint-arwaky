# Diagnostic tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is the caller the agent (inward) or the surface (outward)?**
   - *Inward / capabilities implement it* → `_protocol`.
   - *Outward / surface calls it* → `_aggregate`.
2. **Does each protocol trait carry every method its capability owns?**
   - *No* → add the missing methods to that trait; a second capability seam gets its own trait in the same file.
3. **Does the aggregate declare exactly one `execute()` method?**
   - *No* → collapse to a single `execute(request: RequestVO) -> ResponseVO`; new consumer verbs are new variants on `RequestVO` + a dispatch arm in the agent, not new aggregate methods.
4. **Do signatures use shared VOs and no impl-layer imports?**
   - *No* → fix per HOW-TO Rules; `lint-arwaky-cli scan` will re-flag AES402/AES201.
5. **Does `lint-arwaky-cli scan <contract-dir>` exit 0?**
   - *No* → fix findings, re-scan, then implement with `aes-capabilities` or `aes-agent`.
