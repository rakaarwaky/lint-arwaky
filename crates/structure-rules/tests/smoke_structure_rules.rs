// PURPOSE: smoke test for structure-rules — the app boots and answers in under
// 5s.
//
// One test, one assertion about the aggregate responding at all. The other
// structure-rules suites assert rule behaviour and can take their time; this one
// is the "is it wired and alive" gate a PR runs on every build.
use shared_structure_rules::taxonomy_structure_rules_request::StructureRequest;
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;
use std::time::{Duration, Instant};
use structure_rules_lint_arwaky::root_structure_rules_container::RootStructureRulesContainer;

/// The budget this suite holds itself to. The skill fixes smoke at under 5s.
const SMOKE_BUDGET: Duration = Duration::from_secs(5);

#[test]
fn structure_rules_boots_and_answers_a_request_within_the_smoke_budget() {
    let started = Instant::now();
    let aggregate = RootStructureRulesContainer::orchestrator();
    // The repository root is the real target a `scan .` would audit, so this
    // measures the cost a user actually pays rather than an empty workspace.
    let response = aggregate.execute(StructureRequest::audit_all("."));

    // `StructureResponse` has one variant, so this binding is exhaustive. It is
    // read rather than discarded so a second variant added later becomes a
    // compile error here instead of a silently unasserted answer.
    let StructureResponse::Findings { findings } = response;
    let elapsed = started.elapsed();

    assert!(
        elapsed < SMOKE_BUDGET,
        "a structure audit over the repository took {elapsed:?}, past the \
         {SMOKE_BUDGET:?} smoke budget"
    );
    // The count is not asserted: the smoke test asks "does it answer", and the
    // count belongs to whichever rule the finding came from.
    let _ = findings.len();
}
