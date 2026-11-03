// Public Gitleaks test vectors — allowlisted in .gitleaks.toml.
// https://github.com/gitleaks/gitleaks/blob/master/config/gitleaks.toml
// These are the canonical example secrets shipped with Gitleaks itself.
// They must NOT be reported when the good workspace is scanned with its
// .gitleaks.toml.

const AWS_ACCESS_KEY_ID: &str = "AKIAIOSFODNN7EXAMPLE";
const GITHUB_PAT: &str = "ghp_000000000000000000000000000000000000";