# Anti-patterns

The linter covers naming, imports, and primitives. These need a reader (HOW-TO § Rules):

- **Protocol with several methods**: one file = one feature; a second capability seam gets its own trait in the same file — not a separate file.
- **Aggregate with several methods**: exactly one `execute()` is the only legal entry point; a new consumer verb means a new variant on `RequestVO`, not a new aggregate method.
- **Contract importing capabilities/agents/surface/root**: dependency arrow inverted (AES201/AES205).
- **Primitives in signatures**: replace with taxonomy VOs (AES402).
- **Restating HOW-TO rules in SKILL.md**: delegate — this file only routes.

## Concrete classes in contract files (AES402 contract method default body)

**The invariant**: a `contract_*_protocol.py` file may contain only ABC declarations. Every class in it must be `class I<Name>…(ABC)` with `@abstractmethod` and `...` / `pass` bodies. A concrete class — one with real method bodies, even if it inherits an ABC — is a layering violation, not a linter false positive. AES402 flags every non-stub method in it.

**Why it matters**: a shared concrete base class placed in a contract file looks like a harmless convenience but inverts the intended dependency direction and blocks every consumer from seeing the protocol shape. The architecture has exactly one place for shared implementation logic: `utility_*` (stateless functions, no classes per AES404) or `taxonomy_*_constant` (literals only).

**What to do instead**:

- Shared *logic* → `utility_*` as stateless functions; capabilities call them directly.
- A *base class* with real bodies → this pattern is not sanctioned; remove it and use composition (capability holds the utility and delegates) instead of inheritance.
- If you see AES402 on a class that is not `I…(ABC)`, the fix is to move the class out of the contract file and refactor the consumers — not to add an exception or stub the body.
