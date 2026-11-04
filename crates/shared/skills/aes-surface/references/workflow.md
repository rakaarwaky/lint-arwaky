# Workflow

1. Determine the tier (Smart / Utility / Passive) and pick the matching suffix.
2. Resolve the feature surface dir. Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
3. Draft from the language HOW-TO § Template / § Section Contract; Smart injects the aggregate via DI.
4. Enforce that tier's import rules; strip business logic and computation.
5. Check error handling: nothing discarded, no empty fallbacks; state fields are VOs.
6. Register the module where required, verify with `lint-arwaky-cli scan`, then wire via `aes-root`.
