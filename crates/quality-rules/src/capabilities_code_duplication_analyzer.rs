// PURPOSE: CodeDuplicationAnalyzer — AES305: detect files with excessive duplication across the codebase
// ALGORITHM (file-level similarity, not per-block):
//   1. Accept pre-fetched (path, content) entries from caller
//   2. For each file, tokenize content into lines; precompute trimmed (raw) and normalized lines
//   3. Slide a window of `min_lines` over lines; hash each window to a u64 key (raw and normalized)
//   4. Two global maps keyed on u64 hash; store file indices
//   5. For each file, compute identical_pct (raw shared) and similar_pct (normalized shared)
//   6. identical_pct > identical_threshold → "identical" violation; else similar_pct > similar_threshold → "similar"
//   7. One violation per duplication group (UX-S5-005): a connected cluster of
//      flagged files sharing windows produces a single finding anchored on the
//      alphabetically first file, naming the other participants. The message
//      (UX-S2-002, UX-S4-001) is a bounded plain-language statement of the
//      duplicated percentage and the threshold, with a bounded list of the top
//      5 duplicated blocks.
//   8. `others` (matching files) resolved at output time only for flagged files,
//      avoiding per-window HashSet cloning and String key allocations.

use shared_common::LintMessage;
use shared_config_system::ArchitectureConfig;
use shared_quality_rules::contract_quality_protocol::ICodeMetricAnalyzerProtocol;
use shared_quality_rules::taxonomy_quality_rules_vo::AesCodeAnalysisViolation;
use shared_quality_rules::utility_code_duplication_detector::{is_import_line, normalize_line};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

// ─── Block 1: Module Declaration ────────────────────────────

pub struct CodeDuplicationAnalyzer {
    config: Arc<ArchitectureConfig>,
}

impl CodeDuplicationAnalyzer {
    /// Build a CodeDuplicationAnalyzer from the shared ArchitectureConfig.
    pub fn from_config(config: Arc<ArchitectureConfig>) -> Self {
        Self { config }
    }
}

// ─── Block 2: Module Contract ────────────────

impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates_entries(
        &self,
        entries: &[(PathBuf, String)],
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        let config = self.config.as_ref();
        let aes305 = config.rules.iter().find(|r| r.name.value == "AES305");
        let min_lines = aes305
            .map(|r| r.code_analysis.min_lines.value as usize)
            .filter(|&v| v > 0)
            .unwrap_or(10);

        // Three-tier fallback: explicit new field → duplication_threshold → built-in default
        let dup = aes305.and_then(|r| r.code_analysis.duplication_threshold);
        let identical_threshold = aes305
            .and_then(|r| r.code_analysis.identical_threshold)
            .or(dup)
            .unwrap_or(75.0);
        let similar_threshold = aes305
            .and_then(|r| r.code_analysis.similar_threshold)
            .or(dup)
            .unwrap_or(75.0);

        // Borrow path/content as &str instead of cloning into (String, String)
        let borrowed: Vec<(&str, &str)> = entries
            .iter()
            .map(|(p, c)| (p.to_str().unwrap_or_default(), c.as_str()))
            .collect();
        self.check_file_similarity_entries(
            &borrowed,
            min_lines,
            identical_threshold,
            similar_threshold,
        )
    }
}

// ─── Block 3: Module Helpers ───────

impl CodeDuplicationAnalyzer {
    /// File-level similarity analysis using pre-read entries.
    /// Optimized: u64 window hashes replace String keys; `others` resolved at
    /// output time only for flagged files (no per-window HashSet cloning).
    fn check_file_similarity_entries(
        &self,
        entries: &[(&str, &str)],
        min_dup_lines: usize,
        identical_threshold_pct: f64,
        similar_threshold_pct: f64,
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        if entries.is_empty() || min_dup_lines == 0 {
            return Vec::new();
        }

        let n = entries.len();

        // Per-file preprocessed lines: raw (trimmed) and normalized.
        let mut file_raw: Vec<Vec<&str>> = Vec::with_capacity(n);
        let mut file_norm: Vec<Vec<String>> = Vec::with_capacity(n);
        for (_, content) in entries {
            let lines: Vec<&str> = content.lines().collect();
            let raw: Vec<&str> = lines.iter().map(|l| l.trim()).collect();
            let norm: Vec<String> = lines.iter().map(|l| normalize_line(l)).collect();
            file_raw.push(raw);
            file_norm.push(norm);
        }

        // Global maps: u64 window hash -> set of file indices sharing that window.
        let mut raw_global: HashMap<u64, HashSet<usize>> = HashMap::with_capacity(n);
        let mut norm_global: HashMap<u64, HashSet<usize>> = HashMap::with_capacity(n);

        // Per-file window records: (start, end, raw_hash, norm_hash) per position.
        let mut file_windows: Vec<Vec<(usize, usize, u64, u64)>> = vec![Vec::new(); n];

        for fi in 0..n {
            let raw = &file_raw[fi];
            let norm = &file_norm[fi];
            let len = raw.len();
            if len < min_dup_lines {
                continue;
            }
            let mut wh: Vec<(usize, usize, u64, u64)> = Vec::new();
            for start in 0..=(len - min_dup_lines) {
                let end = start + min_dup_lines;
                let w = &raw[start..end];
                if is_import_only(w) {
                    continue;
                }
                let raw_hash = hash_window(w.iter().copied());
                let norm_hash = hash_window(norm[start..end].iter().map(|s| s.as_str()));
                raw_global.entry(raw_hash).or_default().insert(fi);
                norm_global.entry(norm_hash).or_default().insert(fi);
                wh.push((start, end, raw_hash, norm_hash));
            }
            file_windows[fi] = wh;
        }

        // Keys (hashes) shared by 2+ files.
        let shared_raw: HashSet<u64> = raw_global
            .iter()
            .filter(|(_, set)| set.len() > 1)
            .map(|(k, _)| *k)
            .collect();
        let shared_norm: HashSet<u64> = norm_global
            .iter()
            .filter(|(_, set)| set.len() > 1)
            .map(|(k, _)| *k)
            .collect();

        // Per-file shared-window counts and positions. `others` is intentionally
        // NOT cloned here — it is resolved at output time only for flagged files.
        let mut raw_shared_counts = vec![0usize; n];
        let mut norm_shared_counts = vec![0usize; n];
        let mut raw_total = vec![0usize; n];
        let mut norm_total = vec![0usize; n];
        let mut raw_positions: Vec<Vec<WindowPos>> = vec![Vec::new(); n];
        let mut norm_positions: Vec<Vec<WindowPos>> = vec![Vec::new(); n];

        for fi in 0..n {
            for (start, end, raw_hash, norm_hash) in &file_windows[fi] {
                raw_total[fi] += 1;
                norm_total[fi] += 1;
                let start_line = start + 1;
                let end_line = *end;
                if shared_norm.contains(norm_hash) {
                    norm_shared_counts[fi] += 1;
                    norm_positions[fi].push(WindowPos {
                        start_line,
                        end_line,
                        raw_hash: *raw_hash,
                        norm_hash: *norm_hash,
                    });
                }
                if shared_raw.contains(raw_hash) {
                    raw_shared_counts[fi] += 1;
                    raw_positions[fi].push(WindowPos {
                        start_line,
                        end_line,
                        raw_hash: *raw_hash,
                        norm_hash: *norm_hash,
                    });
                }
            }
        }

        // Generate violations.
        //
        // UX-S5-005: one finding per duplication group. Flagged files that
        // share at least one window with each other form a cluster; each
        // cluster produces a single finding anchored on the alphabetically
        // first path, naming the other participants.
        const TOP_K: usize = 5;

        // Build per-file flag records (fired tier, percentage, threshold).
        #[derive(Clone, Copy)]
        struct Flag {
            fired: &'static str,
            pct: f64,
            threshold: f64,
        }
        let mut flags: Vec<Option<Flag>> = vec![None; n];
        let mut flagged: Vec<usize> = Vec::new();
        for fi in 0..n {
            if file_raw[fi].len() < min_dup_lines {
                continue;
            }
            let total = norm_total[fi];
            if total == 0 {
                continue;
            }
            let identical_pct = raw_shared_counts[fi] as f64 / total as f64 * 100.0;
            let similar_pct = norm_shared_counts[fi] as f64 / total as f64 * 100.0;

            // Highest-severity applicable classification wins.
            let flag = if identical_pct > identical_threshold_pct {
                Flag {
                    fired: "identical",
                    pct: identical_pct,
                    threshold: identical_threshold_pct,
                }
            } else if similar_pct > similar_threshold_pct {
                Flag {
                    fired: "similar",
                    pct: similar_pct,
                    threshold: similar_threshold_pct,
                }
            } else {
                continue;
            };
            flags[fi] = Some(flag);
            flagged.push(fi);
        }

        // Union-find over flagged files: two flagged files are in the same
        // group when they share at least one (tier-specific) window.
        let mut parent: Vec<usize> = (0..n).collect();
        fn find(v: &mut [usize], x: usize) -> usize {
            let mut root = x;
            while v[root] != root {
                root = v[root];
            }
            // Path compression.
            let mut cur = x;
            while cur != root {
                let next = v[cur];
                v[cur] = root;
                cur = next;
            }
            root
        }
        // For efficiency: for each flagged file, look at its shared windows
        // and union it with every other flagged file that shares at least one
        // of them. One pass over the distinct shared windows is enough to
        // join the whole cluster.
        for &fi in &flagged {
            let fired = flags[fi].unwrap().fired;
            let positions = if fired == "identical" {
                &raw_positions
            } else {
                &norm_positions
            };
            let global_map = if fired == "identical" {
                &raw_global
            } else {
                &norm_global
            };
            for p in &positions[fi] {
                let h = if fired == "identical" {
                    p.raw_hash
                } else {
                    p.norm_hash
                };
                if let Some(file_set) = global_map.get(&h) {
                    for &ofi in file_set {
                        if ofi != fi && flags[ofi].is_some() {
                            let a = find(&mut parent, fi);
                            let b = find(&mut parent, ofi);
                            if a != b {
                                parent[b] = a;
                            }
                        }
                    }
                    // Break after the first shared window: the global map lists
                    // every file that shares that hash, so one match already
                    // reveals the whole cluster for this hash — further
                    // windows only union members that are already connected.
                    break;
                }
            }
        }

        // Group flagged files by root.
        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        for &fi in &flagged {
            groups.entry(find(&mut parent, fi)).or_default().push(fi);
        }

        let mut violations: Vec<(String, AesCodeAnalysisViolation)> = Vec::new();
        for group_files in groups.into_values() {
            // Anchor on the alphabetically first path; others are the rest.
            let mut sorted_files = group_files.clone();
            sorted_files.sort_by(|&a, &b| entries[a].0.cmp(entries[b].0));
            let anchor = sorted_files[0];
            let anchor_flag = flags[anchor].unwrap();
            let anchor_path = entries[anchor].0;

            let others: Vec<String> = sorted_files[1..]
                .iter()
                .map(|&ofi| entries[ofi].0.to_string())
                .collect();
            let others_joined = if others.is_empty() {
                String::new()
            } else {
                others.join(", ")
            };

            // Bounded plain-language message (UX-S2-002, UX-S4-001): the
            // percentage of the file's code that appears elsewhere, the
            // threshold, and a bounded list of the top-5 duplicated blocks.
            let positions = if anchor_flag.fired == "identical" {
                &raw_positions
            } else {
                &norm_positions
            };
            let global_map = if anchor_flag.fired == "identical" {
                &raw_global
            } else {
                &norm_global
            };
            let range_positions: Vec<(usize, usize)> = positions[anchor]
                .iter()
                .map(|p| (p.start_line, p.end_line))
                .collect();
            let ranges = collapse_ranges(&range_positions);

            let mut range_details: Vec<String> = Vec::new();
            for (rs, re) in ranges.iter().take(TOP_K) {
                // Resolve other files at output time, deduped by window hash.
                let mut seen_hashes = HashSet::new();
                let mut others_in_range: HashSet<usize> = HashSet::new();
                for p in positions[anchor]
                    .iter()
                    .filter(|p| p.start_line >= *rs && p.end_line <= *re)
                {
                    let h = if anchor_flag.fired == "identical" {
                        p.raw_hash
                    } else {
                        p.norm_hash
                    };
                    if seen_hashes.insert(h) {
                        if let Some(file_set) = global_map.get(&h) {
                            for &ofi in file_set {
                                if ofi != anchor {
                                    others_in_range.insert(ofi);
                                }
                            }
                        }
                    }
                }
                let mut detail: Vec<String> = Vec::new();
                for &ofi in &others_in_range {
                    // Line range into `ofi`: the intersection of all stored windows
                    // that overlap the current collapsed range.
                    let ofi_windows = &positions[ofi];
                    let overlapping: Vec<_> = ofi_windows
                        .iter()
                        .filter(|w| w.start_line <= *re && w.end_line >= *rs)
                        .collect();
                    if overlapping.is_empty() {
                        continue;
                    }
                    let (os, oe) = overlapping.iter().fold((usize::MAX, 0), |(s, e), w| {
                        (s.min(w.start_line), e.max(w.end_line))
                    });
                    detail.push(format!("{}:lines {os}-{oe}", entries[ofi].0));
                }
                let joined = detail.join(", ");
                range_details.push(format!("lines {rs}-{re} duplicated in {joined}"));
            }
            let extra = ranges.len().saturating_sub(TOP_K);

            let mut msg = format!(
                "AES305 ({}): {}% of this file's code appears in another file (the limit is {}%).",
                anchor_flag.fired, anchor_flag.pct, anchor_flag.threshold
            );
            if !others_joined.is_empty() {
                msg.push_str(&format!(" Also duplicated in {others_joined}."));
            }
            if !range_details.is_empty() {
                msg.push_str(" Blocks: ");
                msg.push_str(&range_details.join("; "));
            }
            if extra > 0 {
                msg.push_str(&format!(" (+{extra} more)"));
            }

            violations.push((
                anchor_path.to_string(),
                AesCodeAnalysisViolation::CodeDuplication {
                    reason: Some(LintMessage::new(msg)),
                },
            ));
        }

        violations
    }
}

/// Per-file window position record: 1-based start/end lines plus the u64 hashes.
#[derive(Clone, Copy)]
struct WindowPos {
    start_line: usize,
    end_line: usize,
    raw_hash: u64,
    norm_hash: u64,
}

/// Hash a window of lines to a u64 key using whole-string SipHash per line
/// (no per-character hashing, no String join allocation).
/// Uses UFCS to avoid unused-import lint on the Hash/Hasher traits.
fn hash_window<'a, I>(lines: I) -> u64
where
    I: Iterator<Item = &'a str>,
{
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for line in lines {
        std::hash::Hash::hash(line, &mut hasher);
    }
    std::hash::Hasher::finish(&hasher)
}

/// Collapse per-window positions into maximal consecutive line ranges.
/// 1-based, inclusive.
///
/// B-6a test seam: `pub` only so cross-crate tests can cover it directly
/// (capability sources must stay free of inline test modules, AES403);
/// production traffic enters through
/// [`ICodeMetricAnalyzerProtocol::handle_duplicates_entries`].
pub fn collapse_ranges(positions: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for &(s, e) in positions {
        if let Some(last) = ranges.last_mut() {
            if s <= last.1 + 1 {
                last.1 = e.max(last.1);
                continue;
            }
        }
        ranges.push((s, e));
    }
    ranges
}

fn is_import_only(w: &[&str]) -> bool {
    w.iter().all(|l| {
        let t = l.trim();
        !t.is_empty() && is_import_line(t)
    })
}
