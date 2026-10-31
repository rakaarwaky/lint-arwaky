# FRD — git-hooks

## Feature Overview

The git-hooks feature sits in the capability layer and is invoked by the CLI surface for pre-commit enforcement, hook installation, hook removal, and configuration bootstrap. It orchestrates git diff inspection and lint pipeline execution over the changed-file set, writing and removing the pre-commit hook file inside the repository's hook directory.

```
[CLI surface] --> [git-hooks orchestrator]
                   |-- diff & lint check  --> [git diff / lint pipeline]
                   |-- install hook       --> [repo hook directory]
                   |-- uninstall hook     --> [repo hook directory]
                   |-- config & identity  --> [config file]
```

## Functional Requirements

### FR-GitHooks-001: Detect Changed Files and Run Lint Check

**Description:** The feature shall detect the changed-file set in the working tree against the repository base branch and run the lint pipeline over that set before a commit is accepted. For every changed file that is lintable, the feature shall evaluate AES rule compliance and return the collected lint findings. A clean changed file shall contribute zero findings, and any file carrying an AES violation shall surface as a blocking result so the commit is refused.

#### Business Logic
- The diff status of any path pair shall be classified as `Unchanged`, `Modified`, `MissingFirst`, `MissingSecond`, `BothMissing`, or `NotAFile`.
- The changed-file list shall be computed against an explicit base branch, falling back to the repository default branch when no base branch is supplied.
- Lint findings shall be produced only for files inside the changed set; unchanged files shall not be linted.
- An empty changed-file set shall yield an empty lint result list without blocking the commit.
- A changed file that carries at least one AES violation shall yield one or more blocking findings.

#### Edge Cases
- A diff side that is not a regular file (`NotAFile`) shall be tolerated and shall not fail the check.
- A missing feature branch shall fall back to the default-branch lookup instead of erroring.
- A path that is not a git repository shall surface a diff detection error to the caller.
- An empty diff (no changed files) shall return an empty lint result list.

#### Input
| No | Detail |
|----|--------|
| 1 | `GitHooksRequest::RunCheck { path }` carrying the repository file path, plus an optional base branch name. |

#### Output
| No | Detail |
|----|--------|
| 1 | `GitHooksResponse::RunCheck` carrying `LintResultList`; empty when no lintable file changed. |

#### Protocol
| No | Detail |
|----|--------|
| 1 | `IDiffDetectionProtocol` |

#### Method
| No | Detail |
|----|--------|
| 1 | `get_diff`, `get_changed_files`, `get_default_branch`, `run_git_diff_check` |

#### Event
| No | Detail |
|----|--------|
| 1 | None; results are returned synchronously to the hook process. |

#### Error
| No | Detail |
|----|--------|
| 1 | `GitHookError` surfaced on the response when the path is not a repository or a file is unreadable; a missing base branch degrades to the default branch. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A changed file carries an AES violation | The pre-commit check runs over the changed set | The response carries at least one blocking finding for that file and the commit is refused |
| All changed files are clean | The pre-commit check runs over the changed set | The response carries an empty lint result list and the commit proceeds |
| The path is not a git repository | The pre-commit check runs | The response carries a `GitHookError` diff detection failure |
| No files are changed against the base branch | The pre-commit check runs | The response carries an empty lint result list |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Blocking accuracy on seeded violation | 100% of seeded violations surface as at least one blocking finding |
| Clean-file false positive rate | 0 blocking findings for a seeded clean changed file |

### FR-GitHooks-002: Install Pre-Commit Hook

**Description:** The feature shall install the pre-commit hook into the repository so that every commit runs the lint check before acceptance. Installation shall create the repository hook directory when it is missing, write the hook file pointing at the supplied executable path, and return the stable manager identity used in the configuration.

#### Business Logic
- The feature shall write the pre-commit hook file into the repository hook directory pointing at the supplied executable path.
- The hook directory shall be created when it does not exist and reused when it does.
- The feature shall return a stable manager identity value across repeated invocations.
- Reinstallation over an existing hook file shall overwrite it cleanly.

#### Edge Cases
- An existing hook directory shall be reused without duplication.
- A missing hook directory shall be created before the hook file is written.
- A repository without a `.git` directory shall fail with `GitHookError` rather than writing outside the repository.

#### Input
| No | Detail |
|----|--------|
| 1 | `GitHooksRequest::Install { executable_path }` carrying the file path of the lint executable. |

#### Output
| No | Detail |
|----|--------|
| 1 | `GitHooksResponse::Install` carrying `SuccessStatus` on success or `GitHookError` on failure. |

#### Protocol
| No | Detail |
|----|--------|
| 1 | `IHookInstallProtocol` |

#### Method
| No | Detail |
|----|--------|
| 1 | `install_pre_commit`, `get_hook_manager_identity` |

#### Event
| No | Detail |
|----|--------|
| 1 | None; installation is a synchronous filesystem write. |

#### Error
| No | Detail |
|----|--------|
| 1 | `GitHookError` when the repository lacks a `.git` directory or the hook file write is denied. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A repository has no hook directory yet | The install action runs | The hook directory is created and the pre-commit hook file exists pointing at the supplied executable path |
| A repository already has a hook file | The install action runs again | The existing hook file is overwritten cleanly and the install reports success |
| The path is not a git repository | The install action runs | The response carries `GitHookError` and no file is written outside the repository |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Install-on-missing-repo success rate | 100% of seeded hook-less repositories gain a pre-commit hook file in one call |
| Reinstall idempotency | 0 residual duplicates after repeated installs on the same repository |

### FR-GitHooks-003: Uninstall Pre-Commit Hook

**Description:** The feature shall uninstall the pre-commit hook and restore the repository to its pre-install state. Uninstallation shall remove the hook file the feature installed and report success; it shall remain idempotent so that repeated or no-op removals still succeed.

#### Business Logic
- The feature shall remove the pre-commit hook file it installed and return a success status.
- Uninstalling when no hook exists shall succeed without error.
- A round trip of install followed by uninstall shall leave the repository hook directory in its original state.

#### Edge Cases
- A user-replaced hook that no longer points at the manager shall be left in place by a safety check rather than deleted.
- A missing `.git` directory shall report `GitHookError` instead of panicking.
- Repeated uninstall calls shall keep succeeding.

#### Input
| No | Detail |
|----|--------|
| 1 | `GitHooksRequest::Uninstall` carried with the repository path in context. |

#### Output
| No | Detail |
|----|--------|
| 1 | `GitHooksResponse::Uninstall` carrying `SuccessStatus` on success or `GitHookError` on failure. |

#### Protocol
| No | Detail |
|----|--------|
| 1 | `IHookUninstallProtocol` |

#### Method
| No | Detail |
|----|--------|
| 1 | `uninstall_pre_commit` |

#### Event
| No | Detail |
|----|--------|
| 1 | None; uninstallation is a synchronous filesystem delete. |

#### Error
| No | Detail |
|----|--------|
| 1 | `GitHookError` when the repository is unusable or the removal is denied. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| A hook was installed by this feature | The uninstall action runs | The hook file is removed and the response carries success |
| No hook exists | The uninstall action runs | The response carries success without modification of any user file |
| A user-replaced hook no longer points at the manager | The uninstall action runs | The replaced hook file is left in place and the response still carries success |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Install-uninstall round trip fidelity | 100% of seeded repositories restore the original hook directory state exactly |
| No-op uninstall success rate | 100% of repeated uninstalls report success |

### FR-GitHooks-004: Initialize Project Config and Manage Ignore Rules

**Description:** The feature shall initialize the git-hooks project configuration and manage its ignore rules. Initialization shall create or read the configuration and describe the outcome; ignore-rule updates shall add or remove a single rule without duplicating it.

#### Business Logic
- Initialization shall create the configuration on first use or read it when it exists, returning a description of the result.
- An ignore-rule update shall add or remove a single ignore rule per invocation and shall not duplicate a rule updated twice.
- The configuration shall identify its manager as the git hook manager.
- A missing configuration file on a read shall be reported as not found rather than silently created unless initialization is the requested flow.

#### Edge Cases
- A configuration file not found on a read shall return a not-found description.
- Updating the same ignored rule twice shall not duplicate the entry.
- Initialization on an existing configuration shall be idempotent.
- Paths outside the repository shall be rejected early.

#### Input
| No | Detail |
|----|--------|
| 1 | A path string, plus an optional `HookIgnoreUpdateVO` carrying the rule and the add-or-remove flag, sourced from `GitHooksRequest::GetManagerIdentity` and the config-init flow. |

#### Output
| No | Detail |
|----|--------|
| 1 | `GitHooksResponse::GetManagerIdentity` carrying the manager identity, plus a description text reporting the config operation result. |

#### Protocol
| No | Detail |
|----|--------|
| 1 | `IConfigInitProtocol` |

#### Method
| No | Detail |
|----|--------|
| 1 | `initialize_config`, `update_ignore_rule` |

#### Event
| No | Detail |
|----|--------|
| 1 | None; configuration writes are synchronous. |

#### Error
| No | Detail |
|----|--------|
| 1 | A missing configuration on a read yields a `config_file_not_found` description; malformed update requests are rejected with a failure description. |

#### Test
| Context | When Action | Then Observable Result |
|---------|-------------|------------------------|
| No configuration exists | The config-init flow runs | The configuration is created and the description reports initialization |
| The configuration is missing on a read | A read runs without initialization | The response reports `config_file_not_found` |
| The same ignore rule is updated twice | Both updates run | The configuration holds exactly one entry for that rule |
| The manager identity is requested | The identity flow runs | The readback identity matches the git hook manager |

#### Metric
| Target Metric | Value |
|---------------|-------|
| Config round trip fidelity | 100% of initialize-then-update sequences leave the configuration with exactly the requested ignore rules |
| Identity readback accuracy | 100% of identity reads return the git hook manager value |

## Public Interface

| Aspect | Detail |
|--------|--------|
| Agent | `GitHooksOrchestrator` |
| Aggregate | `IGitHooksAggregate` |
| Method | `execute` |
| Request | `GitHooksRequest` (`RunCheck`, `Install`, `Uninstall`, `GetManagerIdentity`) |
| Response | `GitHooksResponse` (`RunCheck`, `Install`, `Uninstall`, `GetManagerIdentity`) |

## Reference

| Artifact | Purpose | Link |
|----------|---------|------|
| PRD | Business goals, user stories, and scope | [PRD.md](../../PRD.md) |
| Backlog | Task tracking, epics, and implementation status | [BACKLOG.md](./BACKLOG.md) |
| Roadmap | Timeline, phasing, and dependencies | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries, tech stack, and layer contract | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
