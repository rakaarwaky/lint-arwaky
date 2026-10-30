# ARCHITECTURE Rules

**Purpose:** Document the layered architecture — what each layer is, what it may depend on, and what it must not touch — so both humans and agents modify the system safely.

**Audience:** Engineers, Tech Lead, AI agents that read or generate structural code.

**Scope:** The architectural rules that govern the workspace. Exactly one per project root.

**Location:** Project root (or at `README.md` level when the repo has a single crate/package).

**Length:** 50–500 lines (flat budget shared by every document type).

## Rules

1. **Write rules, not descriptions.** Every section must say what an agent or engineer is allowed to do or not do. A paragraph that reads like marketing text instead of an enforcement rule is the bug in this file.
2. **Dependencies are explicit.** Every layer states exactly what it may import from, and exactly what it may not. Vague "follows best practices" lines are replaced with named-layer restrictions (`Layer X depends only on Layer Y`).
3. **No per-file exceptions without a reason.** When a layer has a special case, state the exception as its own subsection and name the failure mode it prevents.
4. **One diagram per page.** ASCII trees are acceptable. Embedding large PNG or SVG images violates the length budget and breaks copy-paste for agents.
5. **Do not restate code.** If a pattern is visible in the source already, the architecture file explains *why* the pattern exists, not the implementation.
6. **Name the naming convention.** File names must communicate three parts — layer, concern, role — joined by underscores, followed by the language extension: `layer_concern_role.<ext>`.
7. **No implementation detail.** Concrete class names, API signatures, and library choices belong in `FRD.md`. This file owns the boundary rules; FRD owns the per-feature mechanics.
8. **Nothing about the current build.** Whether a layer shipped or not is a backlog claim with evidence, not an architecture sentence.
