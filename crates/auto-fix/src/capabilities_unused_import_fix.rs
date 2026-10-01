// PURPOSE: UnusedImportFix — FR-AutoFix-001 capability.
//
// Implements IUnusedImportFixProtocol to remove unused import lines
// (use, import, from, require(), = require()) from source files.
// All stateless logic is provided by shared_auto_fix::utility_word_boundary
// helpers; this struct owns the IFileSystemIOProtocol seam directly.

use shared_auto_fix::contract_fix_protocol::IUnusedImportFixProtocol;
use shared_auto_fix::{FailReason, FixOutcome, SkipReason};
use shared_common::LineNumber;
use shared_common::taxonomy_path_vo::FilePath;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct UnusedImportFix {
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ──────────────

impl IUnusedImportFixProtocol for UnusedImportFix {
    fn fix_unused_import(&self, file_path: &str, line: LineNumber) -> FixOutcome {
        self.fix_impl(file_path, line, false)
    }
    fn fix_unused_import_dry(
        &self,
        file_path: &str,
        line: LineNumber,
        dry_run: bool,
    ) -> FixOutcome {
        self.fix_impl(file_path, line, dry_run)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl UnusedImportFix {
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
        let target_line = lines[target_idx].trim();

        // Expand import patterns — JS typically uses `const x = require('foo')`
        let is_import = target_line.starts_with("use ")
            || target_line.starts_with("import ")
            || target_line.starts_with("from ")
            || target_line.starts_with("require(")
            || target_line.contains("= require(");

        if !is_import {
            return FixOutcome::skipped(SkipReason::NotAnImportLine);
        }

        // Multi-line import detection
        // Line has unclosed { → multi-line
        if target_line.contains('{') && !target_line.contains('}') {
            return FixOutcome::skipped(SkipReason::MultiLineImport);
        }
        // Line ends with trailing comma → likely continuation
        if target_line.ends_with(',') {
            if (target_idx + 1) < lines.len() {
                let next_line = lines[target_idx + 1].trim();
                if next_line.starts_with('}')
                    || next_line.is_empty()
                    || next_line.starts_with("use ")
                {
                    return FixOutcome::skipped(SkipReason::MultiLineImport);
                }
            } else {
                return FixOutcome::skipped(SkipReason::MultiLineImport);
            }
        }
        // Previous line has unclosed block → this is a continuation
        if target_idx > 0 {
            let prev_line = lines[target_idx - 1].trim();
            if prev_line.ends_with(',') || (prev_line.contains('{') && !prev_line.contains('}')) {
                return FixOutcome::skipped(SkipReason::MultiLineImport);
            }
        }

        if dry_run {
            return FixOutcome::applied(0);
        }

        // Remove the import line
        let mut result = String::new();
        for (i, l) in lines.iter().enumerate() {
            if i != target_idx {
                result.push_str(l);
                result.push('\n');
            }
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
