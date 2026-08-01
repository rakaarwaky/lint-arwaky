use async_trait::async_trait;
use serde_json::Value;
use shared::cli_commands::{LintResult, LintResultList};

use shared::code_analysis::{ILinterAdapterProtocol, LinterOperationError};

use shared::common::{AdapterError, FilePath, ICommandExecutorProtocol, Severity};

use shared::common::{
    AdapterName, ColumnNumber, ComplianceStatus, ErrorCode, ErrorMessage, LineNumber, LintMessage,
    LocationList, PatternList,
};
use shared::external_lint::utility_external_lint::resolve_cargo_working_dir;

use std::sync::Arc;
use tracing::debug;

// ─── Block 1: Struct Definition ───────────────────────────

/// Adapter for Rust Clippy static analysis.
pub struct RustLinterAdapter {
    executor: Arc<dyn ICommandExecutorProtocol>,
    _bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ILinterAdapterProtocol for RustLinterAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("clippy")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let working_dir = resolve_cargo_working_dir(path);
        let working_dir_str = &working_dir.value;

        let cargo_toml = Path::new(working_dir_str).join("Cargo.toml");
        if !cargo_toml.exists() {
            debug!(
                "Skipping clippy scan: Cargo.toml not found at {:?}",
                cargo_toml
            );
            return Ok(LintResultList::new(results));
        }

        let cmd = vec![
            "cargo".to_string(),
            "clippy".to_string(),
            "--message-format=json".to_string(),
        ];
        let result = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir.clone(),
                Some(shared::taxonomy_duration_vo::Timeout::new(180.0)),
            )
            .await
            .map_err(|e| {
                LinterOperationError::Adapter(AdapterError::new(
                    self.name(),
                    ErrorMessage::new(e.to_string()),
                ))
            })?;

        let output = if result.stdout.trim().is_empty() {
            result.stderr.clone()
        } else {
            result.stdout.clone()
        };

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() || !line.starts_with('{') {
                continue;
            }
            match serde_json::from_str::<Value>(line) {
                Ok(data) => {
                    if data.get("reason").and_then(|r| r.as_str()) != Some("compiler-message") {
                        continue;
                    }
                    let msg = match data.get("message") {
                        Some(m) => m,
                        None => continue,
                    };
                    let level = match msg.get("level").and_then(|l| l.as_str()) {
                        Some(l) => l.to_lowercase(),
                        None => "warning".to_string(),
                    };
                    let code = match msg
                        .get("code")
                        .and_then(|c| c.get("code"))
                        .and_then(|c| c.as_str())
                    {
                        Some(c) => c.to_string(),
                        None => "clippy::warning".to_string(),
                    };
                    let message_text = match msg.get("message").and_then(|m| m.as_str()) {
                        Some(m) => m.to_string(),
                        None => "Clippy finding".to_string(),
                    };
                    let spans: Vec<Value> = match msg.get("spans").and_then(|s| s.as_array()) {
                        Some(s) => s.clone(),
                        None => Vec::new(),
                    };

                    for span in &spans {
                        let is_primary = span
                            .get("is_primary")
                            .and_then(|v| v.as_bool())
                            .unwrap_or_default();
                        if !is_primary {
                            continue;
                        }
                        let filename = match span.get("file_name").and_then(|f| f.as_str()) {
                            Some(f) if !f.is_empty() => f,
                            _ => continue,
                        };
                        let resolved_file =
                            shared::common::utility_path_normalization::resolve_capabilities_path(
                                match FilePath::new(filename.to_string()) {
                                    Ok(fp) => fp,
                                    Err(_) => path.clone(),
                                },
                                Some(path.clone()),
                            );
                        let line_num = match span.get("line_start").and_then(|v| v.as_u64()) {
                            Some(v) => v as i64,
                            None => 1,
                        };
                        let column_num = match span.get("column_start").and_then(|v| v.as_u64()) {
                            Some(v) => v as i64,
                            None => 1,
                        };
                        let severity = map_clippy_severity(&code, &level);
                        results.push(LintResult {
                            file: resolved_file,
                            line: LineNumber::new(line_num),
                            column: ColumnNumber::new(column_num),
                            code: ErrorCode::raw(code.as_str()),
                            message: LintMessage::new(message_text.as_str()),
                            source: Some(AdapterName::raw("clippy")),
                            severity,
                            enclosing_scope: None,
                            related_locations: LocationList::new(),
                        });
                    }
                }
                Err(_) => continue,
            }
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let working_dir = resolve_cargo_working_dir(path);
        let cmd = vec![
            "cargo".to_string(),
            "clippy".to_string(),
            "--fix".to_string(),
            "--allow-dirty".to_string(),
            "--allow-staged".to_string(),
        ];
        let _ = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir,
                Some(shared::taxonomy_duration_vo::Timeout::new(180.0)),
            )
            .await;
        Ok(ComplianceStatus::new(true))
    }
}

// PURPOSE: RsClippyAdapter — ILinterAdapterProtocol implementation for Clippy linting integration
//
// Executes `cargo clippy --message-format=json` as a subprocess, then parses
// the JSON output line by line. Clippy outputs one JSON object per diagnostic
// message, each containing spans (source locations), severity levels, and
// lint codes.
//
// The adapter handles:
//   - Finding the correct Cargo.toml parent directory
//   - Parsing the JSON stream (filtering for compiler-message reasons)
//   - Resolving relative file paths to absolute across workspaces
//   - Converting Clippy severity levels to AES severity levels
//   - Falling back to stderr if stdout is empty (Clippy sometimes outputs to stderr)
//
// NOTE: apply_fix runs `cargo clippy --fix` which modifies files in place.
// This is the only adapter that supports auto-fix.
use std::path::Path;

impl RustLinterAdapter {
    pub fn new(executor: Arc<dyn ICommandExecutorProtocol>, bin_path: Option<FilePath>) -> Self {
        Self {
            executor,
            _bin_path: bin_path,
        }
    }
}

// (No constructors or helpers found in this file)

// (No constructors or helpers found in this file)

// ─── Block 3: Constructors, Helpers, Private Methods ──────

/// Map Clippy lint group to lint-arwaky severity per FR-004.
///
/// Extracts the lint group from the code string (e.g., `clippy::needless_return` → `style`)
/// and maps it to the appropriate severity. For unknown lints, falls back to the compiler
/// level (`error` → HIGH, `warning` → MEDIUM).
fn map_clippy_severity(code: &str, level: &str) -> Severity {
    let lint_name = code.strip_prefix("clippy::").unwrap_or(code);
    match lint_name {
        // correctness lints → CRITICAL
        "approx_constant"
        | "array_into_iter"
        | "assertions_on_constants"
        | "bool_assert_comparison"
        | "collection_is_never_read"
        | "derive_ord_xor_partial_ord"
        | "drop_non_drop"
        | "duration_subsec"
        | "eq_op"
        | "erasing_numeric_zeros"
        | "forgetting_copy_types"
        | "if_same_then_else"
        | "inconsistent_digit_grouping"
        | "indexing_slicing"
        | "invalid_regex"
        | "manual_assert"
        | "mismatched_target_os"
        | "misrefactored_assign_op"
        | "never_loop"
        | "nonsensical_open_options"
        | "not_unsafe_ptr_arg_deref"
        | "overflow_check_conditional"
        | "panicking_unwrap"
        | "redundant_closure_for_method_calls"
        | "redundant_copy"
        | "redundant_field_names"
        | "search_is_some"
        | "self_named_module_files"
        | "shifting_negative_overflow"
        | "suspicious_assign_format"
        | "suspicious_else_formatting"
        | "suspicious_map"
        | "suspicious_op_assign_impl"
        | "suspicious_operation_groupings"
        | "suspicious_splitn"
        | "suspicious_xor_used_as_pow"
        | "transmute_float_to_int"
        | "transmute_int_to_nonzero"
        | "undocumented_unsafe_blocks"
        | "unconditional_recursion"
        | "unreachable"
        | "unreachable_patterns"
        | "unused_imports"
        | "unused_io_amount"
        | "unused_variables"
        | "zero_divided_by_zero"
        | "zero_literal" => Severity::CRITICAL,
        // suspicious lints → HIGH
        "almost_swapped"
        | "approx_constant"
        | "clone_on_copy"
        | "crosspointer_transmute"
        | "decimal_literal_representation"
        | "deref_addrof"
        | "drop_non_drop"
        | "extend_with_drain"
        | "float_cmp_const"
        | "for_kv_map"
        | "if_let_redundant_pattern_matching"
        | "if_same_then_else"
        | "iter_cloned_collect"
        | "iter_next_slice"
        | "let_unit_value"
        | "lossless_float_literal"
        | "macro_use_imports"
        | "match_on_vec_items"
        | "maybe_misuse_vec"
        | "mem_discriminant"
        | "mem_replaceOptionwithNone"
        | "mem_replace_with_default"
        | "needless_borrow"
        | "needless_late_init"
        | "needless_question_mark"
        | "no_effect_replace"
        | "non_canonical_clone"
        | "non_canonical_partial_ord"
        | "option_env_unwrap"
        | "option_map_unit_fn"
        | "print_stdout"
        | "ptr_arg"
        | "redundant_closure"
        | "redundant_else"
        | "redundant_feature_names"
        | "redundant_slicing"
        | "single_char_pattern"
        | "single_char_lifetime_names"
        | "single_match"
        | "suspicious_arithmetic_impl"
        | "suspicious_assignment_formatting"
        | "suspicious_else_formatting"
        | "suspicious_format"
        | "suspicious_map"
        | "suspicious_op_assign_impl"
        | "suspicious_op_ref"
        | "suspicious_recent_impl"
        | "suspicious_splitn"
        | "suspicious_to_string"
        | "suspicious_xor_used_as_pow"
        | "to_string_in_format"
        | "transmute_float_to_int"
        | "transmute_int_to_char"
        | "transmute_int_to_float"
        | "transmute_int_to_nonzero"
        | "transmute_num_to_bytes"
        | "transmute_ptr_to_ptr"
        | "transmute_reinterpret"
        | "undocumented_unsafe_blocks"
        | "uninlined_format_args"
        | "unit_cmp"
        | "unnecessary_cast"
        | "unnecessary_operation"
        | "unneeded_field_pattern"
        | "unstable_as_mut_slice"
        | "unstable_as_slice"
        | "unused_io_amount"
        | "unused_label" => Severity::HIGH,
        // style lints → MEDIUM
        "bool_to_int_cast"
        | "bool_comparison"
        | "borrowed_box"
        | "bytes_count_to_len"
        | "char_lit_as_byte"
        | "clone_on_ref_ptr"
        | "collapsible_else_if"
        | "collapsible_if"
        | "collapsible_match"
        | "collapsible_str_if"
        | "comparison_chain"
        | "comparison_to_empty"
        | "declare_interior_mutable_const"
        | "default_trait_access"
        | "derivable_impls"
        | "doc_markdown"
        | "enum_variant_names"
        | "exhaustive_enums"
        | "exhaustive_structs"
        | "explicit_iter_loop"
        | "from_over_into"
        | "get_first"
        | "identity_op"
        | "impl_trait_in_params"
        | "implicit_hasher"
        | "init_numbered_const"
        | "into_iter_on_ref"
        | "is_digit_ascii_radix"
        | "iter_skip_next"
        | "large_stack_arrays"
        | "len_without_is_empty"
        | "len_zero"
        | "let_and_return"
        | "manual_ascii_check"
        | "manual_map"
        | "manual_range_contains"
        | "manual_unwrap_or"
        | "match_as_ref"
        | "match_ref_pats"
        | "match_single_binding"
        | "needless_borrowed_reference"
        | "needless_continue"
        | "needless_doctest_main"
        | "needless_pass_by_value"
        | "needless_return"
        | "neg_multiply"
        | "new_without_default"
        | "no_effect"
        | "non_expressive_names"
        | "ok_expect"
        | "op_deref"
        | "option_map_or_none"
        | "or_fun_call"
        | "println_empty_string"
        | "ptr_arg"
        | "question_mark"
        | "redundant_field_names"
        | "redundant_pattern"
        | "redundant_pattern_matching"
        | "ref_option_ref"
        | "regex_simple"
        | "self_named_module_files"
        | "short_lifetimes"
        | "single_char_add_str"
        | "single_component_path_imports"
        | "single_match"
        | "string_add"
        | "string_add_assign"
        | "string_to_string"
        | "struct_field_names"
        | "suspicious_else_formatting"
        | "tabs_in_doc_comments"
        | "to_digit_is_some"
        | "todo"
        | "try_err"
        | "unnecessary_cast"
        | "unnecessary_closure"
        | "unnecessary_closure_to_method_calls"
        | "unnecessary_def_path"
        | "unnecessary_filter_map"
        | "unnecessary_fallible_conversions"
        | "unnecessary_lazy_evaluations"
        | "unnecessary_literal_unwrap"
        | "unnecessary_operation"
        | "unnecessary_unwrap"
        | "unrelated_pattern_in_binding_after_or"
        | "unused_async"
        | "unused_io_amount"
        | "useless_attribute"
        | "useless_format"
        | "useless_vec"
        | "vec_init_then_push"
        | "verbose_bit_mask"
        | "write_with_newline" => Severity::MEDIUM,
        // complexity lints → MEDIUM
        "bool_to_int_with_if"
        | "box_collection"
        | "box_vec"
        | "builtin_type_shadow"
        | "bytes_len_to_count"
        | "clone_on_copy"
        | "collapsible_match"
        | "comparison_chain"
        | "cyclomatic_complexity"
        | "derive_hash_xor_eq"
        | "derive_ord_xor_partial_ord"
        | "derivable_impls"
        | "double_comparison"
        | "double_parens"
        | "duration_subsec"
        | "explicit_counter_loop"
        | "explicit_iter_loop"
        | "filter_map_identity"
        | "filter_map_next"
        | "find_map"
        | "flat_map_identity"
        | "flat_map_option"
        | "get_last_with_len"
        | "get_unwrap"
        | "identity_op"
        | "if_let_else"
        | "if_same_then_else"
        | "implicit_saturating_arithmetic"
        | "inconsistent_digit_grouping"
        | "inefficient_to_string"
        | "into_iter_on_ref"
        | "iter_cloned_collect"
        | "iter_count"
        | "iter_next"
        | "iter_skip_next"
        | "manual_assert"
        | "manual_range"
        | "manual_range_contains"
        | "manual_saturating_arithmetic"
        | "manual_str_add"
        | "map_clone"
        | "map_flatten"
        | "map_unwrap_or"
        | "match_as_ref"
        | "match_ref_pats"
        | "match_single_binding"
        | "needless_borrow"
        | "needless_borrowed_reference"
        | "needless_late_init"
        | "needless_question_mark"
        | "needless_range_loop"
        | "needless_return"
        | "needless_split_string"
        | "needless_update"
        | "no_effect"
        | "nonminimal_bool"
        | "option_as_deref"
        | "option_filter_map"
        | "option_map_or_none"
        | "option_map_unit_fn"
        | "or_fun_call"
        | "range_minus_one"
        | "range_plus_one"
        | "redundant_closure"
        | "redundant_closure_for_method_calls"
        | "redundant_pattern_matching"
        | "redundant_slicing"
        | "redundant_type_annotations"
        | "result_map_or_in_option"
        | "result_unit_err"
        | "search_is_some"
        | "single_char_pattern"
        | "single_match"
        | "skip_collect_next_prev"
        | "suspicious_arithmetic_impl"
        | "to_digit_is_some"
        | "to_string_in_format"
        | "try_err"
        | "type_complexity"
        | "unit_arg"
        | "unnecessary_cast"
        | "unnecessary_closure"
        | "unnecessary_lazy_evaluations"
        | "unnecessary_operation"
        | "unneeded_field_pattern"
        | "zero_divided_by_zero" => Severity::MEDIUM,
        // perf lints → HIGH
        "as_ptr_cast_mut"
        | "bytes_count_to_len"
        | "clone_on_ref_ptr"
        | "collection_is_never_read"
        | "iter_count"
        | "large_stack_arrays"
        | "manual_clamp"
        | "manual_memcpy"
        | "manual_str_repeat"
        | "map_entry"
        | "needless_collect"
        | "needless_pass_by_value"
        | "path_ends_with_ext"
        | "single_char_pattern"
        | "string_extend_chars"
        | "trivial_regex_copy"
        | "unnecessary_to_owned"
        | "unstable_as_mut_slice"
        | "unstable_as_slice"
        | "useless_vec"
        | "vec_init_then_push"
        | "write_with_newline" => Severity::HIGH,
        // pedantic lints → LOW
        "must_use_candidate"
        | "module_name_repetitions"
        | "missing_errors_doc"
        | "missing_panics_doc"
        | "missing_safety_doc"
        | "doc_markdown"
        | "cast_possible_truncation"
        | "cast_possible_wrap"
        | "cast_precision_loss"
        | "cast_sign_loss"
        | "fallible_impl_from"
        | "if_not_else"
        | "inherent_to_string"
        | "items_after_statements"
        | "manual_let_else"
        | "match_same_arms"
        | "needless_pass_by_value"
        | "option_if_let_else"
        | "redundant_else"
        | "redundant_closure_for_method_calls"
        | "semicolon_if_nothing_returned"
        | "single_use_lifetimes"
        | "str_to_string"
        | "string_add_assign"
        | "string_to_string"
        | "suboptimal_flops"
        | "wildcard_enum_match_arm" => Severity::LOW,
        // nursery lints → LOW
        "map_unwrap_or"
        | "mutex_atomic"
        | "needless_borrowed_reference"
        | "option_if_let_else"
        | "recursive_type_alias"
        | "rest_pat_in_fully_bound_structs"
        | "semicolon_if_nothing_returned"
        | "unnecessary_lazy_evaluations"
        | "unused_self" => Severity::LOW,
        // restriction lints → LOW
        | "print_stdout"
        | "print_stderr"
        | "dbg_macro"
        | "exit"
        | "panic"
        | "unwrap_used"
        | "expect_used"
        | "get_unwrap"
        | "indexing_slicing"
        | "unimplemented"
        | "todo"
        | "unreachable"
        | "missing_asserts_for_indexing"
        | "cast_lossless"
        | "checked_conversions"
        | "explicit_read_args"
        | "if_then_some_else_none"
        | "redundant_type_annotations"
        | "string_add"
        | "string_to_string"
        | "try_unwrap"
        | "use_debug"
        | "verbose_file_reads"
        | "missing_inline_in_public_items"
        | "must_use_unit"
        | "empty_enum_variants_with_braces"
        | "enum_glob_use"
        | "disallowed_types"
        | "as_conversions"
        | "decimal_literal_representation"
        | "default_numeric_fallback"
        | "deref_by_slicing"
        | "empty_enum_variants_with_braces"
        | "fallible_impl_from"
        | "filetype_is_file"
        | "from_over_into"
        | "if_then_some_else_none"
        | "impl_trait_in_params"
        | "infinite_iter"
        | "large_include_file"
        | "lossless_float_literal"
        | "mixed_read_write_in_expression"
        | "multiple_inherent_impl"
        | "partial_pub_fields"
        | "pattern_type_mismatch"
        | "pub_use"
        | "rest_pat_in_fully_bound_structs"
        | "same_name_method"
        | "self_named_module_files"
        | "string_add"
        | "string_slice_chars"
        | "suboptimal_flops"
        | "suspicious_xor_used_as_pow"
        | "try_unwrap"
        | "undocumented_unsafe_blocks"
        | "unneeded_field_pattern"
        | "verbose_file_reads" => Severity::LOW,
        // fallback: use compiler level
        _ => {
            if level == "error" {
                Severity::HIGH
            } else {
                Severity::MEDIUM
            }
        }
    }
}
