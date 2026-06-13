# BRIEFING — 2026-06-13T13:02:00+07:00

## Mission
Orchestrate and execute production readiness fixes in the `lint-arwaky` workspace.

## 🔒 My Identity
- Archetype: Project Orchestrator
- Roles: orchestrator, user_liaison, human_reporter, successor
- Working directory: /home/raka/mcp-arwaky/lint-arwaky/.agents/orchestrator/
- Original parent: main agent
- Original parent conversation ID: 6ce300d9-f8de-4f09-83cb-55524349710f

## 🔒 My Workflow
- **Pattern**: Project
- **Scope document**: /home/raka/mcp-arwaky/lint-arwaky/.agents/orchestrator/PROJECT.md
1. **Decompose**: Decompose production readiness fixes into individual milestones matching the tasks in the plan.
2. **Dispatch & Execute** (pick ONE):
   - **Delegate (sub-orchestrator)**: For each milestone, spawn a sub-orchestrator (or a worker/explorer/reviewer loop) to implement the fix and verify.
3. **On failure** (in this order):
   - Retry: nudge stuck agent or re-send task
   - Replace: spawn fresh agent with partial progress
   - Skip: proceed without (only if non-critical)
   - Redistribute: split stuck agent's remaining work
   - Redesign: re-partition decomposition
   - Escalate: report to parent (sub-orchestrators only, last resort)
4. **Succession**: Succession at 16 spawns, write handoff.md, spawn successor.
- **Work items**:
  1. Cycle Detection todo!() fix [pending]
  2. SourceParserOrchestrator default todo!() fix [pending]
  3. PlaceholderAnalyzer panic!() fix [pending]
  4. Mutex unwrap() fix [pending]
  5. Duplicate metrics provider removal [pending]
  6. Workspace version synchronization [pending]
  7. PluginCommandsOrchestrator real logic [pending]
  8. ReportCommandsOrchestrator real logic [pending]
  9. MCP server println!() cleanup [pending]
  10. CLI setup hardcoded path fix [pending]
  11. Test data removal from production scanner [pending]
  12. Orphan surfaces analyzer TODO fix [pending]
- **Current phase**: 1
- **Current focus**: Milestone decomposition and PROJECT.md creation

## 🔒 Key Constraints
- Never write, modify, or create source code files directly.
- Never run build/test commands yourself — require workers to do so.
- Never reuse a subagent after it has delivered its handoff.
- Only use file-editing tools for metadata/state files (.md) in .agents/ folder.

## Current Parent
- Conversation ID: 6ce300d9-f8de-4f09-83cb-55524349710f
- Updated: not yet

## Key Decisions Made
- Decompose the fixes into 12 milestones corresponding to the 12 tasks in the implementation plan.

## Team Roster
| Agent | Type | Work Item | Status | Conv ID |
|-------|------|-----------|--------|---------|

## Succession Status
- Succession required: no
- Spawn count: 0 / 16
- Pending subagents: none
- Predecessor: none
- Successor: not yet spawned

## Active Timers
- Heartbeat cron: dcfa2088-4c78-4426-986c-9a263f1ea86b/task-9
- Safety timer: none
- On succession: kill all timers before spawning successor
- On context truncation: run manage_task(Action="list") — re-create if missing

## Artifact Index
- /home/raka/mcp-arwaky/lint-arwaky/.agents/orchestrator/PROJECT.md — Global project plan & milestone tracker
- /home/raka/mcp-arwaky/lint-arwaky/.agents/orchestrator/progress.md — Step-by-step progress tracking
- /home/raka/mcp-arwaky/lint-arwaky/.agents/orchestrator/ORIGINAL_REQUEST.md — Verbatim user request record
