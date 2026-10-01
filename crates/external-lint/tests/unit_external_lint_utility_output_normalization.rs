// PURPOSE: Test ANSI normalization of captured external-tool output
// (shared_external_lint::utility_output_normalization).

use shared_external_lint::utility_output_normalization::strip_ansi_escapes;

// ── Uncolored output ──────────────────────────────────────

#[test]
fn plain_output_is_returned_unchanged() {
    let raw = "[warn] src/a.ts\n[warn] Code style issues found.\n";
    assert_eq!(strip_ansi_escapes(raw), raw);
}

// ── Colored output: the CI failure this prevents ───────────
//
// Prettier colorizes its report prefix whenever the environment says it may,
// so on a runner a line reads "[<ESC>[33mwarn<ESC>[39m] src/a.ts". An adapter
// matching the literal "[warn]" then drops every finding and the scan reports
// the tool as clean — a silent pass, not a failure.

#[test]
fn colorized_warn_prefix_becomes_a_plain_prefix() {
    let raw = "Checking formatting...\n[\u{1b}[33mwarn\u{1b}[39m] src/a.ts\n";
    let stripped = strip_ansi_escapes(raw);
    assert!(stripped.contains("[warn] src/a.ts"), "got {stripped:?}");
    assert!(
        !stripped.contains('\u{1b}'),
        "escape survived: {stripped:?}"
    );
}

#[test]
fn colorized_paths_and_columns_are_cleaned_for_regex_matching() {
    // tsc matches a path/line/column pattern anchored at the start of the line.
    let raw = "src/a.ts:12:5 - error TS2345: Argument of type.";
    let colored = format!("\u{1b}[31m{raw}\u{1b}[0m\n");
    assert_eq!(strip_ansi_escapes(&colored), format!("{raw}\n"));
}

#[test]
fn colorized_delimiter_text_is_cleaned_for_substring_matching() {
    // markdownlint splits on " error ", so a colored severity must not break it.
    let raw = "docs/bad.md:1:1 error MD018/no-missing-space-atx No space";
    let colored = format!("\u{1b}[33m{raw}\u{1b}[0m\n");
    let stripped = strip_ansi_escapes(&colored);
    assert!(stripped.contains(" error MD018/"), "got {stripped:?}");
}

// ── Sequence shapes ────────────────────────────────────────

#[test]
fn osc_and_two_character_escapes_are_consumed_whole() {
    // OSC title terminated by BEL, then a bare ESC-7 (save cursor).
    let raw = "\u{1b}]0;title\u{7}[warn] src/a.ts\u{1b}7done";
    assert_eq!(strip_ansi_escapes(raw), "[warn] src/a.tsdone");
}

#[test]
fn osc_terminated_by_string_terminator_is_consumed_whole() {
    // ESC \ (ST) terminates an OSC string; the payload must not leak through.
    let raw = "\u{1b}]0;title\u{1b}\\[warn] src/a.ts";
    assert_eq!(strip_ansi_escapes(raw), "[warn] src/a.ts");
}

#[test]
fn trailing_escape_without_operand_is_dropped() {
    assert_eq!(strip_ansi_escapes("done\u{1b}"), "done");
}
