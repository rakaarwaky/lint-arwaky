# Clippy Linting Rules

Clippy is the official Rust linter, available at https://github.com/rust-lang/rust-clippy. It catches common mistakes and enforces idiomatic Rust code.

## Rule Categories

| Prefix      | Category    | Description                                           |
| ----------- | ----------- | ----------------------------------------------------- |
| correctness | Correctness | Code that is outright wrong or has undefined behavior |
| style       | Style       | Code that is less readable or not idiomatic           |
| complexity  | Complexity  | Code that is unnecessarily complex                    |
| perf        | Performance | Code that is sub-optimal in performance               |
| pedantic    | Pedantic    | Opinionated/enforced best practices (opt-in)          |
| nursery     | Nursery     | New lints still under development                     |
| cargo       | Cargo       | Issues with Cargo.toml manifest                       |
| suspicious  | Suspicious  | Code that is likely incorrect or bug-prone            |

## Key Rules

| Code        | Rule                         | Severity                                                     |
| ----------- | ---------------------------- | ------------------------------------------------------------ |
| correctness | unused                       | Unused variables, functions, imports                         |
| correctness | needless_deref               | Unnecessary dereference (`*x` when `x: &T`)                  |
| correctness | needless_borrow              | Unnecessary borrow (`&x` when `x: &T`)                       |
| correctness | let_unit_value               | `let _ = ` instead of `let _ = expr` for side effects        |
| correctness | no_effect                    | Expression with no effect (e.g., bare boolean literal)       |
| correctness | unused_unit                  | Unnecessary unit expression                                  |
| style       | single_char_add_str          | Using `push_str("x")` instead of `push('x')`                 |
| style       | unnecessary_lazy_evaluations | Unnecessary `unwrap_or_else` when `unwrap_or` suffices       |
| style       | redundant_field_names        | Redundant field names in struct literal                      |
| style       | unnecessary_cast             | Unnecessary type cast (`x as i32` when already i32)          |
| style       | useless_conversion           | Unnecessary `.into()` or `.to_owned()`                       |
| complexity  | too_many_arguments           | Function with too many parameters (>7)                       |
| complexity  | too_many_lines               | Function/closure with too many lines (>100)                  |
| complexity  | cognitive_complexity         | Function exceeds cognitive complexity limit                  |
| complexity  | cyclomatic_complexity        | Function exceeds cyclomatic complexity limit                 |
| complexity  | unnecessary_sort_by          | Complex sort closure replaceable with simpler call           |
| perf        | large_enum_variant           | Enum variant with large size difference                      |
| perf        | needless_heap_allocations    | Unnecessary `Box`, `Vec`, or `String` allocation             |
| perf        | slow_vector_initialization   | Slow vector initialization pattern                           |
| perf        | unnecessary_to_owned         | Unnecessary `.to_owned()` or `.clone()`                      |
| perf        | redundant_clone              | Call to `.clone()` that is unnecessary                       |
| suspicious  | unused                       | Dead code, unused imports                                    |
| suspicious  | similarity                   | Near-identical code blocks detected (copy-paste)             |
| suspicious  | float_equality_without_abs   | Float equality comparison without epsilon                    |
| suspicious  | assert_eq_on_mut             | Assert on mutable reference produces inconsistent debugging  |
| pedantic    | missing_errors_doc           | Public function returning `Result` lacks error documentation |
| pedantic    | missing_panics_doc           | Public function that can panic lacks documentation           |
| pedantic    | wildcard_imports             | Use of `use module::*` wildcard imports                      |
| pedantic    | module_name_repetitions      | Module name repeated in struct name pattern                  |
| cargo       | cargo_common_metadata        | Missing common fields in Cargo.toml                          |
| cargo       | multiple_crate_versions      | Multiple versions of the same crate in dependencies          |
| cargo       | wildcard_dependencies        | Dependencies with version `"*"` wildcard                     |
