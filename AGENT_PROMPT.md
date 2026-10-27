You are fixing lint-arwaky doc violations in `crates/external-lint` inside Orca worktree at /home/raka/orca/workspaces/lint-arwaky/fix-external-lint, based on commit 9ae91776 (c0026cb3-pr-final tip).

## Goal
Reach 0 violations for `lint-arwaky-cli docs crates/external-lint` in this crate only.

## Your worktree
- Path: /home/raka/orca/workspaces/lint-arwaky/fix-external-lint
- Branch: rakaarwaky/fix-external-lint
- Base: 9ae91776 (c0026cb3-pr-final)
- Violation details: /home/raka/orca/workspaces/lint-arwaky/fix-external-lint/violations.txt (21 violations)
- Binary: /home/raka/orca/workspaces/lint-arwaky/c0026cb3-pr-final/target/debug/lint-arwaky-cli

## Violation types in this crate (21)
- AES606:missing_required_doc
- AES601:backlog_id_not_pr
- AES606:section_not_table
- AES606:table_missing_columns
- AES606:entry_missing_row

## Rules
1. Work ONLY inside crates/external-lint. Do NOT touch other crates, root docs, workspaces-good, workspaces-bad.
2. Do not edit Rust source unless the violation is in a .rs doc comment block.
3. Follow the closed H2 doc templates per doc type.
4. AES601:backlog_id_not_pr — replace cited BACKLOG IDs with IDs the paired FRD/spec actually declares, or add missing IDs to the paired spec table.
5. AES606:section_not_table — replace the body with a table carrying required columns (see WHY/FIX in violations.txt).
6. AES606:table_missing_columns — add missing columns to the table.
7. AES606:entry_missing_row — add missing rows listed in FIX text.
8. Run cargo fmt and cargo clippy after changes if any Rust files changed.
9. Verify: ./target/debug/lint-arwaky-cli docs crates/external-lint → Total: 0 violations.
10. Also run ./target/debug/lint-arwaky-cli check . from repo root to confirm no new violations.
11. Commit all changes in this worktree with conventional commit message.

You have full authority to edit crates/external-lint to reach 0 violations. Do not ask for permission; proceed and commit.