# Step 0: Read the step matching your violation

Before applying any fix, establish a baseline so the turn-progress gate can measure movement.

1. Run `lint-arwaky-cli report .` once now and record the violation counts for each rule code.
2. Identify which rule code your scan reported a violation for (e.g. `AES101`).
3. Read the step file matching that rule code from the Routing table in `SKILL.md` — e.g. `AES101` maps to [`step-1-AES101.md`](step-1-AES101.md), `AES602` maps to [`step-27-AES602.md`](step-27-AES602.md).
4. Apply only that step's fix steps. Do not walk through every step in sequence.

The end-of-turn report will show a Delta column against your start-of-turn baseline. A positive delta means the fix run added problems; the turn is not done.

## Commands

| Command | Use |
| ------- | --- |
| `lint-arwaky-cli report .` | Record baseline violation counts for the target path. |

## Completion

- `lint-arwaky-cli report .` has been run once and its output recorded.
- The relevant step file for your rule code has been read.
- No fix steps have been applied yet.
