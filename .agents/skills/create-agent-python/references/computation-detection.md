# Computation Detection Rules

Agent layer must not contain domain computation.

## Forbidden

- arithmetic, totals, averages, counts used as domain decisions,
- sum, fold, parsing, normalization, deriving domain meaning from data.

## Allowed

- iterating to call dependencies, routing results,
- pushing results into a collection, propagating errors,
- continuing or stopping pipeline.

## Bad

```python
total = len(files)
average = total_score / total
```

## Good

```python
summary = self.analyzer.summarize(files)
```
