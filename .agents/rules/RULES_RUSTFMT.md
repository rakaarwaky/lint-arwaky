# Rustfmt Formatting Rules

See [README.md](../README.md) for Rust adapter usage and [RULES_CLIPPY.md](RULES_CLIPPY.md) for related Rust linting.

Rustfmt is the official Rust code formatter, available at https://github.com/rust-lang/rustfmt. It enforces consistent code style across the entire Rust ecosystem.

## Checking Formatting

```bash
# Check formatting without modifying
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all
```

## Key Checks

| Check                       | Description                                                          | Violation   |
| --------------------------- | -------------------------------------------------------------------- | ----------- |
| trailing_comma              | Missing trailing comma in match arms, struct fields, function params | Reformatted |
| max_width                   | Line exceeds configured max width (default 100)                      | Reformatted |
| tab_spaces                  | Indentation uses wrong number of spaces (default 4)                  | Reformatted |
| newline_style               | Wrong line ending style (LF vs CRLF)                                 | Reformatted |
| reorder_imports             | Imports not grouped/ordered correctly                                | Reformatted |
| reorder_modules             | Module declarations not in correct order                             | Reformatted |
| reorder_impl_items          | Items within `impl` block not ordered                                | Reformatted |
| blank_lines_upper_bound     | Too many blank lines between items                                   | Reformatted |
| blank_lines_lower_bound     | Missing blank lines between items                                    | Reformatted |
| use_small_heuristics        | Braces/spacing not following small heuristics                        | Reformatted |
| fn_params_layout            | Function parameters not formatted correctly                          | Reformatted |
| match_block_trailing_comma  | Missing trailing comma in match arms                                 | Reformatted |
| struct_lit_single_line      | Struct literal could fit on single line                              | Reformatted |
| imports_layout              | Import block layout not correct (mixed styles)                       | Reformatted |
| merge_imports               | Multiple imports from same module not merged                         | Reformatted |
| use_try_shorthand           | `try!(expr)` not replaced with `expr?`                               | Reformatted |
| use_field_init_shorthand    | Redundant field name syntax in struct init                           | Reformatted |
| normalize_comments          | Comments not normalized to preferred style                           | Reformatted |
| wrap_comments               | Comments exceeding max width not wrapped                             | Reformatted |
| format_code_in_doc_comments | Code blocks in doc comments not formatted                            | Reformatted |
| format_macro_bodies         | Macro body not formatted                                             | Reformatted |
| format_strings              | String literals exceeding max width not wrapped                      | Reformatted |
