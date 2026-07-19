// Verifies the AES501/AES502 engine fix: cross-crate `use` of a leaf module
// file must register an inbound link on the REAL leaf module file, not only on
// its domain `mod.rs`. This is the test that was previously impossible because
// `resolve_module_file` lived entirely in the resolver.
use orphan_detector_lint_arwaky::capabilities_orphan_filename_extractor::OrphanFilenameExtractor;
use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use orphan_detector_lint_arwaky::infrastructure_file_cache::OrphanFileCache;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::taxonomy_orphan_contract_vo::OrphanFileListVO;
use std::path::Path;
use std::sync::Arc;

fn write(path: &Path, content: &str) {
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p).unwrap();
    }
    std::fs::write(path, content).unwrap();
}

#[test]
fn cross_crate_use_links_to_leaf_module_file() {
    let base = std::env::temp_dir().join("arwaky_leaf_link_test");
    let _ = std::fs::remove_dir_all(&base);

    let shared_src = base.join("crates/shared/src");
    // The consumed leaf module (exactly the kind false-flagged as AES501).
    write(
        &shared_src.join("orphan-detector/taxonomy_orphan_result_utility.rs"),
        "pub fn mk_orphan_result() {}\n",
    );
    write(
        &shared_src.join("orphan-detector/mod.rs"),
        "pub mod taxonomy_orphan_result_utility;\n",
    );
    // A consumer in a DIFFERENT crate that imports the leaf module.
    let consumer = base.join("crates/orphan-detector/src/capabilities_orphan_surfaces_analyzer.rs");
    write(
        &consumer,
        "use shared::orphan_detector::taxonomy_orphan_result_utility::mk_orphan_result;\n\
         pub fn run() { let _ = mk_orphan_result(); }\n",
    );

    let resolver = OrphanGraphResolver::new(
        Arc::new(OrphanFilenameExtractor::new()),
        Arc::new(OrphanFileCache::new()),
    );

    let files = vec![OrphanFileListVO::new(vec![
        shared_src
            .join("orphan-detector/taxonomy_orphan_result_utility.rs")
            .to_string_lossy()
            .to_string(),
        shared_src
            .join("orphan-detector/mod.rs")
            .to_string_lossy()
            .to_string(),
        consumer.to_string_lossy().to_string(),
    ])];

    let ctx = resolver.build_graph_context(&files, base.to_str().unwrap());

    let leaf = shared_src
        .join("orphan-detector/taxonomy_orphan_result_utility.rs")
        .to_string_lossy()
        .to_string();
    let inbound = ctx.inbound.mapping.get(&leaf);

    assert!(
        inbound.is_some_and(|v| !v.is_empty()),
        "leaf module file {leaf} must have an inbound link from the cross-crate consumer"
    );
    let _ = std::fs::remove_dir_all(&base);
}
