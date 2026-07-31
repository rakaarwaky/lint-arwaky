// PURPOSE: Acceptance test — FR-001 AST-Based Import Graph Construction.
// Requirement: Build bidirectional import graph from all workspace source files using AST parsing for Rust (syn) and structured line parsing for Python/TS.

use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use shared::orphan_detector::{IOrphanGraphResolverProtocol, OrphanFileListVO};
use std::fs;

#[test]
fn fr001_ast_import_graph_construction_rust_python_ts() {
    let resolver = OrphanGraphResolver::default();
    let dir = tempfile::tempdir().unwrap();
    let root_path = dir.path();

    let crates_dir = root_path.join("crates").join("foo").join("src");
    fs::create_dir_all(&crates_dir).unwrap();

    let rust_file = crates_dir.join("capabilities_user_service.rs");
    fs::write(
        &rust_file,
        "use crate::utility_user_helper;\npub struct UserService;\n",
    )
    .unwrap();

    let py_file = crates_dir.join("capabilities_user_service.py");
    fs::write(
        &py_file,
        "from modules.utility import helper\nclass UserService:\n    pass\n",
    )
    .unwrap();

    let ts_file = crates_dir.join("capabilities_user_service.ts");
    fs::write(
        &ts_file,
        "import { helper } from './utility_helper';\nexport class UserService {}\n",
    )
    .unwrap();

    let files_vo = OrphanFileListVO::new(vec![
        rust_file.to_str().unwrap().to_string(),
        py_file.to_str().unwrap().to_string(),
        ts_file.to_str().unwrap().to_string(),
    ]);

    let context = resolver.build_graph_context(&[files_vo], root_path.to_str().unwrap());

    // Graph context should be populated with node entries without panic
    assert!(
        !context.import_graph.mapping.is_empty() || !context.inbound_links.mapping.is_empty(),
        "FR-001 FAIL: AST graph context must capture files"
    );
}
