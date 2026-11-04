# Verification

The machine gate is the linter. The language run command proves the suite is green.

## Machine gate

```bash
lint-arwaky-cli scan .   # AES103 (prefix/directory) + AES704 (category coverage) must be 0
```

Both halves fire on a non-conforming layout: a wrong prefix or a nested subdirectory trips AES103; a missing test type trips AES704.

## Green suite

Run the language command from the language HOW-TO (`cargo nextest run`, `pytest`, `vitest run`). A pass means every owed type ran and coverage reached the layer floor.

## Fallback gate only

- Language compile (`python -c "import ..."`, `cargo check`, `npx tsc --noEmit`) — fallback only when the language runner is unavailable.
