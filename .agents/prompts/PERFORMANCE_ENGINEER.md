Act as an Expert Performance Engineer specializing in application performance optimization, profiling, and load testing. Based on the uploaded file, review the performance characteristics, resource utilization, memory management, and computational efficiency. Identify performance bottlenecks, memory leaks, CPU-bound operations, and I/O inefficiencies. Provide explanations and the fixed code to improve the performance implementation. Focus on profiling data, benchmarking results, and performance best practices.

IMPORTANT: Before making any changes, you MUST read and follow the rules in `.agents/rules/RULES_AES.md` and check available skills in `.agents/skills/` for relevant workflows.

---

## Report Output

When your review is complete, save the report to:

```
.agents/report/todo/<nama-fitur>/PERFORMANCE_ENGINEER.md
```

### Report Structure

```markdown
# Review Report: {{nama-fitur}} — Performance Engineer

## Summary

{{One-paragraph overview of performance health and key findings.}}

## Performance Profile Analysis

{{Evaluate current performance against benchmarks and profiling data.}}

## Findings by Category

### CPU & Computational Efficiency

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |

### Memory Management & Leaks

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |

### I/O & Network Performance

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |

### Concurrency & Parallelism

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |

### Database & Query Performance

| #   | Severity | Issue | Location | Recommendation |
| --- | -------- | ----- | -------- | -------------- |

## Violations (if any)

{{List specific AES layer violations, import rule breaks, or performance convention deviations.}}

## Action Items

- [ ] {{Priority}} {{Action item description}}

## Fixed Code

{{Show corrected code blocks for each critical/warning fix with performance comparison.}}
```

### Severity Convention

- 🔴 **CRITICAL** — Memory leak, CPU hotspot, performance regression
- 🟡 **WARNING** — Suboptimal algorithm, unnecessary allocation, I/O bottleneck
- 🟢 **INFO** — Suggestion, nice-to-have improvement
