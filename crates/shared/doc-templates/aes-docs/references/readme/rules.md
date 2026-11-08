# README Rules

**Purpose:** Answer *how do I get this running, and where do I look next*.

**Audience:** A developer who just cloned the repo, possibly you in six months.

**Scope:** Onboarding: prerequisites, quick start, commands.

**Location:** Project root.

**Length:** 50–500 lines (flat budget shared by every document type).

## Rules

1. **The test is time, not completeness.** Clone → install → build → run in under 10 minutes. If the Quick Start cannot clear that, the missing step is the bug in this file.
2. **Every command was actually run, and says how to confirm it worked.** A reader who cannot tell success from silence will assume failure and go ask a human.
3. **Prerequisites are the ones the build fails without**, with versions pinned. Optional tooling listed as required teaches readers to distrust the list.

## Section Contract

| Section | Why it belongs here |
| --------- | --------------------- |
| Project Overview | Decides in five seconds whether to keep reading. |
| Prerequisites | Prevents a build failure with no cause. Watch for optional tooling listed as required. |
| Quick Start | The 10-minute promise, as `Step \| Action \| Command`. |
| Reference | Required H2; carries an empty `Artifact \| Purpose \| Link` table when no artifact is mandated. |
