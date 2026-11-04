# Templates

Copy, fill, delete nothing. Each language HOW-TO carries its own template block:

- Python PEP 257 docstring template: see [python-doc.md](python-doc.md) § Template
- Rust `///` rustdoc template: see [rust-doc.md](rust-doc.md) § Template
- TypeScript JSDoc/TSDoc template: see [typescript-doc.md](typescript-doc.md) § Template

The template is the single source of truth for the doc-comment shape in that language. A new
module's doc comment copies the template, fills every `<placeholder>`, and deletes nothing.
