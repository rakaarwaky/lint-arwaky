# Verification Checklist

- [ ] Contract file uses correct suffix: `_port`, `_protocol`, or `_aggregate`.
- [ ] Contract contains only trait definitions.
- [ ] Contract contains no `impl` blocks.
- [ ] Contract contains no default method bodies.
- [ ] Contract contains no private helper signatures.
- [ ] Trait includes `Send + Sync` bounds.
- [ ] Trait is object-safe when intended for `Arc<dyn Trait>`.
- [ ] Async trait methods are dyn-compatible.
- [ ] Contract imports only taxonomy and contract types.
- [ ] Contract does not import from capabilities, agents, or surface.
- [ ] Contract signatures use shared VOs for domain data.
- [ ] Error types come from shared taxonomy.
- [ ] New contract module is registered in `mod.rs`.
- [ ] `cargo check -p shared` passes.
