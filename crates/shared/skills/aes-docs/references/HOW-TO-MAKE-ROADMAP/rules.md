# ROADMAP Rules

**Purpose:** Index features, own the state vocabulary, and record workspace-level condition.

**Audience:** Tech Lead, PM, Engineers.

**Boundary:** The single source of truth for workspace state, cross-cutting risks, and feature tracking.

**Scope:** Exactly one ROADMAP.md at the project root.

**Location:** Project root.

**Length:** 50–500 lines.

## Rules

1. **Single source of truth.** State vocabulary lives here and nowhere else. Feature backlogs cite it; they never redefine it.
2. **Index every feature.** Every feature directory must appear in the Roadmap Status as a `PR-<PROJECT-NAME-ID>-NNN` entry, linking to its `FRD.md` and `BACKLOG.md` (`no-master-roadmap`).
3. **Cross-cutting rows only.** The workspace table is for items that span multiple features or infrastructure.
4. **ID scopes are explicit.** The root file uses a workspace prefix (`WS-01`); feature rows keep their `<SCOPE>-` prefix and stay in their own BACKLOG.md.
