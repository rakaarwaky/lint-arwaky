# Verification Checklist

- [ ] Contract file uses correct suffix: `_port`, `_protocol`, or `_aggregate`.
- [ ] Contract contains only interface definitions.
- [ ] Contract contains no method implementations.
- [ ] Contract contains no private helper signatures.
- [ ] Interface is exported with `export interface`.
- [ ] Methods have proper TypeScript type annotations.
- [ ] Contract imports only taxonomy and contract types.
- [ ] Contract does not import from capabilities, agents, or surface.
- [ ] Contract signatures use shared VOs for domain data.
- [ ] Error types come from shared taxonomy.
- [ ] New contract module is registered in `index.ts`.
- [ ] `npx tsc --noEmit` passes.
