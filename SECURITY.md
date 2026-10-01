# Security Policy

## Supported Versions

Security fixes are applied to the latest released `3.7.x` line. Older versions
are not maintained — upgrade to the newest release before reporting an issue.

| Version | Supported          |
| ------- | ------------------ |
| 3.7.x   | :white_check_mark: |
| < 3.7   | :x:                |

## Reporting a Vulnerability

Please report suspected vulnerabilities privately — do **not** open a public
issue for security problems.

- Open a [GitHub security advisory](https://github.com/rakaarwaky/lint-arwaky/security/advisories/new)
  for the repository, or contact the maintainer directly.
- Include a description, affected version, reproduction steps, and any relevant
  logs or proof-of-concept.

You can expect an acknowledgement within a few business days. Once a fix is
released, the advisory is published with credit to the reporter unless anonymity
is requested.

## Incident Response

The repository maintainer is responsible for triage, containment, release, and
notification. These are best-effort targets: CRITICAL reports are triaged within
1 business day, WARNING reports within 5 business days, and INFO reports during
the normal release cycle.

If a published release is confirmed vulnerable, the maintainer marks the GitHub
Release as deprecated or pre-release, follows the [DEPLOY.md rollback
plan](DEPLOY.md#rollback-plan), and publishes a patched release as soon as it is
ready. The published GitHub Security Advisory is the primary downstream-user
notification; release notes link to it where disclosure timing permits.

Security investigations should preserve records from the dedicated
`lint_arwaky::audit` tracing target. It records structured security-sensitive
actions and outcomes without source-file contents. Redirect stderr to a
user-protected file when a persistent forensic record is required, for example:

```bash
RUST_LOG='warn,lint_arwaky::audit=info' lint-arwaky-mcp 2>>~/.lint-arwaky-audit.log
```

## Scope and MCP Trust Model

Lint Arwaky runs locally and makes no network calls in its core linting path.
The MCP server communicates only over stdin/stdout (JSON-RPC 2.0). Treat output
from external tools (cargo-audit, ESLint, git remotes, etc.) as untrusted data.

The MCP server deliberately has no application-level authentication: the local
OS process boundary is its authorization boundary. Any process connected to its
stdio is assumed to have the privileges of the user running the server and can
invoke all registered tools, including `fix`, `install-hook`, and
`uninstall-hook`. Nevertheless, every MCP argument is untrusted input and must
be validated, especially paths passed to `execute_command`; the previously
identified path-validation gap demonstrates why a trusted local client does not
make its arguments safe. Do not expose the stdio transport through a shared
container, multi-user runner, network relay, or remote development service
without first adding stronger authentication, authorization, and destructive
operation confirmation. Any future confirmation mechanism must reuse the same
confirmation policy as the TUI rather than creating a second policy.

## Logging and Local Alerts

CLI and MCP general logs default to `warn`. The dedicated
`lint_arwaky::audit` target is enabled at `info` by default and writes structured
security events to stderr. Current debug/trace call sites in the MCP server,
dispatcher, and maintenance crates were audited and do not log source-file
contents. Elevated `RUST_LOG` settings can expose project paths, command
arguments, and external-tool diagnostics; review elevated logs before sharing
them and store them with user-only permissions.

Repeated rejected MCP input is considered suspicious. The alerting policy is:
three or more rejected path-validation attempts in one MCP server session within
one minute should produce a local, user-visible `WARN` event. This is a policy
threshold for future path-validation counters, not telemetry: alerts remain
local and informational. Lint Arwaky sends no logs, anomalies, or usage telemetry
to maintainers or third parties, and provides no automatic anomaly-reporting
mechanism. Users may choose to attach reviewed logs to a private vulnerability
report.

## Credentials Used in CI/CD

Repository workflows use only GitHub's per-run default `GITHUB_TOKEN`. It is
short-lived and automatically rotated by GitHub; no repository-managed personal
access token or third-party secret is used by the current release chain. GitHub
OIDC supplies the short-lived identity used for build-provenance attestations.
The repository maintainer owns review and rotation if a manually managed secret
is introduced, and this section must be updated in the same change.
