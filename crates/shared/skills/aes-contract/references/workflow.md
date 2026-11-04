# Workflow

1. **Resolve the contract dir** in the shared domain beside taxonomy.
2. **Analyze**: which suffix? who implements, who consumes? Run `lint-arwaky-cli scan <contract-dir>` — findings are your work list.
3. **Draft protocol**: one file per feature, one trait/class/interface per capability seam, each rich with named methods. See the language HOW-TO (e.g. [python-contract.md](python-contract.md)).
4. **Draft aggregate**: exactly one `execute()` method returning a taxonomy-defined response enum. Same HOW-TO § Template / § Section Contract.
5. **Register** in `__init__.py` / `mod.rs` / `index.ts`.
6. **Verify**: `lint-arwaky-cli scan <contract-dir>` (HOW-TO § Verify), then implement with `aes-capabilities` / `aes-agent`.
