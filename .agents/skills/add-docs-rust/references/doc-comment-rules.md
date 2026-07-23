# Doc Comment Rules

## Purpose

`///` doc comments provide API documentation visible in `cargo doc` and IDEs.

## Rules

1. **Every public struct** — `///` doc comment with summary
2. **Every public method** — `///` doc comment with Args/Returns/Errors/Example
3. **Explain "what" and "why"** — not "how" (code shows how)
4. **Use `# Arguments`** — document each parameter
5. **Use `# Returns`** — document return value
6. **Use `# Errors`** — document error conditions
7. **Use `# Example`** — provide valid Rust example code

## Template

````rust
/// One-liner describing struct purpose.
pub struct MyStruct {
    // ...
}

/// One-liner describing method purpose.
///
/// # Arguments
///
/// * `param1` - Description of param1
/// * `param2` - Description of param2
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// Returns `Err` when error condition occurs
///
/// # Example
///
/// ```
/// let result = my_method("input")?;
/// ```
pub fn my_method(param1: &str, param2: i32) -> Result<bool, Error> {
    // ...
}
````

## Anti-Patterns

- ❌ Missing doc comments → every public item needs `///` doc comment
- ❌ Using `//` instead of `///` → use `///` for cargo doc visibility
- ❌ Missing parameter documentation → all parameters must be documented
- ❌ Missing error documentation → document error conditions
- ❌ Invalid example code → ensure examples compile
- ❌ Explaining "how" instead of "what/why" → code shows how
