# aes-docs — Diagnostic Tree

Ask these questions in order. The first "No" dictates your next action.

1. **Can a stakeholder understand this project's purpose in 30 seconds?**
   - *No* → Add `PRD.md` (what/why).
2. **Can a tech lead see every feature, shared policy, and workspace truth in one place?**
   - *No* → Add `ROADMAP.md` (`Progress Overview` / `Roadmap Status` / `Reference`).
3. **Can an engineer implement this from the spec alone?**
   - *No* → Add `FRD.md` (how).
4. **Can a reader tell what is actually true today for one feature, and re-run the evidence?**
   - *No* → Add `BACKLOG.md` beside that feature's spec.
5. **Can a developer clone, build, and run in under 10 minutes?**
   - *No* → Add `README.md` (how to use).
6. **Can an agent work here safely without being told twice?**
   - *No* → Add `AGENTS.md` (how to work here).
7. **Can a new engineer tell which layer a file belongs in, and what it may import?**
   - *No* → Add `ARCHITECTURE.md` (layer boundaries).
8. **Can a first-time contributor set up the project and open a passing PR?**
   - *No* → Add `CONTRIBUTING.md` (how to ship a change).
