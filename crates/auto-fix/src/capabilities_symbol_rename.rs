// PURPOSE: SymbolRename — FR-AutoFix-003 capability.
//
// Implements ISymbolRenameProtocol to perform mechanical symbol renaming
// with word-boundary-aware replacement. Pure rename logic lives in
// shared::common::utility_word_boundary; this struct owns the
// IFileSystemIOProtocol seam directly.

use shared::auto_fix::contract_fix_protocol::ISymbolRenameProtocol;
use shared::auto_fix::{FailReason, FixOutcome, RUST_KEYWORDS, SkipReason};
use shared::common::taxonomy_name_vo::SymbolName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::{word_boundary_count, word_boundary_replace};
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SymbolRename {
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ──────────────

impl ISymbolRenameProtocol for SymbolRename {
    fn rename_symbol(
        &self,
        file_path: &str,
        old_name: &SymbolName,
        new_name: &SymbolName,
    ) -> FixOutcome {
        self.rename_impl(file_path, old_name.value(), new_name.value(), false)
    }
    fn rename_symbol_dry(
        &self,
        file_path: &str,
        old_name: &str,
        new_name: &str,
        dry_run: bool,
    ) -> FixOutcome {
        self.rename_impl(file_path, old_name, new_name, dry_run)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl SymbolRename {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { io }
    }

    pub(crate) fn rename_impl(
        &self,
        file_path: &str,
        old_name: &str,
        new_name: &str,
        dry_run: bool,
    ) -> FixOutcome {
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

        // Keyword conflict detection
        if RUST_KEYWORDS.contains(&new_name) {
            return FixOutcome::skipped(SkipReason::KeywordConflict);
        }

        if !content.contains(old_name) {
            return FixOutcome::skipped(SkipReason::SymbolNotFound);
        }

        let change_count = word_boundary_count(&content, old_name);

        if change_count == 0 {
            return FixOutcome::skipped(SkipReason::SymbolNotFound);
        }

        if dry_run {
            return FixOutcome::applied(change_count);
        }

        let new_content = word_boundary_replace(&content, old_name, new_name);
        if new_content != content {
            if self
                .io
                .write_string(std::path::Path::new(fpath.value()), &new_content)
                .is_ok()
            {
                return FixOutcome::applied(change_count);
            }
            return FixOutcome::failed(FailReason::WriteError);
        }
        FixOutcome::skipped(SkipReason::AlreadyValid)
    }
}
