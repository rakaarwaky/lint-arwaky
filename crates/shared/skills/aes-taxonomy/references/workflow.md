# Workflow

1. Resolve the shared taxonomy dir beside contracts.
2. Determine type (VO / Entity / Error / Event / Constant / Request / Response). Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
3. Draft the file from the language HOW-TO § Template / § Section Contract.
4. Register in `__init__.py` / `mod.rs` / `index.ts`.
5. Verify with `lint-arwaky-cli scan <layer-path>` (HOW-TO § Verify), then wire through `aes-contract` / `aes-capabilities` as needed.
