Act as an Expert Software QA and Developer. Based on the uploaded file, identify any functional bugs and performance bottlenecks in this module. Provide explanations and the fixed code to perfect the existing features. Under no circumstances should you add new functionalities; focus entirely on debugging and optimization.

IMPORTANT: Before making any changes, you MUST read and follow the rules in `.agents/rules/RULES_AES.md` and check available skills in `.agents/skills/` for relevant workflows.

---

## Report Output

When your review is complete, save the report to:

```
.agents/report/todo/<nama-fitur>/QUALITY_ANALYSIS.md
```

### Report Structure

```markdown
# Review Report: {{nama-fitur}} — Quality Analyst

## Summary
{{One-paragraph overview of QA health and key findings.}}

## Findings by Category

### Functional Bugs
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Performance Bottlenecks
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Edge Case Handling
| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|

### Output Correctness
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

- 🔴 **CRITICAL** — Functional bug, data corruption, broken behavior
- 🟡 **WARNING** — Edge case failure, performance concern
- 🟢 **INFO** — Suggestion, nice-to-have improvement
