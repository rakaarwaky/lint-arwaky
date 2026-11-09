# TUI — DESIGN

> Visual and interaction contract for the terminal surface. Tokens, panel
> geometry, and component anatomy.
> Condition: [ROADMAP.md](../../ROADMAP.md) (workspace) +
> [BACKLOG.md](BACKLOG.md).

## Design Overview

`utility_tui_theme.rs` is the single source of truth for every token below. A
renderer resolves a token through the `color()` helper rather than applying a
palette constant, so `NO_COLOR` collapses the whole surface to the terminal's
monochrome default in one place.

## Design Requirements

### DS-TUIC-001: Contrast Ownership

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
| Default selection | Selected row band on a terminal theme the app cannot sample | `HIGHLIGHT` | `Color::DarkGray` |
| High contrast selection | Same band, opt-in via `LINT_ARWAKY_TUI_HIGH_CONTRAST=1` | reversed video | `Modifier::REVERSED` |

The terminal emulator, not this application, decides the RGB behind every ANSI
name, so the app cannot guarantee a contrast ratio. It therefore owns two
things it can guarantee: the band's weight (`BOLD` in both modes) and an
opt-in mode that inverts the colours the terminal is already rendering legibly.
`NO_COLOR` wins over high contrast.

### DS-TUIC-002: Colors

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
| Accent | Focused panel border, selected file name, phase label | `ACCENT` | `Color::Cyan` |
| Key | Shortcut key glyphs, tree expand/collapse markers | `KEY` | `Color::Yellow` |
| Label | File names, status text, body copy | `LABEL` | `Color::White` |
| Separator | Unfocused borders, `│` dividers, muted hint copy | `SEPARATOR` | `Color::DarkGray` |
| Violations | Violation counts above zero | `VIOLATIONS` | `Color::Red` |
| Clean | Violation counts at zero, installed adapters | `CLEAN` | `Color::Green` |
| Pending | Busy state, confirm border, warning glyph | `PENDING` | `Color::Yellow` |
| Directory | Directory names, Contract layer badge | `DIRECTORY` | `Color::Blue` |
| Capabilities badge | `[cap]` layer badge | `CAPABILITIES_BADGE` | `Color::Magenta` |
| Background | Panel and overlay canvas fill | `BACKGROUND` | `Color::Black` |
| Path input | Path dialog input text | `PATH_INPUT` | `Color::Yellow` |
| Focus confirm | Path dialog `>` prompt | `FOCUS_CONFIRM` | `Color::Green` |
| Scrollbar | Scrollbar thumb and track | `SCROLLBAR` | `Color::DarkGray` |
| Header | App title in the header row | `HEADER` | `Color::Cyan` |

Every value resolves through `color()`, which returns `Color::Reset` when
`NO_COLOR` is set.

### DS-TUIC-003: Typography

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
| Body | Every panel line | terminal default | none |
| Heading | Panel titles, phase label, selected name | `EMPHASIS_HEADING` | `Modifier::BOLD` |
| Selected | Selected row, confirm label | `EMPHASIS_SELECTED` | `Modifier::BOLD` |

Typography is owned by the terminal emulator: the app sets no font, size, or
leading. The only typographic control it holds is weight, and it exposes
exactly two named modifiers so a renderer never writes `Modifier::BOLD` inline.

### DS-TUIC-004: Layout & Spacing

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
| Header row | App title and current path | literal | `Constraint::Length(1)` |
| Panel band | Tree, file list, and preview | literal | `Constraint::Min(10)` |
| Shortcut band | Three rows of key hints | literal | `Constraint::Length(3)` |
| Status row | Status, selection, violation count | literal | `Constraint::Length(1)` |
| Tree column | Three-column mode, left panel | `PANEL_TREE_WIDTH_PCT` | 35% |
| File list column | Three-column mode, middle panel | `PANEL_FILE_LIST_WIDTH_PCT` | 20% |
| Preview column | Three-column mode, right panel | `PANEL_PREVIEW_WIDTH_PCT` | 45% |
| Narrow breakpoint | Below this the band shows one panel | `NARROW_BREAKPOINT_WIDTH` | 100 cols |
| Minimum width | Below this the layout is refused | `MIN_TERMINAL_WIDTH` | 40 cols |
| Minimum height | Below this the layout is refused | `MIN_TERMINAL_HEIGHT` | 15 rows |

Spacing is one cell: panel borders sit adjacent with no gutter, so the only
horizontal separation is the `│` of two touching borders. Panel geometry is
computed once per frame by `compute_panel_layout` and shared by the renderer
and the mouse hit-test, so a click cannot land in a different panel than the
one it appears to.

### DS-TUIC-005: Elevation & Depth

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
| Panel | Base surface | `Borders::ALL` | one-cell border |
| Overlay | Path dialog, confirm gate | `Clear` then draw | paints over every panel |

There is no z-order beyond two levels: base surfaces and overlays. An overlay
clears its rect before drawing, so panel content never bleeds through a modal.

### DS-TUIC-006: Shapes

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
| Panel border | Every panel | `Borders::ALL` | single line |
| Confirm border | Destructive-action gate | `ratatui::symbols::border::THICK` | thick line |
| Confirm border (NO_COLOR) | Same gate, monochrome | `ASCII_BORDER` | `+ - \|` |
| Scrollbar | Vertical right edge | `ScrollbarOrientation::VerticalRight` | `↑`/`↓` end symbols |
| Separator | Status and header dividers | `│` → `\|` under `NO_COLOR` | one cell |
| Warning glyph | Violation counts, confirm modal | `⚠` → `!` under `NO_COLOR` | one cell |
| Bar filled / empty | Scan progress | `█`/`░` → `=`/space under `NO_COLOR` | one cell |

Glyph and border fallbacks live in the theme module (`glyph::*`) so a new
overlay cannot ship a second, different ASCII vocabulary under the same
`NO_COLOR` contract.

### DS-TUIC-007: Kind

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
| Panel | Browsable region with focus border | `Block` + `List`/`Paragraph` | — |
| Modal | Blocking gate over all panels | `ConfirmModal` | 5 rows |
| Dialog | Text entry over all panels | `PathScreen` | 60% × 30% centered |
| Status line | One-line run state | `StatusComponent` | one row |

### DS-TUIC-008: Request Shape

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
| Key event | Keyboard binding resolved through the table | `SHORTCUTS` | `Trigger` → `ShortcutAction` → `TuiEvent` |
| Mouse click | Panel-relative target | `hit_test` | `Tree` \| `FileList{row}` \| `Preview{row}` |
| Progress message | Worker thread to event loop | `ScanUpdate` | `Progress` \| `Complete` \| `Cancelled` |

No renderer reads a raw crossterm event; the event loop translates first.

### DS-TUIC-009: Invariants

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
| Single busy slot | Scan and background action cannot overlap | `BusySlot` | one of `Scan`, `Action` |
| Single binding table | Dispatch, shortcut bar, and help share one source | `SHORTCUTS` | `surface_bindings_store` |
| Single layout source | Renderer and hit-test share one geometry | `compute_panel_layout` | `surface_tui_layout` |
| Single severity badge | Every surface prints one vocabulary | `Severity::badge` | `[CRIT]` … `[INFO]` |

### DS-TUIC-010: Change Checklist

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
| New color | Add the token here and in `utility_tui_theme` | — | both, in one change |
| New key | Add one `Shortcut` row | `SHORTCUTS` | bar and help update themselves |
| New overlay | Reuse `glyph::*` fallbacks and `centered_rect` | — | no local border set |
| New state | Add a row to DS-TUIC-011 and DS-TUIC-012 | — | no blank panel |

### DS-TUIC-011: States

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
| Idle | Nothing has run in this directory | preview copy | names `c` and `s` |
| Loading | Scan in flight, count unknown | status line | `waiting for file count…` |
| Loading (determinate) | Scan in flight, count known | status line | `bar 4021/10240 files` |
| Empty | Directory holds no entries | Files panel | `No files in this folder` |
| Empty (root) | Path dialog dismissed on first run | Files panel | names the `r` binding |
| Success | Action finished with no violations | status line | `Done` |
| Partial | Action finished with violations | status line | `N violations` + per-file counts |
| Error | Action or read failed | preview body | one named next action |
| Disabled | Action cannot run in TUI | preview body | points at the CLI command |

### DS-TUIC-012: Error States

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
| Unreadable file | Permission, binary, or I/O error | preview body | `Cannot read this file ({reason})` |
| Empty file | File exists and has no content | preview body | `Empty file` |
| Inaccessible directory | `read_dir` returned nothing for a non-empty reason | Files panel | `Cannot open this folder` |
| Busy slot declined | A key arrived while another action runs | status line | `Busy: {holder} is running` |
| Invalid path | Path dialog input is not a directory | status line | names `Tab` for CWD |
| Scan failed | Dispatcher reported failure | status line + preview | retry hint, same key |

Every error names one action available from the screen it appears on.

### DS-TUIC-013: Accessibility

| Variant | Use case | Token | Value|
|---------|----------|-------|-------|
| Monochrome | `NO_COLOR` set | `color()` | `Color::Reset` |
| High contrast | Low-contrast terminal theme | `LINT_ARWAKY_TUI_HIGH_CONTRAST` | reversed selection |
| Keyboard only | Every action reachable without a pointer | `SHORTCUTS` | full coverage |
| Screen reader | Full-screen redraw is unreadable to AT | help overlay | points at `lint-arwaky-cli` |
| Minimum size | Layout refuses below 40×15 | `MIN_TERMINAL_*` | message states both sizes |

A full-screen, cursor-repositioning terminal UI has no accessibility tree, so
the app cannot expose panel semantics to a screen reader. The contract is
therefore explicit: every action has a keyboard path, the help overlay names
the CLI equivalent for screen-reader users, and no state is conveyed by colour
alone — each one also carries a glyph or a word.

## Reference

| Artifact | Purpose | Link|
|----------|---------|------|
| BACKLOG.md | Backlog tracking | [BACKLOG.md](BACKLOG.md)|
| PRD.md | Product requirements | [PRD.md](../../PRD.md)|
| ROADMAP.md | Roadmap status | [ROADMAP.md](../../ROADMAP.md)|
| ARCHITECTURE.md | Layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md)|
