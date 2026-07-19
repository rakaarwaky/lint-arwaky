# Computation Detection Rules

Agent layer must not contain domain computation.

## Forbidden

- arithmetic,
- totals,
- averages,
- counts used as domain decisions,
- sum,
- fold,
- parsing,
- normalization,
- deriving domain meaning from data.

## Allowed

- iterating to call dependencies,
- routing results,
- pushing results into a collection,
- propagating errors,
- continuing or stopping pipeline.

## Bad

```rust
let total = files.len();
let average = total_score / total;
```

## Good

```rust
let summary = self.analyzer.summarize(files);
```

Note: Patterns like `.iter()` or `.map()` can appear in harmless technical code. Always inspect context. The real violation is **domain computation/transformation**, not merely using iterator control flow.
