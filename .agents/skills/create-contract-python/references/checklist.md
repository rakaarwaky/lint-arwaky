# Verification Checklist

- [ ] Contract file uses correct suffix: `_port`, `_protocol`, or `_aggregate`.
- [ ] Contract contains only ABC definitions.
- [ ] Contract contains no method implementations.
- [ ] Contract contains no default method bodies.
- [ ] Contract contains no private helper signatures.
- [ ] ABC inherits from `ABC`.
- [ ] All methods use `@abstractmethod` decorator.
- [ ] Contract imports only taxonomy and contract types.
- [ ] Contract does not import from capabilities, agents, or surface.
- [ ] Contract signatures use shared VOs for domain data.
- [ ] Error types come from shared taxonomy.
- [ ] New contract module is registered in `__init__.py`.
- [ ] `python -c "import <module>"` passes.
