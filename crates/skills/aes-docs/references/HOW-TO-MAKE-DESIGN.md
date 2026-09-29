# HOW TO MAKE DESIGN.md

> **Purpose**: Define the visual and interaction contract of a surface so
> designers and engineers can implement or audit it without asking the
> author.
>
> **Audience**: Designers, Surface Engineers, TUI developers.
>
> **Scope**: One top-level design spec per project. Optional theme — adopt
> only when the surface exposes visual tokens (colors, type scale,
> component anatomy).
>
> **Location**: Project root.
>
> **Length**: 50–500 lines (flat budget shared by every document type).

---

## Rules

1. **Tokens, not screenshots.** A color name maps to a single CSS variable
   (or language-equivalent token). Tokens stay consistent; names drift.
   Every visual decision must be traceable to a token row.
2. **Colors are roles, not palettes.** Base surfaces build the canvas.
   Primary, Secondary, and Tertiary accents cover the semantic hierarchy.
   Do not add more than three accent roles — if you need a fourth, reuse
   one of the diagnostics.
3. **Diagnostics share a base value.** Error, warning, info, and success
   colors come from the same chromatic family so they compare cleanly
   and do not compete with each other on the canvas.
4. **Hairlines and contours are structural.** Hairlines mark containment;
   contours mark hierarchy. Both belong in the spec because they affect
   layout and contrast, not just decoration.
5. **Typography has one fixed scale.** Name the scale (e.g. Major Third
   1.250) and list every stop. Every heading, label, and body text maps
   to one stop. If a size cannot map, either rewrite the rule or drop
   the size.
6. **Spacing multiplies from one unit.** A single base unit powers every
   margin, padding, and gap. Every declared spacing value equals an
   integer multiple of that unit.
7. **Elevation is a discrete ladder.** Shadows map to z-order, not to
   ad-hoc depth values. Every elevation level gets a named stop and a
   clear use case.
8. **Component anatomy is a table, not a paragraph.** Each row covers
   Variant, Use case, States, and Token mapping. Paragraph prose hides
   states and invites inconsistency.

---

## Workflow

1. **Create file** → `DESIGN.md` at repo root.
2. **Section: Brand & Style** — Colors first, then Typography, Layout &
   Spacing, Elevation, and Shapes.
3. **Section: Components** — add the sub-sections in the order below;
   create a sub-section only when that component family exists on the
   surface.
4. **Verify** → `lint-arwaky-cli docs` passes; required sections exist.

## Template

Copy, fill, delete nothing. Replace italicized placeholders with your
values; omit a sub-section entirely when the surface has no equivalent.
Never insert a zero-row table — an empty table is easier to miss than a
missing section.

```markdown
# DESIGN — <project-name>

> Visual and interaction contract. Tokens, scales, and component anatomy.
> Audience: designers, surface engineers, TUI developers.
> Condition: [ROADMAP.md](ROADMAP.md) (workspace) + each feature
> [BACKLOG.md](BACKLOG.md).

## Brand & Style

### Colors

#### Base Surfaces

| Token            | Value | Usage |
|------------------|-------|-------|
| `--surface-base` | <hex> | default background |
| `--surface-muted`| <hex> | card backgrounds, alternate rows |
| `--surface-elevated` | <hex> | overlays, popups |

#### Primary Accent

| Token           | Value | Usage |
|-----------------|-------|-------|
| `--color-primary`   | <hex> | primary actions, links, active states |

#### Secondary Accent

| Token             | Value | Usage |
|-------------------|-------|-------|
| `--color-secondary` | <hex> | secondary actions, secondary emphasis |

#### Tertiary Accent

| Token             | Value | Usage |
|-------------------|-------|-------|
| `--color-tertiary`  | <hex> | tertiary accents, low-emphasis highlights |

#### System Diagnostics

| Token               | Value | Usage |
|---------------------|-------|-------|
| `--color-error`         | <hex> | error states, validation failures |
| `--color-warning`       | <hex> | warning states, caution notices |
| `--color-info`          | <hex> | informational banners |
| `--color-success`       | <hex> | success confirmations |

#### Hairlines and Contours

| Token                      | Value | Usage |
|----------------------------|-------|-------|
| `--hairline-thin`            | <px>  | separator, hairline dividers |
| `--hairline-thick`           | <px>  | strong borders |
| `--contour-radius-sm`        | <px>  | small component radius |
| `--contour-radius-md`        | <px>  | card / dialog radius |
| `--contour-radius-lg`        | <px>  | overlay / modal radius |

### Typography

| Token                  | Value                 | Usage |
|------------------------|-----------------------|-------|
| `--font-family-sans`   | <font>                | body, headings |
| `--font-family-mono`   | <font>                | code, literals |
| `--type-scale-xs`      | <size>/<line-height>  | caption |
| `--type-scale-sm`      | <size>/<line-height>  | label, helper |
| `--type-scale-md`      | <size>/<line-height>  | body |
| `--type-scale-lg`      | <size>/<line-height>  | heading level 3 |
| `--type-scale-xl`      | <size>/<line-height>  | heading level 2 |
| `--type-scale-xxl`     | <size>/<line-height>  | heading level 1 |
| `--type-scale-display` | <size>/<line-height>  | display / hero |

### Layout & Spacing

| Token          | Value | Usage |
|----------------|-------|-------|
| `--space-unit` | <px>  | base spacing unit |
| `--space-xs`   | <px>  | 1 × `--space-unit` |
| `--space-sm`   | <px>  | 2 × `--space-unit` |
| `--space-md`   | <px>  | 3 × `--space-unit` |
| `--space-lg`   | <px>  | 4 × `--space-unit` |
| `--space-xl`   | <px>  | 6 × `--space-unit` |
| `--space-xxl`  | <px>  | 8 × `--space-unit` |
| `--grid-gap`   | <px>  | default gap between grid items |

### Elevation & Depth

| Token                 | Value      | Usage |
|-----------------------|------------|-------|
| `--elevation-0`       | none       | flat surfaces |
| `--elevation-1`       | <shadow>   | cards, panels |
| `--elevation-2`       | <shadow>   | popovers, drawers |
| `--elevation-3`       | <shadow>   | dialogs, menus |
| `--elevation-overlay` | <shadow>   | full-screen overlays |

### Shapes

| Token               | Value | Usage |
|---------------------|-------|-------|
| `--shape-circle`    | 50%   | avatar, badge, FAB |
| `--shape-round-sm`  | <px>  | small inputs |
| `--shape-round-md`  | <px>  | buttons, cards |
| `--shape-round-lg`  | <px>  | modals, panels |

## Components

### Buttons & Triggers

| Variant  | Use case        | States                                         | Token map |
|----------|-----------------|------------------------------------------------|-----------|
| `<name>` | <purpose>       | default / hover / pressed / disabled / focus   | <tokens>  |

### Status Indicators & Chips

| Variant  | Use case        | Semantic                  | Color token | States                        |
|----------|-----------------|---------------------------|-------------|-------------------------------|
| `<name>` | <purpose>       | success / warning / error / info | <token>   | default / active / muted      |

### Parallel Stream Cards

| Field           | Role                                     |
|-----------------|------------------------------------------|
| icon / glyph    | visual anchor                            |
| headline        | one-line description                     |
| meta            | secondary metadata                       |
| action trigger  | primary call-to-action                   |
| state layer     | progress / status indicator              |

### Input Fields & Command Bar

| Field      | Role                       | States                                            |
|------------|----------------------------|---------------------------------------------------|
| label      | field identifier           | default / filled / error / disabled / focus       |
| hint       | supporting copy            | default / error                                   |
| input area | typed value                | default / hover / focus / disabled                |
| suffix     | action button              | default / loading / error                         |

### Navigation & Tab Bar

| Level      | Role                     | States                                       |
|------------|--------------------------|----------------------------------------------|
| global nav | workspace-level root     | active / hover / selected                    |
| local tab  | section switcher         | active / inactive / disabled                 |
| breadcrumbs| path context             | default / current                            |

## Reference

- PRD: <link to root PRD.md>
- Backlog: [BACKLOG.md](BACKLOG.md)
- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md)
```

---

## Section Contract

Every section is required when the surface exposes the corresponding
visual dimension. Omit a section only when the surface has no equivalent
need (for example, a headless CLI tool without tabular output may skip
Components → Parallel Stream Cards). Each table row must be populated —
a zero-row table is a silent violation of the contract.

| Section                          | Why it belongs here                                                        |
|----------------------------------|----------------------------------------------------------------------------|
| Brand & Style                    | The token dictionary component authors consume.                            |
| Colors — Base Surfaces           | Everything rests on these three tones.                                     |
| Colors — Primary Accent          | Headline semantic; the first thing readers reach for.                      |
| Colors — Secondary Accent        | Backing semantic; reinforces without competing.                            |
| Colors — Tertiary Accent         | Low-emphasis highlight; underline, not headline.                           |
| Colors — System Diagnostics      | Error/warning/info/success families used by every control.                 |
| Colors — Hairlines & Contours    | Structural lines and radii that create hierarchy.                          |
| Typography                       | One scale, one or two families, every stop assigned.                       |
| Layout & Spacing                 | A single base unit that powers every margin and padding.                   |
| Elevation & Depth                | Shadow stops mapped to z-order, not ad-hoc depth values.                   |
| Shapes                           | Radius/roundness tokens that prevent ad-hoc corners.                       |
| Components — Buttons & Triggers| Interactive entry points with explicit state map.                          |
| Components — Status Indicators & Chips| Semantic chips that communicate system state at a glance.           |
| Components — Parallel Stream Cards| Card-like containers for multi-item streams.                           |
| Components — Input Fields & Command Bar| Textual entry surfaces with labeled validation states.               |
| Components — Navigation & Tab Bar| Hierarchical navigation anchors with active/inactive states.             |
| Reference                        | Connects the spec back to the spec chain.                                  |

---

## Verify

```bash
lint-arwaky-cli docs .
# Checks: dead-link, root-relative-link, absolute-path, secret-in-docs,
# doc-length, unreferenced-file (when checked against skill references).
```

Manual: every color token appears at least once in a component table;
every typography stop is assigned to at least one heading/paragraph use
case; no component table is empty — if a component family has no
instances yet, skip the subsection rather than inserting a zero-row
table.