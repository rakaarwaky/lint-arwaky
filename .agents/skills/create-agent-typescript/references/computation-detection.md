# Computation Detection Rules

Agent layer must not contain domain computation.

## Forbidden

- arithmetic, totals, averages, counts used as domain decisions,
- reduce, fold, parsing, normalization, deriving domain meaning from data.

## Allowed

- iterating to call dependencies, routing results,
- pushing results into a collection, propagating errors,
- continuing or stopping pipeline.

## Bad

```typescript
const total = files.length;
const average = totalScore / total;
```

## Good

```typescript
const summary = this.analyzer.summarize(files);
```
