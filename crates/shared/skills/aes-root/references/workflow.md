# Workflow

1. Determine role — Container (wire one feature) or Entry (bootstrap all).
2. Resolve the package top. Run `lint-arwaky-cli scan <layer-path>` — findings are your work list.
3. Create `root_<concept>_<suffix>` from the language HOW-TO § Template / § Section Contract.
4. Wire dependencies: instantiate capabilities, bind them to contract protocol/aggregate types.
5. Compose: the entry builds every container and starts the surface or CLI loop.
6. Register the module, verify with `lint-arwaky-cli scan`.
