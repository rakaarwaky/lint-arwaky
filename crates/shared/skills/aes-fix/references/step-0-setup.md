# Step 0: Setup, orchestration, and ship

This step owns the full run: baseline, per-member subagent fan-out, delta verification, and the PR. Read only the rule step file matching each violation; do not walk every rule step in sequence.

## 1. Baseline

1. Run `lint-arwaky-cli report .` once now and record the violation counts for each rule code.
2. Identify which rule code your scan reported a violation for (e.g. `AES101`).
3. Read the step file matching that rule code from the Routing table in `SKILL.md`

## 2. Group violations by member

From the baseline `lint-arwaky-cli report .` output:

1. List every violating file.
2. Group the files by owning member. A member is the top-level directory or crate that owns
3. Keep only members that own at least one violation; members with a clean report are skipped.

## 3. Spawn one subagent per member

For each member in section 2, spawn a subagent with exactly this brief:

- The list of that member's violations only (rule code, file path, line, message).
- The rule reference link for each violated rule code
- The instruction: read the referenced step files, apply their fix steps to the listed files, run the verify command named at the end of each step file, and stop.
- The boundary: the subagent does not touch files owned by another member.

## 4. Verify the delta

Run `lint-arwaky-cli report .` again. Compare against the baseline:

- A zero or negative delta for every rule the subagents touched is the pass condition.
- A positive delta means the fix run added problems. Identify the rule and file, re-fix inline, and re-run the report. Repeat until the delta is zero or negative everywhere.

## 5. Ship

1. Open a PR with all the fixes from the subagents.
2. Merge the PR to main.

Do not ship with a positive delta outstanding.

## Commands

## Completion

- `lint-arwaky-cli report .` has been run at the start and its output recorded.
- Every violating member has a finished subagent; no member was skipped with violations outstanding.
- The end-of-turn report shows zero or negative delta for every touched rule.
- The PR is open and merged to main.
