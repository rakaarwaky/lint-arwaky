# Workflow

Step-by-step sequence for creating a new `capabilities_*` file.

1. **Confirm the work is protocol behaviour** (not orchestration, not data, not stateless mechanics). If it is stateless → `aes-utility`. If it is orchestration → `aes-agent`.
2. **Ensure the `_protocol` contract exists** — if missing, run `aes-contract` first.
3. **Resolve the feature folder** for the capability file. Run `lint-arwaky-cli scan <layer-path>`; findings are the work list.
4. **Implement the 3 blocks** from the language HOW-TO § Template / § Section Contract.
5. **Strip forbidden imports / inter-capability deps / local domain models**; move constants and pure helpers to `utility_*`.
6. **Register in the shared barrel** (`__init__.py` / `mod.rs` / `index.ts`), then verify with `lint-arwaky-cli scan`.
7. **Wire via `aes-agent`** and, if applicable, `aes-root`.
