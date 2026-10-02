// PURPOSE: smoke test for doc-rules — the app boots and answers in under 5s.
//
// One test, one assertion about the aggregate responding at all. The other
// doc-rules suites assert rule behaviour and can take their time; this one is
// the "is it wired and alive" gate a PR runs on every build.
use doc_rules_lint_arwaky::root_doc_rules_container::RootDocRulesContainer;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;
use std::time::{Duration, Instant};

/// The budget this suite holds itself to. The skill fixes smoke at under 5s.
const SMOKE_BUDGET: Duration = Duration::from_secs(5);

#[test]
fn doc_rules_boots_and_answers_a_request_within_the_smoke_budget() {
    let started = Instant::now();
    let aggregate = RootDocRulesContainer::orchestrator();
    // An empty workspace is the cheapest request that still exercises the whole
    // chain: container, orchestrator, document collector, every capability.
    let response = aggregate.execute(DocRequest::audit_all("."));
    // `DocResponse` has one variant, so this binding is exhaustive. It is read
    // rather than discarded so a second variant added later becomes a compile
    // error here instead of a silently unasserted answer.
    let DocResponse::Findings { findings } = response;
    let elapsed = started.elapsed();

    assert!(
        elapsed < SMOKE_BUDGET,
        "a doc audit over the repository took {elapsed:?}, past the {SMOKE_BUDGET:?} \
         smoke budget"
    );
    // The count is not asserted: the smoke test asks "does it answer", and the
    // count belongs to whichever rule the finding came from.
    let _ = findings.len();
}
