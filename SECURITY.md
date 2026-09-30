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

## Scope

Lint Arwaky runs locally and makes no network calls in its core linting path.
The MCP server communicates only over stdin/stdout (JSON-RPC 2.0). Treat output
from external tools (cargo-audit, ESLint, git remotes, etc.) as untrusted data.
