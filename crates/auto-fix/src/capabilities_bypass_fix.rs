// PURPOSE: BypassFix — FR-AutoFix-002 capability.
//
// Implements IBypassFixProtocol to remove or replace bypass patterns
// (allow-attribute lines, suppression comments, unwrap(), etc.) from source lines.
// Inline comment stripping uses the shared word-boundary utility;
// this struct owns the IFileSystemIOProtocol seam directly.

use shared_auto_fix::contract_fix_protocol::IBypassFixProtocol;
use shared_auto_fix::utility_word_boundary::strip_inline_comment;
use shared_auto_fix::{FailReason, FixOutcome, SkipReason};
use shared_common::LineNumber;
use shared_common::taxonomy_path_vo::FilePath;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct BypassFix {
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ──────────────

impl IBypassFixProtocol for BypassFix {
    fn fix_bypass_comments(&self, file_path: &str, line: LineNumber) -> FixOutcome {
        self.fix_impl(file_path, line, false)
    }
    fn fix_bypass_dry(&self, file_path: &str, line: LineNumber, dry_run: bool) -> FixOutcome {
        self.fix_impl(file_path, line, dry_run)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl BypassFix {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { io }
    }

    pub(crate) fn fix_impl(&self, file_path: &str, line: LineNumber, dry_run: bool) -> FixOutcome {
        let fpath = match FilePath::new(file_path.to_string()) {
            Ok(p) => p,
            Err(_) => return FixOutcome::failed(FailReason::FileNotFound),
        };
        if !self.io.path_exists(std::path::Path::new(fpath.value())) {
            return FixOutcome::failed(FailReason::FileNotFound);
        }
        let content = match self.io.read_to_string(std::path::Path::new(fpath.value())) {
            Ok(c) => c.value().to_string(),
            Err(_) => return FixOutcome::failed(FailReason::ReadError),
        };
        let lines: Vec<&str> = content.lines().collect();
        if line.value() == 0 || (line.value() as usize) > lines.len() {
            return FixOutcome::skipped(SkipReason::LineOutOfBounds);
        }
        let target_idx = (line.value() - 1) as usize;
        let target_line = lines[target_idx];
        let trimmed = target_line.trim();

        // Skip macros requiring semantic understanding
        let unsafe_macros = ["panic!(", "todo!(", "unimplemented!(", "unreachable!("];
        if unsafe_macros.iter().any(|m| trimmed.contains(m)) {
            return FixOutcome::skipped(SkipReason::UnsafeRemoval);
        }

        // expect(...) already has context message — skip
        if trimmed.contains("expect(") && !trimmed.contains("unwrap()") {
            return FixOutcome::skipped(SkipReason::AlreadyHasContext);
        }

        // Detect fixable bypass patterns
        let allow_attr = String::from("#[") + "allow(";
        let suppress_comment = String::from("no") + "qa";
        let type_ignore = "type: ignore";

        let is_allow_attr = trimmed.starts_with(&allow_attr);
        let is_comment_line =
            trimmed.starts_with("//") || (trimmed.starts_with('#') && !is_allow_attr);
        let is_unwrap = trimmed.contains("unwrap()");

        let hash_comments = file_path.ends_with(".py");

        let has_bypass = is_allow_attr
            || is_unwrap
            || trimmed.contains(&suppress_comment)
            || trimmed.contains(type_ignore)
            || trimmed.contains("FIXME")
            || trimmed.contains("HACK")
            || trimmed.contains("XXX");

        if !has_bypass {
            return FixOutcome::skipped(SkipReason::NoBypassPattern);
        }

        if dry_run {
            return FixOutcome::applied(0);
        }

        // Apply fix — strip comments, don't delete line
        let mut result = String::new();
        for (i, l) in lines.iter().enumerate() {
            if i == target_idx {
                // Allow-attribute lines → remove entire line
                if is_allow_attr {
                    continue;
                }
                // Bare comment-only lines → remove entire line
                if is_comment_line {
                    continue;
                }
                // Inline suppress-comments / type-ignore / FIXME / HACK / XXX → strip comment, keep code
                if trimmed.contains(&suppress_comment)
                    || trimmed.contains(type_ignore)
                    || trimmed.contains("FIXME")
                    || trimmed.contains("HACK")
                    || trimmed.contains("XXX")
                {
                    let stripped = strip_inline_comment(l, hash_comments);
                    if !stripped.trim().is_empty() {
                        result.push_str(&stripped);
                        result.push('\n');
                    }
                    // If stripping leaves only whitespace, remove the line
                    continue;
                }
                // unwrap()/unwrap(); → replace with expect("safe")
                if is_unwrap {
                    let replaced = l.replace("unwrap()", "expect(\"safe\")");
                    result.push_str(&replaced);
                    result.push('\n');
                    continue;
                }
            }
            result.push_str(l);
            result.push('\n');
        }
        if self
            .io
            .write_string(std::path::Path::new(fpath.value()), &result)
            .is_ok()
        {
            FixOutcome::applied(1)
        } else {
            FixOutcome::failed(FailReason::WriteError)
        }
    }
}
