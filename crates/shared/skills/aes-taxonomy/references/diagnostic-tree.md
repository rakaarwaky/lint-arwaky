# Diagnostic tree

Ask these questions in order. The first "No" dictates your next action.

1. **Is this a domain value, identity, failure, fact, literal, or an aggregate boundary?**
   - *Value* → `_vo`; *identity* → `_entity`; *failure* → `_error`; *fact* → `_event`; *literal* → `_constant`; *inbound aggregate call* → `_request`; *outbound aggregate call* → `_response`.
2. **Does the file import anything above taxonomy or touch I/O?**
   - *Yes* → strip the import / move I/O to capabilities or utility.
3. **Are domain fields raw primitives?**
   - *Yes* → wrap each in a VO (AES401).
4. **Is the file registered in the shared barrel?**
   - *No* → register; else the module is dead code.
5. **Does `lint-arwaky-cli scan <layer-path>` exit 0?**
   - *No* → fix findings, re-scan.
