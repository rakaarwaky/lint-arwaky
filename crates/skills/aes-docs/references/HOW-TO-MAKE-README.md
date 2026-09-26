# HOW TO MAKE README.md

> **Purpose**: Answer *how do I get this running, and where do I look
> next*.
>
> **Audience**: A developer who just cloned the repo, possibly you in six
> months.
>
> **Scope**: Onboarding: prerequisites, quick start, structure, commands.
> Exactly one per project root.
>
> **Location**: Project root.
>
> **Length**: 50–500 lines (flat budget shared by every document type).

---

## Rules




1. **The test is time, not completeness.** Clone → install → build → run
in under 10 minutes. If the Quick Start cannot clear that, the
missing step is the bug in this file.
2. **Every command was actually run, and says how to confirm it
worked.** A reader who cannot tell success from silence will assume
failure and go ask a human.
3. **Prerequisites are the ones the build fails without**, with versions
pinned. Optional tooling listed as required teaches readers to
distrust the list.

---




## Workflow

1. **Create file** → `README.md` at repo or package root.
2. **Section: Title** — project/tool name.
3. **Section: Quick Start** — 3–5 commands to get running.
4. **Section: Architecture** — diagram or layer description.
5. **Section: Commands** — CLI reference.
6. **Verify** → `aa check docs` passes; quick start works.

## Template

Copy, fill, delete nothing.

```markdown
# <project-name>

> One-liner: what this project does and who it's for.

## Prerequisites

- <see per-language table below>
- <other dependencies>

## Quick Start

<see per-language table below — then make it self-confirming>

## Project Structure

<see per-language block below>

## Architecture

See `ARCHITECHTURE.md`.

## Available Scripts / Commands

<see per-language table below — Rust: "Available Commands"; Python and
TypeScript: "Available Scripts">

## Configuration

<Environment variables, config files — names only, never values>

## Testing

See `TESTING.md` 

## Contributing

See `CONTRIBUTING.md` 

## License

<License type>

```

---

## Section Contract

Every section is required unless marked optional. Each exists for one
reason.

| Section                    | Why it belongs here                                                                    |
| -------------------------- | -------------------------------------------------------------------------------------- |
| One-liner                  | Decides in five seconds whether to keep reading. Watch for restating the project name. |
| Prerequisites              | Prevents a build failure with no cause. Watch for optional tooling listed as required. |
| Quick Start                | The 10-minute promise. Watch for steps that assume state the reader lacks.             |
| Architecture               | Delegate to \`ARCHITECHTURE.md\` file                                                  |
| Project Structure          | Teaches where specs and backlogs live. Watch for every generated file listed.          |
| Available Scripts/Commands | The daily loop, as CI runs it. Watch for flags that differ from the CI job.            |
| Configuration              | What to set before the first run. Watch for values, secrets, or real hostnames.        |
| Testing                    | Delegate to \`TESTING.md\` file                                                        |
| Contributing               | Delegate to \`CONTRIBUTING.md\` file                                                   |
| License                    | Delegate to \`LICENSE\` file                                                           |

---

## Verify

```bash
aa check docs .
# Checks: readme-section-missing, dead-link, absolute-path, secret-in-docs, doc-length.
```
