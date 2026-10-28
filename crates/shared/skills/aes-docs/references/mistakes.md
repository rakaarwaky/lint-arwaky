# aes-docs — Common Mistakes (Anti-Patterns)

The invariant codes above cover the machine-checkable ones. These need a reader.

### Structural

- **One document for all audiences**: Split by audience. Each file answers one question.
- **FRD at the project root**: It belongs with the feature code, beside its backlog.
- **Feature backlog carrying workspace rows or restating root policy**: Cross-cutting rows and the `Roadmap Status` roll-up live once in `ROADMAP.md` (root master).
- **PRD carrying SQL schemas or API detail**: The PRD audience cannot read them. Move to FRD.

### Cadence and code surface

- **Documents "write & forget"**: Re-run `lint-arwaky-cli docs` each sprint. Drift is silent.
- **`//` instead of `///` in Rust**: Plain comments are invisible to the doc generator.
- **Missing module docstrings or undocumented parameters**: The generated API surface stays incomplete.
