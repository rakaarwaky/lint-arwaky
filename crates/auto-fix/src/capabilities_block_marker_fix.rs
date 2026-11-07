// PURPOSE: BlockMarkerFix — FR-AutoFix-005 capability.
//
// Implements IBlockMarkerFixProtocol to rewrite a block-marker banner line
// to canonical form `// ─── Block N: Module <Name> ───` (Rust/TS) or
// `# ─── Block N: Module <Name> ───` (Python). Only name/mismatch banners
// are rewritten — marker count/order violations require manual review.

use shared_auto_fix::contract_fix_protocol::IBlockMarkerFixProtocol;
use shared_auto_fix::taxonomy_auto_fix_constant::NAMES;
use shared_auto_fix::{FailReason, FixOutcome, SkipReason};
use shared_common::LineNumber;
use shared_common::taxonomy_path_vo::FilePath;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Module Declaration ───

pub struct BlockMarkerFix {
    io: Arc<dyn IFileSystemIOProtocol>,
}

impl IBlockMarkerFixProtocol for BlockMarkerFix {
    fn fix_block_marker(&self, file_path: &str, line: LineNumber) -> FixOutcome {
        self.fix_impl(file_path, line, false)
    }

    fn fix_block_marker_dry(&self, file_path: &str, line: LineNumber, dry_run: bool) -> FixOutcome {
        self.fix_impl(file_path, line, dry_run)
    }
}

// ─── Block 3: Module Helpers ───

impl BlockMarkerFix {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { io }
    }

    fn fix_impl(&self, file_path: &str, line: LineNumber, dry_run: bool) -> FixOutcome {
        let fpath = match FilePath::new(file_path.to_string()) {
            Ok(p) => p,
            Err(_) => return FixOutcome::failed(FailReason::FileNotFound),
        };
        let path = Path::new(fpath.value());
        if !self.io.path_exists(path) {
            return FixOutcome::failed(FailReason::FileNotFound);
        }
        let content = match self.io.read_to_string(path) {
            Ok(c) => c.value().to_string(),
            Err(_) => return FixOutcome::failed(FailReason::ReadError),
        };
        let mut lines: Vec<&str> = content.lines().collect();
        let idx = match usize::try_from(line.value)
            .ok()
            .and_then(|i| i.checked_sub(1))
        {
            Some(i) if i < lines.len() => i,
            _ => return FixOutcome::skipped(SkipReason::LineOutOfBounds),
        };
        let trimmed = lines[idx].trim_start();
        let comment_marker = if trimmed.starts_with('#') { "#" } else { "//" };
        let mut after = if comment_marker == "#" {
            trimmed.strip_prefix('#').unwrap_or(trimmed).trim_start()
        } else {
            trimmed.strip_prefix("//").unwrap_or(trimmed).trim_start()
        };
        after = after
            .trim_start_matches(|c| c == '─' || c == '-')
            .trim_start();
        let mut words = after.split_whitespace();
        let Some(_) = words.next() else {
            return FixOutcome::skipped(SkipReason::NoBypassPattern);
        };
        let Some(n_str) = words.next() else {
            return FixOutcome::skipped(SkipReason::NoBypassPattern);
        };
        let n: u32 = match n_str.trim_end_matches(':').parse() {
            Ok(n) if (1..=3).contains(&n) => n,
            _ => return FixOutcome::skipped(SkipReason::NoBypassPattern),
        };
        let Some(name) = after.split(':').nth(1) else {
            return FixOutcome::skipped(SkipReason::NoBypassPattern);
        };
        let name = name
            .trim()
            .strip_prefix("Module ")
            .unwrap_or("")
            .trim_end_matches(|c| c == '─' || c == '-')
            .trim();
        let expected = if let Some(code) = NAMES.get((n as usize) - 1) {
            code.to_string()
        } else {
            return FixOutcome::skipped(SkipReason::NoBypassPattern);
        };
        let indent = &lines[idx][..lines[idx].len() - lines[idx].trim_start().len()];
        let new_line = format!("{} ─── Block {}: Module {} ───", indent, n, expected);
        let new_line = match comment_marker {
            "#" => format!("# {}", new_line.trim_start()),
            _ => format!("// {}", new_line.trim_start()),
        };
        lines[idx] = new_line.as_str();
        let new_content = lines.join("\n");
        if self.io.write_string(path, &new_content).is_err() {
            return FixOutcome::failed(FailReason::WriteError);
        }
        FixOutcome::applied(1)
    }
}
