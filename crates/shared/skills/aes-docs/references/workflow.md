# aes-docs — Workflow

1. **Resolve the repo-root anchor first.** `lint-arwaky-cli docs`
2. **Analyze**: List feature modules and public items. Run `lint-arwaky-cli docs <path>`. The findings are your work list.
3. **Draft PRD**: Write root `PRD.md` per `references/HOW-TO-MAKE-PRD.md`.
4. **Draft Roadmap**: Write root `ROADMAP.md` per `references/HOW-TO-MAKE-ROADMAP.md`.
5. **Draft FRDs**: Write `FRD.md` in each feature dir per `references/HOW-TO-MAKE-FRD.md`.
6. **Draft Feature Backlogs**: Write one `BACKLOG.md` per feature, beside its spec, per `references/HOW-TO-MAKE-BACKLOG.md`.
7. **Draft README**: Write root `README.md` per `references/HOW-TO-MAKE-README.md`.
8. **Draft AGENTS**: Write root `AGENTS.md` per `references/HOW-TO-MAKE-AGENTS.md`.
9. **Draft ARCHITECTURE**: Write root `ARCHITECTURE.md` per `references/HOW-TO-MAKE-ARCHITECTURE.md` — required when the workspace has more than one layer beyond shared.
10. **Draft CONTRIBUTING**: Write root `CONTRIBUTING.md` per `references/HOW-TO-MAKE-CONTRIBUTING.md` — required when the repo accepts external contributions. One section per contribution path the project actually supports; delete the rest.
11. **Draft DESIGN**: Write root `DESIGN.md` per `references/HOW-TO-MAKE-DESIGN.md` — optional; create it when the surface exposes visual tokens (colors, type scale, component anatomy).
12. **Document Code**: Add doc comments to all public items per language:
    - `references/HOW-TO-MAKE-PYTHON-DOC.md`
    - `references/HOW-TO-MAKE-RUST-DOC.md`
    - `references/HOW-TO-MAKE-TYPESCRIPT-DOC.md`
    Then add type annotations to all signatures.
13. **Verify**: Run `lint-arwaky-cli docs <path>`. Then each touched reference's `Verify` block (including the language doc ref).
