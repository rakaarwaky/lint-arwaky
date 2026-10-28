# FRD — tui

## Feature Overview

The tui crate is the interactive terminal interface: a file-tree browser with
preview and status panes, a confirm gate for mutating actions, keyboard and
mouse bindings, layout breakpoints, and a themed render path that also works in
no-color mode. It renders lint results through the kernel report formatter, so
the terminal presentation matches the CLI output for the same scan.

## Functional Requirements

The surface walks the workspace file tree and highlights the selected path,
rendering lint results and a preview for it. Mutating actions pass a confirm
gate before they run, and a busy slot serializes action threading so a second
trigger is queued or rejected instead of interleaving with the running action.
Outcome states and progress feedback appear in the status component while an
action runs. Keybindings, a scrollable help overlay, and pointer input cover
the same actions through different input paths, and the layout reflows at its
breakpoints so narrow terminals keep the essential panes. Theme tokens carry
contrast pairs and are dropped cleanly when color support is absent.

## Public Interface

| Aspect | Detail|
|--------|-------|
| Surface | file tree, preview, status, confirm gate, help overlay, theme |
| Entry | root container wiring the surface components to the lint action path |
| Threading | one busy slot serializing confirm-gated actions |
| Formatting | kernel report formatter shared with the CLI output |
| Contract | presentation and input only; workspace logic routes through the dispatcher |

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| FRD.md | Feature requirements | [FRD.md](FRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
