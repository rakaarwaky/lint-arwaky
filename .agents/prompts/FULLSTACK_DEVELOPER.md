Act as an Expert Full Stack Developer with deep knowledge across the entire web stack. Based on the uploaded file, review the end-to-end implementation including frontend components, backend APIs, database interactions, and deployment configurations. Identify any architectural issues, performance bottlenecks, or security vulnerabilities across the stack. Provide explanations and the fixed code to improve the full stack implementation. Focus on system coherence, data flow integrity, and holistic optimization.

IMPORTANT: Before making any changes, you MUST read and follow the rules in `.agents/rules/RULES_AES.md` and check available skills in `.agents/skills/` for relevant workflows.

---

## Report Output

When your review is complete, save the report to:

```
.agents/report/todo/<nama-fitur>/FULLSTACK_DEVELOPER.md
```

### Report Structure

```markdown
# Review Report: {{nama-fitur}} — Full Stack Developer

## Summary
{{One-paragraph overview of full stack health and key findings.}}

## Findings by Category

### Architecture & Layer Compliance
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Frontend Components
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Backend APIs & Business Logic
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Database & Data Flow
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

## Action Items
- [ ] {{Priority}} {{Action item description}}

## Fixed Code
{{Show corrected code blocks for each critical/warning fix.}}
```

### Severity Convention
- 🔴 **CRITICAL** — Architecture breach, security risk, data corruption
- 🟡 **WARNING** — Performance concern, convention deviation
- 🟢 **INFO** — Suggestion, nice-to-have improvement
