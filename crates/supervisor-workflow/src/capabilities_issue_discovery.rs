// PURPOSE: Mock GitHub issue discovery + pure "least busy" selection logic.
//
// `MockIssueDiscovery` implements `IIssueDiscoveryProtocol` with clearly
// SYNTHETIC, hardcoded issue data (no network, per plan risk #1). The
// numbers are intentionally out of range of any real `lint-arwaky` issue or
// PR number (verified 2026-10-10: the repo has 0 open issues; the highest
// real identifier in use is in the 1050s), so a real reqwest-backed
// implementation swapping in later can never be confused with this mock
// dataset — and a live API returning zero open issues (the actual current
// state of `rakaarwaky/lint-arwaky`) falls back to this same synthetic set,
// which is the "use mock when live API unavailable" path from the plan.
// `IssueSelectionLogic` is pure and I/O-free so it is directly unit-testable.

use shared_common::taxonomy_common_vo::Count;
use shared_supervisor::contract_supervisor_protocol::IIssueDiscoveryProtocol;
use shared_supervisor::taxonomy_supervisor_vo::GitHubIssueVo;
use shared_supervisor::taxonomy_supervisor_vo::SupervisorError;

// ─── Block 1: Struct Definition ───────────────────────────

/// Hardcoded synthetic open issues, clearly out of range of any real
/// `rakaarwaky/lint-arwaky` identifier (numbers in the 9000s are reserved
/// for this mock so they can never collide with a real PR/issue number).
///
/// `static` + `LazyLock`: the entries allocate `String`/`Vec` (not valid in
/// a `const` expression), so initialization happens lazily on first access —
/// still zero I/O, still deterministic.
static MOCK_ISSUES: std::sync::LazyLock<Vec<GitHubIssueVo>> = std::sync::LazyLock::new(|| {
    vec![
        GitHubIssueVo {
            number: 9001,
            title: "[SYNTHETIC-MOCK] Supervisor cycle report omits per-issue CI poll counts".to_string(),
            body: "SYNTHETIC MOCK DATA — not a real GitHub issue. In a live run this \
                    slot would carry a real open issue. SupervisorCycleReport::outcomes \
                    should also record how many times check_ci_status was polled before \
                    settling."
                .to_string(),
            state: "open".to_string(),
            comments_count: 2,
            reactions_total: 1,
            user_login: "mock-user".to_string(),
            created_at: "2026-09-28T10:12:00Z".to_string(),
            updated_at: "2026-10-02T08:00:00Z".to_string(),
            html_url: "https://github.com/rakaarwaky/lint-arwaky/issues/9001".to_string(),
            labels: vec!["enhancement".to_string(), "good first issue".to_string()],
        },
        GitHubIssueVo {
            number: 9002,
            title: "[SYNTHETIC-MOCK] clippy: collapsible_if false-positive on nested match arms".to_string(),
            body: "SYNTHETIC MOCK DATA — not a real GitHub issue. collapsible_if fires \
                    when the inner arm is a pattern guard, not a bare if; the suggestion \
                    rewrites to an invalid match."
                .to_string(),
            state: "open".to_string(),
            comments_count: 9,
            reactions_total: 4,
            user_login: "mock-user".to_string(),
            created_at: "2026-09-12T15:40:00Z".to_string(),
            updated_at: "2026-10-08T20:15:00Z".to_string(),
            html_url: "https://github.com/rakaarwaky/lint-arwaky/issues/9002".to_string(),
            labels: vec!["bug".to_string(), "clippy".to_string()],
        },
        GitHubIssueVo {
            number: 9003,
            title: "[SYNTHETIC-MOCK] AES704 test-suite coverage miscounts .ts files under packages/".to_string(),
            body: "SYNTHETIC MOCK DATA — not a real GitHub issue. The seven-prefix walk \
                    stops at the first language boundary; TypeScript suites under \
                    packages/ are never visited, so the category check silently passes."
                .to_string(),
            state: "open".to_string(),
            comments_count: 4,
            reactions_total: 2,
            user_login: "mock-user".to_string(),
            created_at: "2026-08-30T09:22:00Z".to_string(),
            updated_at: "2026-09-19T11:05:00Z".to_string(),
            html_url: "https://github.com/rakaarwaky/lint-arwaky/issues/9003".to_string(),
            labels: vec!["bug".to_string(), "aes704".to_string()],
        },
        GitHubIssueVo {
            number: 9004,
            title: "[SYNTHETIC-MOCK] Document `docs` command exit codes in the PRD".to_string(),
            body: "SYNTHETIC MOCK DATA — not a real GitHub issue. The exit-code contract \
                    table in the PRD predates the docs audit pass; exit 4 (docs drift) \
                    is never documented."
                .to_string(),
            state: "open".to_string(),
            comments_count: 0,
            reactions_total: 0,
            user_login: "mock-user".to_string(),
            created_at: "2026-08-14T13:00:00Z".to_string(),
            updated_at: "2026-08-14T13:00:00Z".to_string(),
            html_url: "https://github.com/rakaarwaky/lint-arwaky/issues/9004".to_string(),
            labels: vec!["docs".to_string(), "good first issue".to_string()],
        },
        GitHubIssueVo {
            number: 9005,
            title: "[SYNTHETIC-MOCK] External-lint: ruff adapter ignores per-language pyproject overrides".to_string(),
            body: "SYNTHETIC MOCK DATA — not a real GitHub issue. The adapter resolves \
                    the ruff binary once per run; per-file pyproject.toml overrides \
                    under nested packages are never re-read, so monorepo Ruff profiles \
                    are silently skipped."
                .to_string(),
            state: "open".to_string(),
            comments_count: 6,
            reactions_total: 3,
            user_login: "mock-user".to_string(),
            created_at: "2026-07-22T18:31:00Z".to_string(),
            updated_at: "2026-09-05T07:44:00Z".to_string(),
            html_url: "https://github.com/rakaarwaky/lint-arwaky/issues/9005".to_string(),
            labels: vec!["enhancement".to_string(), "ruff".to_string()],
        },
    ]
});

/// Mock `IIssueDiscoveryProtocol`: returns the first `max` entries of the
/// hardcoded list, no network, no I/O.
#[derive(Debug, Default, Clone, Copy)]
pub struct MockIssueDiscovery;

/// Pure selection policy: rank issues by engagement (`comments_count +
/// reactions_total`), ascending, and keep at most `max_issues`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IssueSelectionLogic {
    pub max_issues: usize,
}

// ─── Block 2: Protocol Trait Implementation ──────────────

impl IIssueDiscoveryProtocol for MockIssueDiscovery {
    fn discover_issues(&self, max: Count) -> Result<Vec<GitHubIssueVo>, SupervisorError> {
        let limit = usize::try_from(max.value()).unwrap_or(usize::MAX);
        Ok(MOCK_ISSUES.iter().take(limit).cloned().collect())
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ───────────

impl MockIssueDiscovery {
    /// Construct the stateless mock.
    pub fn new() -> Self {
        Self
    }
}

impl IssueSelectionLogic {
    /// Pick the least-busy issues from `issues`.
    ///
    /// Sorting is stable so ties keep their original relative order; the
    /// result is capped at `max_issues`.
    pub fn select_least_busy(&self, issues: &[GitHubIssueVo]) -> Vec<GitHubIssueVo> {
        let mut ranked: Vec<&GitHubIssueVo> = issues.iter().collect();
        ranked.sort_by_key(|i| i.busy_score());
        ranked.into_iter().take(self.max_issues).cloned().collect()
    }
}
