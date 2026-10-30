# ROADMAP Rules

**Purpose:** Index features, own the state vocabulary, and record workspace-level condition.

**Audience:** Tech Lead, PM, Engineers.

**Boundary:** The single source of truth for workspace state, cross-cutting risks, and feature tracking.

**Scope:** Exactly one ROADMAP.md at the project root.

**Location:** Project root.

**Length:** 50–500 lines.

## Rules

1. **Single source of truth.** State vocabulary lives here and nowhere else. Feature backlogs cite it; they never redefine it.
2. **Index every feature.** Every feature directory must appear in the Roadmap Status as a `PR-<PROJECT-NAME-ID>-NNN` entry, linking to its `BACKLOG.md` (`no-master-roadmap`).
3. **Cross-cutting rows only.** The workspace table is for items that span multiple features or infrastructure.
4. **ID scopes are explicit.** The root file uses a workspace prefix (`WS-01`); feature rows keep their `<SCOPE>-` prefix and stay in their own BACKLOG.md.
5. **Priority-ordered IDs.** `PR-<PROJECT-NAME-ID>-NNN` entries are listed in strict priority order: all P0 entries first, then P1, then P2. The `NNN` counter increments across all entries; the first entry is `-001` and is always P0. No P1 entry may appear before a P0 entry, and no P2 entry before a P1 entry. Within the same tier, NNN order is the author's call, but it must not jump tiers.
6. **Priority comes from the PRD.** Each entry's `Priority` row must match the tier assigned to that same PR-ID in the root `PRD.md`. The ROADMAP cannot demote or promote an entry the PRD declared at a different tier.
7. **Every entry links its BACKLOG.** Each `PR-<PROJECT-NAME-ID>-NNN` row carries a `Backlog` cell pointing at the feature's `BACKLOG.md` path.
