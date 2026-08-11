// Unit tests — shared/filesystem taxonomy VOs.
use shared_lint_arwaky::common::taxonomy_language_vo::Language;
use shared_lint_arwaky::filesystem::taxonomy_filesystem_vo::{
    ByteCount, CacheStatsVO, DefinitionEntry, FileEntry, FileExtension, FileMode, FileNodeVO,
    GitCommandResult, GraphAnalysisContext, GraphData, GraphStatsVO, ImplEntry, ImportEdgeVO,
    ImportEntry, ImportGraph, ImportType, InboundLinkMap, InheritanceMap, MemoryBudgetVO,
    ParseMetadata, ParseWarning, ParsedLines, RustMetadata, RustUseItem, ScanConfigVO, ScanStage,
    ScanTiming, ToolName,
};
use std::collections::HashMap;
use std::path::PathBuf;

// ── Tool & Extension VOs ────────────────────────────────────
#[test]
fn tool_name_validates_empty() {
    assert!(ToolName::new("").is_err());
    assert!(ToolName::new("   ").is_err());
    let tool = ToolName::new("eslint").expect("valid");
    assert_eq!(tool.value, "eslint");
}

#[test]
fn file_extension_validates_empty() {
    assert!(FileExtension::new("").is_err());
    let ext = FileExtension::new("rs").expect("valid");
    assert_eq!(ext.value, "rs");
}

// ── FileEntry / ParseMetadata ───────────────────────────────
#[test]
fn file_entry_holds_metadata() {
    let entry = FileEntry {
        path: PathBuf::from("/src/main.rs"),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: 42,
        content: "fn main() {}".to_string(),
        parse_ok: true,
        parse_metadata: Some(ParseMetadata::Rust(RustMetadata::default())),
    };
    assert_eq!(entry.extension, "rs");
    assert!(entry.parse_ok);
    assert!(matches!(entry.parse_metadata, Some(ParseMetadata::Rust(_))));
}

#[test]
fn parse_metadata_unknown_variant() {
    let meta = ParseMetadata::Unknown;
    assert!(matches!(meta, ParseMetadata::Unknown));
}

#[test]
fn rust_use_item_shape() {
    let item = RustUseItem {
        path: "std::collections::HashMap".to_string(),
        is_pub: true,
        is_glob: false,
        names: vec!["HashMap".to_string()],
    };
    assert!(item.is_pub);
    assert_eq!(item.path, "std::collections::HashMap");
}

// ── ImportEntry ─────────────────────────────────────────────
#[test]
fn import_entry_fields() {
    let entry = ImportEntry {
        source_file: PathBuf::from("a.rs"),
        raw_path: "b".to_string(),
        resolved_path: Some(PathBuf::from("b.rs")),
        import_type: ImportType::Use,
        language: Language::Rust,
        is_dynamic: false,
        is_resolved: true,
        symbols: vec!["B".to_string()],
        is_reexport: false,
        is_wildcard: false,
    };
    assert!(entry.is_resolved);
    assert_eq!(entry.import_type, ImportType::Use);
    assert_eq!(entry.language, Language::Rust);
}

// ── Graph VOs ───────────────────────────────────────────────
#[test]
fn import_graph_new_wraps_map() {
    let mut mapping = HashMap::new();
    mapping.insert("a.rs".to_string(), vec!["b.rs".to_string()]);
    let graph = ImportGraph::new(mapping);
    assert_eq!(graph.mapping["a.rs"], vec!["b.rs".to_string()]);
}

#[test]
fn graph_analysis_context_new() {
    let ctx = GraphAnalysisContext::new(
        ImportGraph::new(HashMap::new()),
        InboundLinkMap::new(HashMap::new()),
        InheritanceMap::new(HashMap::new()),
        vec!["a.rs".to_string()],
    );
    assert_eq!(ctx.all_workspace_files, vec!["a.rs".to_string()]);
    assert!(ctx.inheritance_map.mapping.is_empty());
}

#[test]
fn graph_data_default_is_empty() {
    let data = GraphData::default();
    assert!(data.reverse_links.is_empty());
    assert!(data.definitions.is_empty());
    assert!(data.implementations.is_empty());
}

// ── InboundLinkMap::get_importers resolution priorities ─────
fn map_of(pairs: &[(&str, &[&str])]) -> InboundLinkMap {
    let mut mapping = HashMap::new();
    for (k, v) in pairs {
        mapping.insert(
            k.to_string(),
            v.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        );
    }
    InboundLinkMap::new(mapping)
}

#[test]
fn get_importers_exact_match() {
    let map = map_of(&[("a.rs", &["b.rs", "c.rs"])]);
    assert_eq!(map.get_importers("a.rs").map(|v| v.len()), Some(2));
}

#[test]
fn get_importers_prefixed_match_prefers_longer() {
    let map = map_of(&[("a.rs", &["b.rs"]), ("./a.rs", &["b.rs", "c.rs", "d.rs"])]);
    // `./`-prefixed key competes and wins because it is longer.
    assert_eq!(map.get_importers("a.rs").map(|v| v.len()), Some(3));
}

#[test]
fn get_importers_marker_relative_match() {
    let map = map_of(&[("crates/foo/src/a.rs", &["b.rs"])]);
    // Strip the first `/crates/` marker → `crates/foo/src/a.rs` matches.
    assert_eq!(
        map.get_importers("/home/user/proj/crates/foo/src/a.rs")
            .map(|v| v.len()),
        Some(1)
    );
}

#[test]
fn get_importers_middle_dot_competes() {
    // The middle-dot variant is derived from the first `/crates/` marker and
    // competes with the exact match, keeping whichever has the longest list.
    // Build the variant key with the same formula as the implementation so the
    // test pins the semantics (competition, longest wins) rather than the
    // exact string layout.
    let path = "a/crates/b.rs";
    let pos = path.find("/crates/").expect("marker present");
    // Mirror the implementation's middle-dot key: insert `/.` right after the
    // `/crates/` marker position so `a/crates/b.rs` → `a/./crates/b.rs`.
    let middle_dot_key = format!("{}/.{}", &path[..pos], &path[pos..]);

    let map = map_of(&[(path, &["x.rs"])]);
    let map_with_dot = map_of(&[
        (path, &["x.rs"]),
        (middle_dot_key.as_str(), &["y.rs", "z.rs"]),
    ]);
    assert_eq!(map_with_dot.get_importers(path).map(|v| v.len()), Some(2));
    // Without the dot variant, exact match still resolves.
    assert_eq!(map.get_importers(path).map(|v| v.len()), Some(1));
}

#[test]
fn get_importers_clean_path_match() {
    let map = map_of(&[("src/a.rs", &["b.rs"])]);
    assert_eq!(map.get_importers("./src/a.rs").map(|v| v.len()), Some(1));
}

#[test]
fn get_importers_boundary_suffix_match() {
    // Boundary-aligned suffix: `src/b_vo.rs` ends with `/b_vo.rs`, and the
    // separator sits exactly at the suffix boundary.
    let map = map_of(&[("src/b_vo.rs", &["b.rs"])]);
    assert_eq!(map.get_importers("/b_vo.rs").map(|v| v.len()), Some(1));
}

#[test]
fn get_importers_boundary_suffix_reverse_direction() {
    // Reverse direction: the mapping key carries the leading separator and the
    // queried path ends with it.
    let map = map_of(&[("/b_vo.rs", &["b.rs"])]);
    assert_eq!(map.get_importers("src/b_vo.rs").map(|v| v.len()), Some(1));
}

#[test]
fn get_importers_boundary_suffix_does_not_partial_match() {
    // `b_vo.rs` is not a boundary-aligned suffix of `src/ab_vo.rs` — the
    // boundary sits on `b`, not on a path separator.
    let map = map_of(&[("src/ab_vo.rs", &["b.rs"])]);
    assert!(map.get_importers("/b_vo.rs").is_none());
}

#[test]
fn get_importers_missing_returns_none() {
    let map = map_of(&[("a.rs", &["b.rs"])]);
    assert!(map.get_importers("nope.rs").is_none());
}

// ── Stats / config VOs ──────────────────────────────────────
#[test]
fn memory_budget_defaults() {
    let budget = MemoryBudgetVO::default();
    assert_eq!(budget.max_file_cache_bytes, 512 * 1024 * 1024);
    assert_eq!(budget.max_file_size_bytes, 2 * 1024 * 1024);
}

#[test]
fn scan_config_default() {
    let config = ScanConfigVO::default();
    assert!(config.ignored_paths.is_empty());
    assert_eq!(config.budget.max_file_size_bytes, 2 * 1024 * 1024);
}

#[test]
fn cache_and_graph_stats_defaults() {
    let cache = CacheStatsVO::default();
    assert_eq!(
        (cache.cached_count, cache.failed_count, cache.total_bytes),
        (0, 0, 0)
    );
    let graph = GraphStatsVO::default();
    assert_eq!(
        (
            graph.node_count,
            graph.edge_count,
            graph.unresolved_count,
            graph.cycle_count
        ),
        (0, 0, 0, 0)
    );
}

#[test]
fn scan_timing_defaults() {
    let timing = ScanTiming::default();
    assert_eq!(timing.total_ms, 0);
    assert_eq!(timing.walk_ms, 0);
}

#[test]
fn scan_stage_variants() {
    assert_ne!(ScanStage::Walk, ScanStage::Parse);
    assert_ne!(ScanStage::Graph, ScanStage::Extract);
}

// ── Misc contract VOs ───────────────────────────────────────
#[test]
fn byte_count_and_file_mode() {
    assert_eq!(ByteCount::new(1024).bytes, 1024);
    assert_eq!(FileMode::new(0o644).bits, 0o644);
}

#[test]
fn git_command_result_and_parsed_lines() {
    let result = GitCommandResult::new("out".to_string(), "err".to_string(), true);
    assert!(result.success);
    assert_eq!(result.stdout, "out");
    let lines = ParsedLines::new(vec!["a".to_string()]);
    assert_eq!(lines.lines, vec!["a".to_string()]);
}

#[test]
fn parse_warning_message() {
    let warning = ParseWarning {
        file_path: PathBuf::from("x.rs"),
        error_detail: "syntax error".to_string(),
    };
    assert!(warning.message().contains("syntax error"));
}

#[test]
fn graph_vo_shapes() {
    let node = FileNodeVO {
        path: PathBuf::from("a.rs"),
        language: Language::Rust,
        is_external: false,
    };
    assert!(!node.is_external);
    let edge = ImportEdgeVO {
        import_type: ImportType::ReExport,
        raw_path: "b".to_string(),
        resolved: true,
        is_reexport: true,
        is_wildcard: false,
    };
    assert!(edge.is_reexport);
    let def = DefinitionEntry {
        name: "Foo".to_string(),
        file_path: PathBuf::from("a.rs"),
        language: Language::Rust,
    };
    assert_eq!(def.name, "Foo");
    let imp = ImplEntry {
        trait_name: "IFoo".to_string(),
        file_path: PathBuf::from("b.rs"),
        language: Language::Rust,
    };
    assert_eq!(imp.trait_name, "IFoo");
}
