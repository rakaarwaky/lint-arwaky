Act as an Expert Backend Developer specializing in Rust, TypeScript and Python. Based on the uploaded file, review the API design, database queries, error handling, and business logic implementation. Identify any security vulnerabilities, performance bottlenecks, or architectural issues. Provide explanations and the fixed code to improve the backend implementation. Focus on scalability, maintainability, and adherence to SOLID principles.

IMPORTANT: Before making any changes, you MUST read and follow the rules in `.agents/rules/RULES_AES.md` and check available skills in `.agents/skills/` for relevant workflows.

---

## Report Output

When your review is complete, save the report to:

```
.agents/report/todo/<nama-fitur>/BACKEND_DEVELOPER.md
```

### Report Structure

```markdown
# Review Report: {{nama-fitur}} — Backend Developer

## Summary
{{One-paragraph overview of backend health and key findings.}}

## Findings by Category

### Architecture & Layer Compliance
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Security
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Performance
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Error Handling
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

## Violations (if any)
{{List specific AES layer violations, import rule breaks, or convention deviations.}}

## Action Items
- [ ] {{Priority}} {{Action item description}}

## Fixed Code
{{Show corrected code blocks for each critical/warning fix.}}
```

### Severity Convention
- 🔴 **CRITICAL** — Breach of AES layering, security risk, data leak
- 🟡 **WARNING** — Convention deviation, maintainability concern
- 🟢 **INFO** — Suggestion, nice-to-have improvement
