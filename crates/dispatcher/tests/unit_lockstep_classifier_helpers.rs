// Lockstep test — pins the mirrored Cargo.toml text-match predicates shared by
// doc-rules and structure-rules against one fixture table, asserting BOTH
// copies on every row.
//
// The peer foundations may not share a module (ARCHITECTURE §8), so
// `exposes_capability_in_cargo_toml` and `has_surface_cargo_deps` exist as
// byte-identical copies: doc-rules' in shared-doc-rules'
// utility_doc_collector, structure-rules' in the structure-rules crate's
// capabilities_pr_la_folder_auditor.
//
// Both copies are `pub` and reachable from here — dispatcher already
// dev-depends on structure-rules (`crates/dispatcher/Cargo.toml:48`), so no
// new dependency edge. The guard is two-way: every fixture row is asserted
// against BOTH copies, so a literal edit to EITHER copy fails CI. The
// fixture verdicts below are hand-derived from the `str::contains`
// semantics both copies share today; they are the ground truth, not a
// re-implementation.

use shared_doc_rules::utility_doc_collector::{
    exposes_capability_in_cargo_toml as doc_exposes, has_surface_cargo_deps as doc_surface_deps,
};
use std::fs;
use std::path::Path;
use structure_rules::capabilities_pr_la_folder_auditor::{
    exposes_capability_in_cargo_toml as structure_exposes,
    has_surface_cargo_deps as structure_surface_deps,
};

/// One fixture folder's expected verdicts, derived from the
/// `str::contains` semantics both peer copies share today.
#[derive(Debug)]
struct Fixture {
    name: &'static str,
    cargo_toml: &'static str,
    expose: bool,
    surface_deps: bool,
}

const FIXTURES: &[Fixture] = &[
    // ── exposes_capability_in_cargo_toml ────────────────────────────────
    // Each literal must be exercised in isolation; a neutral base keeps the
    // other predicate's literals out of the text.
    Fixture {
        name: "capability-literal",
        cargo_toml: "[package]\nname = \"sample-capability\"\n",
        expose: true,
        surface_deps: false,
    },
    // `agent_orchestrator` carries `agent_` as a prefix, so it is a pinned
    // positive. This pins the plain `contains` semantic; a future
    // word-boundary tightening must update this expectation and the
    // structure-rules copy in the same commit.
    Fixture {
        name: "agent-feature-token",
        cargo_toml: "[features]\nagent_orchestrator = []\n",
        expose: true,
        surface_deps: false,
    },
    Fixture {
        name: "agent-literal",
        cargo_toml: "[package]\nname = \"agent_filesystem_orchestrator\"\n",
        expose: true,
        surface_deps: false,
    },
    Fixture {
        name: "orchestrator-literal",
        cargo_toml: "mod agent_core_orchestrator;\n",
        // `orchestrator` alone (no `agent_`/`capability` token) also exposes.
        expose: true,
        surface_deps: false,
    },
    Fixture {
        name: "no-capability-literals",
        cargo_toml: "[package]\nname = \"plain-utils\"\n[dependencies]\nserde = \"1\"\n",
        expose: false,
        surface_deps: false,
    },
    // ── has_surface_cargo_deps ──────────────────────────────────────────
    // Note: `tui` matches no crate name here but is still a literal; the
    // dispatcher literal is exercised by the real dependency spelling.
    Fixture {
        name: "cli-commands-dep",
        cargo_toml: "[dependencies]\nshared-cli-commands.workspace = true\n",
        expose: false,
        surface_deps: true,
    },
    Fixture {
        name: "mcp-server-dep",
        cargo_toml: "[dependencies]\nshared-mcp-server = { workspace = true }\n",
        expose: false,
        surface_deps: true,
    },
    // True-negative for `mcp-server`: the plain `mcp` token does not match
    // the literal, so surface-dep must stay false.
    Fixture {
        name: "plain-mcp-not-a-surface-dep",
        cargo_toml: "[dependencies]\nmcp = \"0.1\"\n",
        expose: false,
        surface_deps: false,
    },
    Fixture {
        name: "tui-dep",
        cargo_toml: "[dependencies]\ntui = \"0.1\"\n",
        expose: false,
        surface_deps: true,
    },
    Fixture {
        name: "dispatcher-dep",
        cargo_toml: "[dependencies]\ndispatcher-lint-arwaky.workspace = true\n",
        expose: false,
        surface_deps: true,
    },
    Fixture {
        name: "no-surface-deps",
        cargo_toml: "[dependencies]\nserde = \"1\"\ntempfile = \"3\"\n",
        expose: false,
        surface_deps: false,
    },
    // True-negatives for each capability literal: dropping the literal's
    // leading letters or shortening it must not expose.
    Fixture {
        name: "aent-file-prefix-not-agent",
        cargo_toml: "[package]\nname = \"aent_filesystem\"\n",
        expose: false,
        surface_deps: false,
    },
    // True-negatives for the capability literals: the plain helpers use
    // `str::contains`, so only a broken literal body (missing letters) stays
    // negative — a mere superstring still fires.
    //
    // `capability` literal — a broken body stays negative.
    Fixture {
        name: "capabilitx-not-capability",
        cargo_toml: "[package]\nname = \"capabilitx\"\n",
        expose: false,
        surface_deps: false,
    },
    // `orchestrator` literal — same story: a broken body stays negative.
    Fixture {
        name: "orchestrtoor-not-orchestrator",
        cargo_toml: "mod orchestrtoor;\n",
        expose: false,
        surface_deps: false,
    },
];

/// Write one fixture folder; both lockstep tests pin against it.
fn write_fixture(name: &str, cargo_toml: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("lockstep-{name}-{:?}", std::process::id()));
    fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("fixture {name}: {e}"));
    fs::write(dir.join("Cargo.toml"), cargo_toml).expect("write fixture Cargo.toml");
    dir
}

#[test]
fn capability_exposure_lockstep_pin() {
    for fixture in FIXTURES {
        let folder = write_fixture(fixture.name, fixture.cargo_toml);
        let doc = doc_exposes(&folder);
        let structure = structure_exposes(&folder);
        assert_eq!(
            doc, fixture.expose,
            "doc-rules exposes_capability_in_cargo_toml diverged on fixture \
             '{fixture:?}' — the capability literals drifted in that copy",
        );
        assert_eq!(
            structure, fixture.expose,
            "structure-rules exposes_capability_in_cargo_toml diverged on \
             fixture '{fixture:?}' — the capability literals drifted in that \
             copy",
        );
        fs::remove_dir_all(&folder).expect("remove fixture");
    }
}

#[test]
fn surface_dependency_lockstep_pin() {
    for fixture in FIXTURES {
        let folder = write_fixture(fixture.name, fixture.cargo_toml);
        let doc = doc_surface_deps(&folder);
        let structure = structure_surface_deps(&folder);
        assert_eq!(
            doc, fixture.surface_deps,
            "doc-rules has_surface_cargo_deps diverged on fixture '{fixture:?}' \
             — the surface-dep literals drifted in that copy",
        );
        assert_eq!(
            structure, fixture.surface_deps,
            "structure-rules has_surface_cargo_deps diverged on fixture \
             '{fixture:?}' — the surface-dep literals drifted in that copy",
        );
        fs::remove_dir_all(&folder).expect("remove fixture");
    }
}

/// A folder with no Cargo.toml at all answers `false` on both predicates in
/// every copy — the read-failure path is part of the mirror too.
#[test]
fn missing_cargo_toml_lockstep_pin() {
    let dir = std::env::temp_dir().join(format!("lockstep-no-cargo-{:?}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    assert!(
        !doc_exposes(&dir),
        "doc-rules copy fired on a missing Cargo.toml"
    );
    assert!(
        !structure_exposes(&dir),
        "structure-rules copy fired on a missing Cargo.toml"
    );
    assert!(
        !doc_surface_deps(&dir),
        "doc-rules copy fired on a missing Cargo.toml"
    );
    assert!(
        !structure_surface_deps(&dir),
        "structure-rules copy fired on a missing Cargo.toml"
    );
    fs::remove_dir_all(&dir).expect("remove fixture");
}

/// The helpers are pure text matches: pass a bare `Path` that does not exist
/// and both copies answer `false` rather than panic.
#[test]
fn nonexistent_folder_lockstep_pin() {
    let absent: &Path = Path::new("/nonexistent/lockstep-fixture");
    assert!(!doc_exposes(absent));
    assert!(!structure_exposes(absent));
    assert!(!doc_surface_deps(absent));
    assert!(!structure_surface_deps(absent));
}
