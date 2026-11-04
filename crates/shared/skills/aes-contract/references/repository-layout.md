# Repository layout

```text
shared-domain/                  # modules/shared/src | crates/shared/src | packages/shared/src
├── taxonomy_<concept>_vo.*     # VOs the contract signatures use
├── contract_<concept>_protocol.*   # one file per feature, one trait/class/interface per seam
└── contract_<concept>_aggregate.*   # exactly one execute() method
```

Same shape for Python (`_protocol.py`), Rust (`_protocol.rs`), and TypeScript (`_protocol.ts`).
