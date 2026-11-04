# Placement

A `utility_*` file belongs in the workspace's **`shared/` folder**, beside `taxonomy_*` and `contract_*` files:

```text
crates/shared/src/     # Rust
modules/shared/src/     # Python
packages/shared/src/    # TypeScript
```

The `shared` folder is locked to taxonomy, utility, and contract files. A `capabilities_*`, `agent_*`, or `surface_*` file found there is an **AES701** violation and must be moved out.

A utility that only one feature uses does not belong in `shared/` — keep it a private helper inside that feature folder.
