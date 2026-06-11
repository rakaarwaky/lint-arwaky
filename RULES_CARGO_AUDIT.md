# Cargo-Audit Security Rules

Cargo-audit audits Cargo.lock for crates with security vulnerabilities, available at https://github.com/rustsec/cargo-audit. It checks against the RustSec Advisory Database.

## Key Checks

| Check         | Category  | Description                                      | Severity |
| ------------- | --------- | ------------------------------------------------ | -------- |
| Unmaintained  | Status    | Crate is no longer maintained                    | Medium   |
| Vulnerability | Security  | Known security vulnerability in a dependency     | Critical |
| Notice        | Advisory  | General security advisory notice                 | Low      |
| Unsound       | Soundness | Crate contains unsound code (undefined behavior) | High     |
| Yanked        | Version   | Crate version has been yanked from crates.io     | Medium   |

## Vulnerability Severity Levels

| Level    | CVSS Score | Action                       |
| -------- | ---------- | ---------------------------- |
| Critical | 9.0–10.0   | Immediate update required    |
| High     | 7.0–8.9    | Update as soon as possible   |
| Medium   | 4.0–6.9    | Update in next release cycle |
| Low      | 0.1–3.9    | Monitor for future updates   |

## Common Advisory Types

| Advisory ID       | Pattern                                         | Risk     |
| ----------------- | ----------------------------------------------- | -------- |
| RUSTSEC-2024-xxxx | Memory safety (buffer overflow, use-after-free) | Critical |
| RUSTSEC-2023-xxxx | Cryptographic weakness                          | High     |
| RUSTSEC-2022-xxxx | Code injection / command injection              | Critical |
| RUSTSEC-2021-xxxx | Denial of service                               | Medium   |
| RUSTSEC-2020-xxxx | Data race / thread safety                       | High     |

## Usage

```bash
# Full audit
cargo audit

# Audit with JSON output
cargo audit --json

# Ignore specific advisories
cargo audit --ignore RUSTSEC-2024-0001

# Only show critical and high
cargo audit --severity high
```
