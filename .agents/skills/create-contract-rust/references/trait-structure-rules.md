# Trait Structure Rules

## 1. Contracts contain trait definitions only

Good:

```rust
pub trait IImportForbiddenProtocol: Send + Sync {
    fn check(&self, source: &SourceContentVO) -> Vec<LintResult>;
}
```

## 2. No default method bodies

Default methods are implementation logic.

## 3. No private helpers or internal stepping stones

Implementation-specific methods belong in the implementor.

## 4. Traits intended for DI MUST be `Send + Sync`

Required for `Arc<dyn Trait>` usage.

## 5. Contracts MUST be object-safe

Avoid patterns that break object safety for `dyn Trait`.

## 6. Async contracts MUST remain dyn-compatible

Use `async_trait` or explicit boxed futures.

## 7. Error strategy

Prefer shared taxonomy error types in contract signatures.
