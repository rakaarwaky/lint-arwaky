// Verifies the AES501/AES502 engine fix against the REAL workspace tree:
// a cross-crate `use shared::orphan_detector::taxonomy_orphan_result_utility::...`
// must register an inbound link on the REAL leaf module file, not only on its
// domain `mod.rs`. Uses the actual repository files so no temp-dir ambiguity.
use orphan_detector_lint_arwaky::capabilities_orphan_filename_extractor::OrphanFilenameExtractor;
use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use orphan_detector_lint_arwaky::infrastructure_file_cache::OrphanFileCache;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::taxonomy_orphan_contract_vo::OrphanFileListVO;
use std::path::PathBuf;
use std::sync::Arc;

#[test]
fn cross_crate_use_links_to_leaf_module_file() {
    // Repo root is the crate's parent (crates/orphan-detector -> workspace root).
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..");
    let root = root.canonicalize().expect("workspace root");

    let leaf = root
        .join("crates/shared/src/orphan-detector/taxonomy_orphan_result_utility.rs");
    let consumer = root.join(
        "crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs",
    );

    assert!(leaf.exists(), "leaf module must exist: {leaf:?}");
    assert!(consumer.exists(), "consumer must exist: {consumer:?}");

    // The consumer must actually import the leaf module cross-crate.
    let content = std::fs::read_to_string(&consumer).unwrap();
    assert!(
        content.contains("shared::orphan_detector::taxonomy_orphan_result_utility"),
        "consumer must import the leaf module cross-crate"
    );

    let resolver = OrphanGraphResolver::new(
        Arc::new(OrphanFilenameExtractor::new()),
        Arc::new(OrphanFileCache::new()),
    );

    let files = vec![OrphanFileListVO::new(vec![
        leaf.to_string_lossy().to_string(),
        consumer.to_string_lossy().to_string(),
    ])];

    let ctx = resolver.build_graph_context(&files, root.to_str().unwrap());

    let leaf_str = leaf.to_string_lossy().to_string();
    let inbound = ctx.inbound_links.mapping.get(&leaf_str);

    assert!(
        inbound.is_some_and(|v| !v.is_empty()),
        "leaf module file {leaf_str} must have an inbound link from the cross-crate consumer"
    );
}
