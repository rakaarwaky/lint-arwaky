# aes-docs — Verification

## Machine Checks

```bash
lint-arwaky-cli docs .                 # invariant audit of every document (strict; every finding gates)
```

A pass means no claim sits in the wrong file, no pointer is broken, and no `Done` row is unevidenced.

## Human Checks

A machine pass does not mean the documents are good. Falsifiable goals, honest exclusions, and a 10-minute Quick Start still need a reader.

## Pre-flight Checklist

- [ ] `lint-arwaky-cli docs <path>` exits 0.
- [ ] Every required document exists in the correct directory.
- [ ] Every `Done` backlog row cites a re-run command, a commit hash, and its exclusions.
- [ ] Documents serve their exact audience (no cross-contamination).
- [ ] Public code items carry doc comments, and surface checks (`cargo doc`, `tsc`, `import`) are clean.
- [ ] Every touched reference file had its specific `Verify` block executed.

## Doc Comment Conventions

Per-language rules, templates, section contracts, and Verify blocks:

- Python (PEP 257): `references/HOW-TO-MAKE-PYTHON-DOC.md`
- Rust (`///` / rustdoc): `references/HOW-TO-MAKE-RUST-DOC.md`
- TypeScript (JSDoc / TSDoc): `references/HOW-TO-MAKE-TYPESCRIPT-DOC.md`
