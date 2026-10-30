# AGENTS Workflow

Follow these steps in order. The output must match the Template exactly.

1. **Determine context** — Agent config for single tool or multi-agent system.
2. **Write frontmatter** — name, description, persona, tools.
3. **Write AGENTS.md** — behavior rules, guardrails, response format.
4. **Verify** → validate with command `lint-arwaky-cli docs`.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: agents-section-missing, ci-command-drift, absolute-path, secret-in-docs, dead-link, doc-length.
```
