# HOW TO MAKE TYPESCRIPT DOC COMMENTS

> **Purpose**: Document every public TypeScript item so JSDoc/TSDoc
> readers know *what* and *why* without opening the body.
>
> **Audience**: Engineers and agents reading IDE hover and generated
> API docs.
>
> **Scope**: Module headers, exported classes/functions, and public
> methods under the package's public surface.
>
> **Location**: On the item itself (`/** … */` immediately above the
> `export` / method).
>
> **Length**: One-liner summary; `@param` / `@returns` only when the
> types are not self-evident.

---

## Rules

Six rules. Each one prevents a specific failure mode.

1. **One-liner at the top of every module.** `/** … */` before the
 first export; no changelog, no status.
2. **Doc comments explain *what* and *why*, never *how*.** Restating
 the implementation trains readers to skip docs.
3. **`@param` and `@returns` on every public method** whose meaning
 is not already obvious from the parameter name and type.
4. **Named `interface` or `type` aliases for complex shapes** instead
 of inline object literals — the alias gets the doc, not each
 accidental field.
5. **Public exports always carry a doc comment.** An `export` with no
 `/**` is an undocumented public API.
6. **No status claims.** Never implemented / shipped / N% done —
 that lives in `BACKLOG.md` (`status-in-spec`).

---

## Workflow

1. **Create file** — module docstring at top of source file.
2. **Write Purpose** — one-line module job.
3. **Write Audience** — who reads this.
4. **Write Scope** — what the module covers.
5. **Verify** → `aa check docs` passes; doc present.

## Template

Copy, fill, delete nothing.

```ts
/** Value objects for import rules. */

/** An import rule: a path pattern and the message it reports. */
export class ImportRuleVO {
  /**
   * @param pattern - Glob matched against a repo-relative path.
   * @param message - Violation text reported to the user.
   */
  constructor(private readonly pattern: string, private readonly message: string) {}

  /**
   * Report whether a path violates this rule.
   *
   * @param path - Repo-relative file path to test.
   * @returns True when the path matches the pattern.
   */
  check(path: string): boolean {
    return minimatch(path, this.pattern);
  }
}
```

---

## Section Contract

Every public item is required to carry the rows that apply. Each
exists for one reason.

| Section                     | Why it belongs here                                                           |
| --------------------------- | ----------------------------------------------------------------------------- |
| Module one-liner (required) | Names the module's job before any export.                                     |
| Export summary (required)   | One sentence: what the export is or does. Watch for restating the identifier. |
| `@param` (rec)              | Explains non-obvious parameters. Skip when the name and type fully say it.    |
| `@returns` (rec)            | States the contract the caller depends on. Skip for `void`.                   |
| Named type alias (rec)      | Gives complex object shapes one documented name. Skip for one-field bags.     |

---

## Verify

```bash
npx tsc --noEmit
# Checks: signatures typecheck; every touched public export has /** */.
# Manual: no "how" narration; @param/@returns present; no status words.
```
