# Workflow

1. Confirm stateless, domain-agnostic, ≥2 consumers (else keep a private helper).
2. Resolve the shared utility dir beside taxonomy. Run `lint-arwaky-cli scan <layer-path>`.
3. Draft free functions from the language HOW-TO § Template / § Section Contract.
4. Move needed literals to `taxonomy_*_constant`; keep business rules in capabilities.
5. Register in `__init__.py` / `mod.rs` / `index.ts`, then verify with `lint-arwaky-cli scan <layer-path>`.
