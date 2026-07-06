# MiMo Code Documentation

> Official documentation scraped from https://mimo.xiaomi.com/mimocode/start
> Generated: 2026-06-30

## Table of Contents

1. [Overview](#overview)
2. [Install & Launch](#install--launch)
3. [Models](#models)
4. [First Prompt](#first-prompt)
5. [Interaction & Input](#interaction--input)
6. [Sessions & Context](#sessions--context)
7. [Modes](#modes)
8. [Tools](#tools)
9. [Config Files](#config-files)
10. [Permissions](#permissions)
11. [Agent Skills](#agent-skills)
12. [Agents](#agents)
13. [Slash Commands](#slash-commands)
14. [CLI Options](#cli-options)
15. [Keybinds](#keybinds)
16. [MCP Servers](#mcp-servers)
17. [Rules](#rules)

---

## Overview

**MiMo Code** is an AI coding agent for developers. It understands your codebase, plans changes, edits code safely, and works across the tools your team already uses — with the same experience in the terminal, the desktop app, and IDE extensions.

### Get Started

- [Install & Launch](#install--launch) — From install to making your first real change in the terminal.
- [Connect Models](#models) — Configure API keys and connect the LLM providers you want to use.
- [First Prompt](#first-prompt) — Your first interaction with MiMo Code.

### What You Can Do

- [Understand your code](#interaction--input) — Map out a repository's structure and find the right place to start.
- [Plan and build features](#modes) — Scope changes and use Plan mode to ship larger tasks with confidence.
- [Review changes](#permissions) — Inspect diffs, run checks, and catch issues before you merge.
- [Customize MiMo Code](#config-files) — Use rules, skills, and commands that match how your team works.
- [Use different agents](#agents) — Configure specialized agents with custom prompts, models, and tools.
- [Debug and fix](#troubleshooting) — Reproduce problems, pinpoint root causes, and verify your fixes.

---

## Install & Launch

To use MiMo Code in your terminal, set up your environment, then install it with any of the methods below.

### Prerequisites

To use MiMo Code in your terminal, you'll need:

1. A modern terminal emulator like:
   - [WezTerm](https://wezterm.org), cross-platform
   - [Alacritty](https://alacritty.org), cross-platform
   - [Ghostty](https://ghostty.org), Linux and macOS
   - [Kitty](https://sw.kovidgoyal.net/kitty/), Linux and macOS
2. API keys for the LLM providers you want to use.

### Install

The easiest way to install MiMo Code is through the install script.

**Recommended for Mac/Linux** (for the best experience, Mac users are strongly encouraged to use iTerm or the VSCode Terminal):

```bash
curl -fsSL https://mimo.xiaomi.com/install | bash
```

**Recommended for Windows:**

```bash
npm install -g @mimo-ai/cli
```

Once installed, [connect a model provider](#models), then [run your first prompt](#first-prompt).

---

## Models

MiMo Code uses the [AI SDK](https://ai-sdk.dev/) and [Models.dev](https://models.dev) to support **75+ LLM providers** and it supports running local models.

### Connect a provider

With MiMo Code you can use any LLM provider by configuring their API keys.

If you are new to using LLM providers, we recommend using [MiMo Token Plan](https://platform.xiaomimimo.com/token-plan). It's a curated list of models that have been tested and verified by the MiMo Code team.

1. Run the `/connect` command in the TUI, select Xiaomi MiMo, and head to [platform.xiaomimimo.com](https://platform.xiaomimimo.com/).

   ```txt
   /connect
   ```

2. Sign in, add your billing details, and copy your API key.

3. Paste your API key.

   ```txt
   ┌ API key
   │
   │
   └ enter
   ```

Alternatively, you can select one of the other providers.

### Providers

Most popular providers are preloaded by default. If you've added the credentials for a provider through the `/connect` command, they'll be available when you start MiMo Code.

### Select a model

Once you've configured your provider you can select the model you want by typing in:

```bash
/models
```

### MiMo Platform (recommended)

The fastest way to get started is the [MiMo Platform](https://platform.xiaomimimo.com/). It is OpenAI-compatible and authenticates with the `api-key` HTTP header. Drop the following into your `mimocode.jsonc`:

```jsonc
{
  "provider": {
    "mimo": {
      "npm": "@ai-sdk/openai-compatible",
      "name": "MiMo",
      "options": {
        "baseURL": "https://api.xiaomimimo.com/v1",
        "headers": { "api-key": "{env:MIMO_API_KEY}" },
      },
      "models": {
        "mimo-v2.5-pro": { "name": "MiMo V2.5 Pro" },
      },
    },
  },
  "model": "mimo/mimo-v2.5-pro",
}
```

The flagship model is `mimo-v2.5-pro`. Reasoning output is returned as `reasoning_content`. An Anthropic-compatible endpoint is also available at `https://api.xiaomimimo.com/anthropic`.

### Loading models

When MiMo Code starts up, it checks for models in the following priority order:

1. The `--model` or `-m` command line flag. The format is the same as in the config file: `provider_id/model_id`.
2. The model list in the MiMo Code config.

   ```json
   {
     "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
     "model": "mimo/mimo-v2.5-pro"
   }
   ```

   The format here is `provider/model`.

3. The last used model.
4. The first model using an internal priority.

---

## First Prompt

Once you've configured a provider, initialize MiMo Code in your project and start asking it questions.

### Initialize

Navigate to a project that you want to work on.

```bash
cd /path/to/project
```

And run MiMo Code.

```bash
mimo
```

Next, initialize MiMo Code for the project by running the following command.

```bash
/init
```

This will get MiMo Code to analyze your project and create an `AGENTS.md` file in the project root.

> **TIP**: You should commit your project's `AGENTS.md` file to Git. This helps MiMo Code understand the project structure and the coding patterns used.

### Usage

You are now ready to use MiMo Code to work on your project. Feel free to ask it anything!

#### Ask questions

You can ask MiMo Code to explain the codebase to you.

> **TIP**: Use the `@` key to fuzzy search for files in the project.

```txt
How is authentication handled in @packages/functions/src/api/index.ts
```

This is helpful if there's a part of the codebase that you didn't work on.

#### Add features

You can ask MiMo Code to add new features to your project. Though we first recommend asking it to create a plan.

1. **Create a plan**

   MiMo Code has a _Plan mode_ that disables its ability to make changes and instead suggest _how_ it'll implement the feature.

   Switch to it using the **Tab** key. You'll see an indicator for this in the lower right corner.

   ```bash
   <TAB>
   ```

   Now let's describe what we want it to do.

   ```txt
   When a user deletes a note, we'd like to flag it as deleted in the database.
   Then create a screen that shows all the recently deleted notes.
   From this screen, the user can undelete a note or permanently delete it.
   ```

   You want to give MiMo Code enough details to understand what you want. It helps to talk to it like you are talking to a junior developer on your team.

   > **TIP**: Give MiMo Code plenty of context and examples to help it understand what you want.

2. **Iterate on the plan**

   Once it gives you a plan, you can give it feedback or add more details.

   ```txt
   We'd like to design this new screen using a design I've used before.
   [Image #1] Take a look at this image and use it as a reference.
   ```

   > **TIP**: Drag and drop images into the terminal to add them to the prompt. MiMo Code can scan any images you give it and add them to the prompt.

3. **Build the feature**

   Once you feel comfortable with the plan, switch back to _Build mode_ by hitting the **Tab** key again.

   ```bash
   <TAB>
   ```

   And asking it to make the changes.

   ```bash
   Sounds good! Go ahead and make the changes.
   ```

#### Make changes

For more straightforward changes, you can ask MiMo Code to directly build it without having to review the plan first.

```txt
We need to add authentication to the /settings route. Take a look at how this is
handled in the /notes route in @packages/functions/src/notes.ts and implement
the same logic in @packages/functions/src/settings.ts
```

You want to make sure you provide a good amount of detail so MiMo Code makes the right changes.

#### Undo changes

Let's say you ask MiMo Code to make some changes.

```txt
Can you refactor the function in @packages/functions/src/api/index.ts?
```

But you realize that it is not what you wanted. You **can undo** the changes using the `/undo` command.

```bash
/undo
```

MiMo Code will now revert the changes you made and show your original message again.

Or you **can redo** the changes using the `/redo` command.

```bash
/redo
```

> **TIP**: You can run `/undo` multiple times to undo multiple changes.

### Customize

And that's it! You are now a pro at using MiMo Code.

To make it your own, we recommend picking a theme, customizing the keybinds, configuring code formatters, creating custom commands, or playing around with the MiMo Code config.

---

## Interaction & Input

MiMo Code provides an interactive terminal interface or TUI for working on your projects with an LLM. This page covers how to enter messages, reference files, run commands, and steer the model while you work in the TUI.

Running MiMo Code starts the TUI for the current directory.

```bash
mimo
```

Or you can start it for a specific working directory.

```bash
mimo /path/to/project
```

Once you're in the TUI, you can prompt it with a message.

```text
Give me a quick summary of the codebase.
```

To run MiMo Code programmatically from the command line instead, see [CLI Options](#cli-options).

### Input basics

Type your message in the input box at the bottom and press `enter` to send.

- **New line:** press `shift+enter`, `ctrl+j`, `ctrl+return`, or `alt+return` to insert a line break without sending.
- **In-line editing:** the input box supports common cursor and deletion shortcuts, e.g. `ctrl+a` / `ctrl+e` to jump to the start/end of the line, `ctrl+w` to delete the previous word, `ctrl+u` to delete to the start of the line.
- **Exit:** press `ctrl+c` or `ctrl+d`, or use the `/exit` command.

> **TIP**: Most actions use the `ctrl+x` leader key: press `ctrl+x` first, then the action key. For example, start a new session with `ctrl+x` then `n`.

### File references

You can reference files in your messages using `@`. This does a fuzzy file search in the current working directory.

```text
How is auth handled in @packages/functions/src/api/index.ts?
```

The content of the file is added to the conversation automatically.

### Image input

MiMo Code can scan images you provide and add them to your prompt. Just **drag and drop the image into the terminal window**.

```txt
[Image #1] Take a look at this image and use it as a reference.
```

Image support depends on whether the selected model supports multimodal input.

### Bash commands

Start a message with `!` to run a shell command.

```bash
!ls -la
```

The output of the command is added to the conversation as a tool result.

### Slash commands

Type `/` followed by a command name to quickly execute actions. For example:

```bash
/help
```

Most commands also have a keybind using `ctrl+x` as the leader key. For the full list of built-in commands, see [Slash Commands](#slash-commands); for keybinds, see [Keybinds](#keybinds).

### Mode switching and interrupts

MiMo Code provides two built-in working modes, **Build** and **Plan**. Build mode can read and write files and run commands directly; Plan mode only outputs an action plan without modifying files, which suits complex or high-risk tasks.

- **Switch modes:** press `tab` during a session to toggle between Build and Plan.
- **Interrupt a response:** while the model is streaming output or calling tools, press `esc` to interrupt the current turn.

### Configure TUI

You can customize TUI behavior through `tui.json` (or `tui.jsonc`).

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/tui.json",
  "theme": "mimocode",
  "keybinds": {
    "leader": "ctrl+x"
  },
  "scroll_speed": 3,
  "scroll_acceleration": {
    "enabled": true
  },
  "diff_style": "auto",
  "mouse": true
}
```

This is separate from `mimocode.json`, which configures server/runtime behavior.

#### TUI Options

- `theme` - Sets your UI theme.
- `keybinds` - Customizes keyboard shortcuts.
- `scroll_acceleration.enabled` - Enable macOS-style scroll acceleration for smooth, natural scrolling.
- `scroll_speed` - Controls how fast the TUI scrolls when using scroll commands (minimum: `0.001`, supports decimal values). Defaults to `3`.
- `diff_style` - Controls diff rendering. `"auto"` adapts to terminal width, `"stacked"` always shows a single-column layout.
- `mouse` - Enable or disable mouse capture in the TUI (default: `true`).

Use `MIMOCODE_TUI_CONFIG` to load a custom TUI config path.

---

## Sessions & Context

MiMo Code persists every conversation as a **session**, retaining its message history and metadata. You can close the terminal and resume your work later.

### Session storage

Session data is saved in MiMo Code's data directory, grouped by working directory.

```
$MIMOCODE_HOME/
├── config/       ← global config files
├── data/         ← auth.json, session database
├── state/
└── cache/
```

`MIMOCODE_HOME` is an absolute path to a single profile root holding the `config/`, `data/`, `state/`, and `cache/` subdirectories; setting it overrides all four XDG base directories.

> **WARNING**: Don't edit these files manually. Session state is managed by MiMo Code, and manual changes may make a session unrecoverable.

### Starting and resuming sessions

Running MiMo Code creates a new session in the current directory.

```bash
mimo
```

You can resume an existing session with the following flags:

```bash
# Resume the most recent session in the current directory
mimo --continue

# Resume a specific session by ID
mimo --session <id>
```

| Flag         | Short | Description                                                             |
| ------------ | ----- | ----------------------------------------------------------------------- |
| `--continue` | `-c`  | Continue the last session                                               |
| `--session`  | `-s`  | Session ID to continue                                                  |
| `--fork`     |       | Fork the session when continuing (use with `--continue` or `--session`) |

### Switching sessions in the TUI

Inside the TUI, you can switch between sessions using slash commands:

- **`/new`** (alias `/clear`) — start a new session, discarding the current context. Keybind `ctrl+x n`.
- **`/sessions`** (aliases `/resume`, `/continue`) — list and switch between sessions. Keybind `ctrl+x l`.

### Listing sessions

Outside the TUI, use the `session` command to manage sessions.

```bash
mimo session list
```

| Flag          | Short | Description                                  |
| ------------- | ----- | -------------------------------------------- |
| `--max-count` | `-n`  | Limit to the most recent N sessions          |
| `--format`    |       | Output format: table or json (default table) |

### Context compaction

Every model has a limited **context window**. As a conversation grows, the MiMo Code CLI automatically compacts earlier messages as the context approaches that limit to free up token space. You can also trigger it manually at any time:

```bash
/compact
```

`/compact` has the alias `/summarize` and the keybind `ctrl+x c`.

Compaction is performed by a hidden Compaction agent that compresses long context into a smaller summary.

#### Configuration

You can control context compaction behavior through the `compaction` option.

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "compaction": {
    "auto": true,
    "prune": true,
    "reserved": 10000
  }
}
```

- `auto` - Automatically compact the session when context is full (default: `true`).
- `prune` - Remove old tool outputs to save tokens (default: `true`).
- `reserved` - Token buffer for compaction. Leaves enough window to avoid overflow during compaction.

To disable automatic compaction entirely, set the environment variable `MIMOCODE_DISABLE_AUTOCOMPACT=true`.

### Forking sessions

Forking lets you derive an independent copy of a session to try new ideas without affecting the original. The two sessions are independent and don't affect each other.

```bash
# Fork from the most recent session and continue
mimo --continue --fork

# Fork from a specific session
mimo --session <id> --fork
```

### Exporting and importing

You can export a session to JSON for archiving, sharing, or bug reports.

```bash
mimo export [sessionID]
```

If you don't provide a session ID, you'll be prompted to choose from the available sessions.

You can then import session data from a local file or a MiMo Code share link.

```bash
mimo import session.json
mimo import https://opncd.ai/s/abc123
```

> **TIP**: Exported files may contain sensitive information (code, command output, paths, etc.). Review the contents before sharing.

---

## Modes

### Built-in

MiMo Code comes with three built-in modes (primary agents), each with a distinct role:

- **`build`** — default primary agent with full tool access for general development work.
- **`plan`** — restricted primary agent for read-only analysis and planning.
- **`compose`** — primary agent that orchestrates work through built-in skills.

There are also **`general`** / **`explore`** — subagents invoked by primary agents for delegated tasks.

### Build

Build is the **default** mode with all tools enabled. This is the standard mode for development work where you need full access to file operations and system commands.

### Plan

A restricted mode designed for planning and analysis. In plan mode, the following tools are disabled by default:

- `write` - Cannot create new files
- `edit` - Cannot modify existing files, except for files located at `.mimocode/plans/*.md` to detail the plan itself
- `patch` - Cannot apply patches
- `bash` - Cannot execute shell commands

This mode is useful when you want the AI to analyze code, suggest changes, or create plans without making any actual modifications to your codebase.

### Compose

Compose is a primary agent in MiMo Code that orchestrates task execution through a curated set of built-in skills. Instead of relying on a single monolithic prompt, Compose consults a library of 13 focused skills—covering testing, debugging, planning, collaboration, and meta-development—and selects the right skill for each step of your workflow.

> **TIP**: Switch to the Compose agent when you want a workflow-driven loop: brainstorm a feature, write a plan, execute it with TDD, verify before completion, and request a code review—all guided by skills.

#### Overview

Compose was inspired by the open-source [superpowers](https://github.com/obra/superpowers) workflow and ported into MiMo Code as a first-class primary agent. Like `build` and `plan`, Compose handles your main conversation and can be selected with the **Tab** key. What sets it apart is the bundled skill library: every skill ships with the binary, is loaded at runtime under the `compose:` namespace, and is available only when the Compose agent is active.

#### Enable Compose mode

Compose is registered as a built-in primary agent under the name `compose`. To activate it:

1. Press **Tab** (or your configured `switch_agent` keybind) to cycle through primary agents until **Compose** is selected.
2. Or invoke it with `@compose` in a message.

No additional configuration is required. The skill bundle is extracted to `{data}/compose/{version}/` on first use and kept up to date across MiMo Code releases.

#### Built-in skills

Compose ships with 13 skills, grouped by category. They are referenced from the model side via the `compose:<short-name>` namespace.

**Testing:**

| Skill         | Purpose                          |
| ------------- | -------------------------------- |
| `compose:tdd` | Test-driven development workflow |

**Debugging:**

| Skill            | Purpose                          |
| ---------------- | -------------------------------- |
| `compose:debug`  | Systematic debugging methodology |
| `compose:verify` | Verification before completion   |

**Collaboration:**

| Skill                | Purpose                                |
| -------------------- | -------------------------------------- |
| `compose:brainstorm` | Brainstorming with the user            |
| `compose:plan`       | Writing implementation plans           |
| `compose:execute`    | Executing an approved plan             |
| `compose:parallel`   | Dispatching parallel agents            |
| `compose:review`     | Requesting a code review               |
| `compose:feedback`   | Receiving and applying review feedback |
| `compose:worktree`   | Working in a git worktree              |
| `compose:merge`      | Finishing a development branch         |
| `compose:subagent`   | Subagent-driven development            |

**Meta:**

| Skill               | Purpose               |
| ------------------- | --------------------- |
| `compose:new-skill` | Authoring a new skill |

### Switching

You can switch between modes during a session using the _Tab_ key. Or your configured `switch_mode` keybind.

---

## Tools

Tools allow the LLM to perform actions in your codebase. MiMo Code comes with a set of built-in tools, but you can extend it with custom tools or MCP servers.

By default, all tools are **enabled** and don't need permission to run. You can control tool behavior through permissions.

### Configure

Use the `permission` field to control tool behavior. You can allow, deny, or require approval for each tool.

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "permission": {
    "edit": "deny",
    "bash": "ask",
    "webfetch": "allow"
  }
}
```

You can also use wildcards to control multiple tools at once:

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "permission": {
    "mymcp_*": "ask"
  }
}
```

### Built-in Tools

#### bash

Execute shell commands in your project environment.

This tool allows the LLM to run terminal commands like `npm install`, `git status`, or any other shell command.

#### edit

Modify existing files using exact string replacements. This is the primary way the LLM modifies code.

#### write

Create new files or overwrite existing ones. Use this to allow the LLM to create new files.

> **NOTE**: The `write` tool is controlled by the `edit` permission, which covers all file modifications (`edit`, `write`, `apply_patch`, `multiedit`).

#### read

Read file contents from your codebase. This tool reads files and returns their contents. It supports reading specific line ranges for large files.

#### grep

Search file contents using regular expressions. Fast content search across your codebase. Supports full regex syntax and file pattern filtering.

#### glob

Find files by pattern matching. Search for files using glob patterns like `**/*.js` or `src/**/*.ts`. Returns matching file paths sorted by modification time.

#### lsp (experimental)

Interact with your configured LSP servers to get code intelligence features like definitions, references, hover info, and call hierarchy.

> **NOTE**: This tool is only available when `MIMOCODE_EXPERIMENTAL_LSP_TOOL=true` (or `MIMOCODE_EXPERIMENTAL=true`).

Supported operations include `goToDefinition`, `findReferences`, `hover`, `documentSymbol`, `workspaceSymbol`, `goToImplementation`, `prepareCallHierarchy`, `incomingCalls`, and `outgoingCalls`.

#### apply_patch

Apply patches to files. This tool applies patch files to your codebase. Useful for applying diffs and patches from various sources.

`apply_patch` uses `output.args.patchText` instead of `output.args.filePath`. Paths are embedded in marker lines within `patchText` and are relative to the project root.

#### skill

Load a skill (a `SKILL.md` file) and return its content in the conversation.

#### todowrite

Manage todo lists during coding sessions. Creates and updates task lists to track progress during complex operations.

> **NOTE**: This tool is disabled for subagents by default, but you can enable it manually.

#### webfetch

Fetch web content. Allows the LLM to fetch and read web pages. Useful for looking up documentation or researching online resources.

#### websearch

Search the web for information.

> **NOTE**: This tool is only available when using the MiMo Code provider or when the `MIMOCODE_ENABLE_EXA` environment variable is set to any truthy value.

Performs web searches using Exa AI to find relevant information online. No API key is required — the tool connects directly to Exa AI's hosted MCP service without authentication.

> **TIP**: Use `websearch` when you need to find information (discovery), and `webfetch` when you need to retrieve content from a specific URL (retrieval).

#### question

Ask the user questions during execution. This tool allows the LLM to ask the user questions during a task. It's useful for:

- Gathering user preferences or requirements
- Clarifying ambiguous instructions
- Getting decisions on implementation choices
- Offering choices about what direction to take

### Custom tools

Custom tools let you define your own functions that the LLM can call. These are defined in your config file and can execute arbitrary code.

### MCP servers

MCP (Model Context Protocol) servers allow you to integrate external tools and services. This includes database access, API integrations, and third-party services.

### Internals

Internally, tools like `grep` and `glob` use [ripgrep](https://github.com/BurntSushi/ripgrep) under the hood. By default, ripgrep respects `.gitignore` patterns.

#### Ignore patterns

To include files that would normally be ignored, create a `.ignore` file in your project root:

```
!node_modules/
!dist/
!build/
```

---

## Config Files

MiMo Code manages runtime parameters through **JSON / JSONC** config files, with multiple layers merged on load. The managed layer takes the highest precedence.

For this project, the main config is [`mimocode.json`](mimocode.json), with local defaults in [`.agents/settings.json`](.agents/settings.json).

### Essentials

- Top-level fields include: `model`, `small_model`, `provider`, `default_agent`, `permission`, `mcp`, `skills`, `compaction`, `checkpoint`, `server`, and `instructions`.
- Custom agents and command templates live under `agent` and `command`.
- Permissions support global and per-agent/tool rules: `allow`, `ask`, or `deny`.
- Provider options support `apiKey`, `baseURL`, `timeout`, and `setCacheKey`.
- Values can reference environment variables with `{env:VAR}` and file contents with `{file:path}`.

### Example

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "model": "mimo/mimo-v2.5-pro",
  "default_agent": "build",
  "share": "manual",
  "autoupdate": true,
  "permission": { "*": "ask", "bash": "allow", "edit": "deny" },
  "compaction": { "auto": true, "prune": true, "tail_turns": 2 },
  "watcher": { "ignore": ["node_modules/**", "dist/**"] },
  "mcp": {},
  "plugin": ["mimocode-helicone-session"],
  "instructions": ["CONTRIBUTING.md"],
  "disabled_providers": ["openai", "gemini"]
}
```

See [`.agents/settings.json`](.agents/settings.json) for the project-local defaults.

Config values can reference environment variables and file contents:

```json
{
  "model": "{env:MIMOCODE_MODEL}",
  "provider": {
    "anthropic": {
      "options": {
        "apiKey": "{env:ANTHROPIC_API_KEY}",
        "baseURL": "{file:~/.secrets/anthropic-endpoint}"
      }
    }
  }
}
```

- `{env:VAR}` — replaced with an empty string when unset
- `{file:path}` — relative to the config file directory, or an absolute path starting with `/` or `~`

> **NOTE**: Placeholders inside `//` single-line comments in JSONC are not substituted.

### Schema

Config files follow `mimo.xiaomi.com/mimocode/config.json`. Add `"$schema"` at the top of the file to enable editor completion and validation:

```json
{ "$schema": "https://mimo.xiaomi.com/mimocode/config.json" }
```

---

## Permissions

MiMo Code uses the `permission` config to decide whether a given action should run automatically, prompt you, or be blocked.

### Actions

Each permission rule resolves to one of:

- `"allow"` — run without approval
- `"ask"` — prompt for approval
- `"deny"` — block the action

### Configuration

You can set permissions globally (with `*`), and override specific tools.

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "permission": {
    "*": "ask",
    "bash": "allow",
    "edit": "deny"
  }
}
```

You can also set all permissions at once:

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "permission": "allow"
}
```

### Granular Rules (Object Syntax)

For most permissions, you can use an object to apply different actions based on the tool input.

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "permission": {
    "bash": {
      "*": "ask",
      "git *": "allow",
      "npm *": "allow",
      "rm *": "deny",
      "grep *": "allow"
    },
    "edit": {
      "*": "deny",
      "packages/web/src/content/docs/*.mdx": "allow"
    }
  }
}
```

Rules are evaluated by pattern match, with the **last matching rule winning**. A common pattern is to put the catch-all `"*"` rule first, and more specific rules after it.

### Wildcards

Permission patterns use simple wildcard matching:

- `*` matches zero or more of any character
- `?` matches exactly one character
- All other characters match literally

### Home Directory Expansion

You can use `~` or `$HOME` at the start of a pattern to reference your home directory.

- `~/projects/*` -> `/Users/username/projects/*`
- `$HOME/projects/*` -> `/Users/username/projects/*`
- `~` -> `/Users/username`

### External Directories

Use `external_directory` to allow tool calls that touch paths outside the working directory where MiMo Code was started.

For example, this allows access to everything under `~/projects/personal/`:

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "permission": {
    "external_directory": {
      "~/projects/personal/**": "allow"
    }
  }
}
```

Any directory allowed here inherits the same defaults as the current workspace. Add explicit rules when a tool should be restricted in these paths:

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "permission": {
    "external_directory": {
      "~/projects/personal/**": "allow"
    },
    "edit": {
      "~/projects/personal/**": "deny"
    }
  }
}
```

### Available Permissions

MiMo Code permissions are keyed by tool name, plus a couple of safety guards:

- `read` — reading a file (matches the file path)
- `edit` — all file modifications (covers `edit`, `write`, `patch`, `multiedit`)
- `glob` — file globbing (matches the glob pattern)
- `grep` — content search (matches the regex pattern)
- `bash` — running shell commands (matches parsed commands like `git status --porcelain`)
- `task` — launching subagents (matches the subagent type)
- `skill` — loading a skill (matches the skill name)
- `lsp` — running LSP queries (currently non-granular)
- `question` — asking the user questions during execution
- `webfetch` — fetching a URL (matches the URL)
- `websearch`, `codesearch` — web/code search (matches the query)
- `external_directory` — triggered when a tool touches paths outside the project working directory
- `doom_loop` — triggered when the same tool call repeats 3 times with identical input

### Defaults

If you don't specify anything, MiMo Code starts from permissive defaults:

- Most permissions default to `"allow"`.
- `doom_loop` and `external_directory` default to `"ask"`.
- `read` is `"allow"`, but `.env` files are denied by default:

```json
{
  "permission": {
    "read": {
      "*": "allow",
      "*.env": "deny",
      "*.env.*": "deny",
      "*.env.example": "allow"
    }
  }
}
```

### What "Ask" Does

When MiMo Code prompts for approval, the UI offers three outcomes:

- `once` — approve just this request
- `always` — approve future requests matching the suggested patterns (for the rest of the current MiMo Code session)
- `reject` — deny the request

The set of patterns that `always` would approve is provided by the tool.

### Agents

You can override permissions per agent. Agent permissions are merged with the global config, and agent rules take precedence.

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "permission": {
    "bash": {
      "*": "ask",
      "git *": "allow",
      "git commit *": "deny",
      "git push *": "deny",
      "grep *": "allow"
    }
  },
  "agent": {
    "build": {
      "permission": {
        "bash": {
          "*": "ask",
          "git *": "allow",
          "git commit *": "ask",
          "git push *": "deny",
          "grep *": "allow"
        }
      }
    }
  }
}
```

You can also configure agent permissions in Markdown:

```markdown
---
description: Code review without edits
mode: subagent
permission:
  edit: deny
  bash: ask
  webfetch: deny
---

Only analyze code and suggest changes.
```

> **TIP**: Use pattern matching for commands with arguments. `"grep *"` allows `grep pattern file.txt`, while `"grep"` alone would block it.

---

## Agent Skills

Agent skills let MiMo Code discover reusable instructions from your repo or home directory. Skills are loaded on-demand via the native `skill` tool—agents see available skills and can load the full content when needed.

For this project, skills are stored in [`.agents/skills/`](.agents/skills) and mirrored via `.mimocode/`.

### Essentials

- Each skill is a folder containing `SKILL.md` with required frontmatter: `name` and `description`.
- Discovery order: project `.mimocode/skills/**/SKILL.md`, compatibility dirs `.claude/.agents/.codex/.opencode`, then global `~/.config/mimocode/skills/**/SKILL.md`.
- Duplicate names are overridden by the last-loaded skill; a warning is logged.
- Permissions control loading behavior: `allow`, `deny`, or `ask` per skill pattern.

See [`.agents/README.md`](.agents/README.md) for local conventions and [SKILL.md](SKILL.md) for the lint-arwaky command catalog.

---

## Agents

Agents are specialized AI assistants configured for specific tasks and workflows. You can switch between agents during a session or invoke them with the `@` mention.

### Types

There are two types of agents in MiMo Code; primary agents and subagents.

#### Primary agents

Primary agents are the main assistants you interact with directly. You can cycle through them using the **Tab** key, or your configured `switch_agent` keybind. Tool access is configured via permissions — for example, Build has all tools enabled while Plan is restricted.

#### Subagents

Subagents are specialized assistants that primary agents can invoke for specific tasks. You can also manually invoke them by **@ mentioning** them in your messages.

### Built-in

- **Build** — default primary agent with full tool access.
- **Plan** — restricted primary agent for planning and analysis; write/edit/execute tools are gated by approval prompts.
- **Compose** — workflow-driven primary agent that uses built-in skills to execute plans.
- **general / explore** — common subagents for delegated tasks.

See [`.agents/README.md`](.agents/README.md) for local agent/skill folder conventions.

- `file edits`: All writes, patches, and edits
- `bash`: All bash commands

This agent is useful when you want the LLM to analyze code, suggest changes, or create plans without making any actual modifications to your codebase.

#### General

_Mode_: `subagent`

A general-purpose agent for researching complex questions and executing multi-step tasks. Has full tool access (except todo), so it can make file changes when needed. Use this to run multiple units of work in parallel.

#### Explore

_Mode_: `subagent`

A fast, read-only agent for exploring codebases. Cannot modify files. Use this when you need to quickly find files by patterns, search code for keywords, or answer questions about the codebase.

#### Compaction

_Mode_: `primary`

Hidden system agent that compacts long context into a smaller summary. It runs automatically when needed and is not selectable in the UI.

#### Title

_Mode_: `primary`

Hidden system agent that generates short session titles. It runs automatically and is not selectable in the UI.

#### Summary

_Mode_: `primary`

Hidden system agent that creates session summaries. It runs automatically and is not selectable in the UI.

### Usage

1. For primary agents, use the **Tab** key to cycle through them during a session. You can also use your configured `switch_agent` keybind.

2. Subagents can be invoked:
   - **Automatically** by primary agents for specialized tasks based on their descriptions.
   - Manually by **@ mentioning** a subagent in your message. For example:

     ```txt
     @general help me search for this function
     ```

3. **Navigation between sessions**: When subagents create child sessions, use `session_child_first` (default: **<Leader>+Down**) to enter the first child session from the parent.

4. Once you are in a child session, use:
   - `session_child_cycle` (default: **Right**) to cycle to the next child session
   - `session_child_cycle_reverse` (default: **Left**) to cycle to the previous child session
   - `session_parent` (default: **Up**) to return to the parent session

### Configure

You can customize the built-in agents or create your own through configuration. Agents can be configured in two ways:

#### JSON

Configure agents in your `mimocode.json` config file:

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "agent": {
    "build": {
      "mode": "primary",
      "model": "mimo/mimo-v2.5-pro",
      "prompt": "{file:./prompts/build.txt}",
      "tools": {
        "write": true,
        "edit": true,
        "bash": true
      }
    },
    "plan": {
      "mode": "primary",
      "model": "mimo/mimo-v2.5-pro",
      "tools": {
        "write": false,
        "edit": false,
        "bash": false
      }
    },
    "code-reviewer": {
      "description": "Reviews code for best practices and potential issues",
      "mode": "subagent",
      "model": "mimo/mimo-v2.5-pro",
      "prompt": "You are a code reviewer. Focus on security, performance, and maintainability.",
      "tools": {
        "write": false,
        "edit": false
      }
    }
  }
}
```

#### Markdown

You can also define agents using markdown files. Place them in:

- Global: `~/.config/mimocode/agents/`
- Per-project: `.mimocode/agents/`

```markdown
---
description: Reviews code for quality and best practices
mode: subagent
model: mimo/mimo-v2.5-pro
temperature: 0.1
tools:
  write: false
  edit: false
  bash: false
---

You are in code review mode. Focus on:

- Code quality and best practices
- Potential bugs and edge cases
- Performance implications
- Security considerations

Provide constructive feedback without making direct changes.
```

The markdown file name becomes the agent name. For example, `review.md` creates a `review` agent.

### Options

#### Description

Use the `description` option to provide a brief description of what the agent does and when to use it. This is a **required** config option.

#### Temperature

Control the randomness and creativity of the LLM's responses with the `temperature` config.

Lower values make responses more focused and deterministic, while higher values increase creativity and variability.

Temperature values typically range from 0.0 to 1.0:

- **0.0-0.2**: Very focused and deterministic responses, ideal for code analysis and planning
- **0.3-0.5**: Balanced responses with some creativity, good for general development tasks
- **0.6-1.0**: More creative and varied responses, useful for brainstorming and exploration

If no temperature is specified, MiMo Code uses model-specific defaults; typically 0 for most models, 0.55 for Qwen models.

#### Max steps

Control the maximum number of agentic iterations an agent can perform before being forced to respond with text only.

```json
{
  "agent": {
    "quick-thinker": {
      "description": "Fast reasoning with limited iterations",
      "prompt": "You are a quick thinker. Solve problems with minimal steps.",
      "steps": 5
    }
  }
}
```

> **CAUTION**: The legacy `maxSteps` field is deprecated. Use `steps` instead.

#### Disable

Set to `true` to disable the agent.

```json
{
  "agent": {
    "review": {
      "disable": true
    }
  }
}
```

#### Prompt

Specify a custom system prompt file for this agent with the `prompt` config.

```json
{
  "agent": {
    "review": {
      "prompt": "{file:./prompts/code-review.txt}"
    }
  }
}
```

#### Model

Use the `model` config to override the model for this agent. Useful for using different models optimized for different tasks.

> **TIP**: If you don't specify a model, primary agents use the model globally configured while subagents will use the model of the primary agent that invoked the subagent.

```json
{
  "agent": {
    "plan": {
      "model": "mimo/mimo-v2.5-pro"
    }
  }
}
```

#### Tools (deprecated)

`tools` is **deprecated**. Prefer the agent's `permission` field for new configs, updates and more fine-granular control.

Allows you to control which tools are available in this agent. You can enable or disable specific tools by setting them to `true` or `false`.

#### Permissions

You can configure permissions to manage what actions an agent can take. Currently, the permissions for the `edit`, `bash`, and `webfetch` tools can be configured to:

- `"ask"` — Prompt for approval before running the tool
- `"allow"` — Allow all operations without approval
- `"deny"` — Disable the tool

#### Mode

Control the agent's mode with the `mode` config. The `mode` option can be set to `primary`, `subagent`, or `all`. If no `mode` is specified, it defaults to `all`.

#### Hidden

Hide a subagent from the `@` autocomplete menu with `hidden: true`. Useful for internal subagents that should only be invoked programmatically by other agents via the Task tool.

#### Task permissions

Control which subagents an agent can invoke via the Task tool with `permission.task`. Uses glob patterns for flexible matching.

```json
{
  "agent": {
    "orchestrator": {
      "mode": "primary",
      "permission": {
        "task": {
          "*": "deny",
          "orchestrator-*": "allow",
          "code-reviewer": "ask"
        }
      }
    }
  }
}
```

#### Color

Customize the agent's visual appearance in the UI with the `color` option.

Use a valid hex color (e.g., `#FF5733`) or theme color: `primary`, `secondary`, `accent`, `success`, `warning`, `error`, `info`.

#### Top P

Control response diversity with the `top_p` option. Values range from 0.0 to 1.0. Lower values are more focused, higher values more diverse.

#### Additional

Any other options you specify in your agent configuration will be **passed through directly** to the provider as model options.

### Create agents

You can create new agents using the following command:

```bash
mimo agent create
```

This interactive command will:

1. Ask where to save the agent; global or project-specific.
2. Description of what the agent should do.
3. Generate an appropriate system prompt and identifier.
4. Let you select which tools the agent can access.
5. Finally, create a markdown file with the agent configuration.

### Use cases

- **Build agent**: Full development work with all tools enabled
- **Plan agent**: Analysis and planning without making changes
- **Review agent**: Code review with read-only access plus documentation tools
- **Debug agent**: Focused on investigation with bash and read tools enabled
- **Docs agent**: Documentation writing with file operations but no system commands

### Examples

#### Documentation agent

```markdown
---
description: Writes and maintains project documentation
mode: subagent
tools:
  bash: false
---

You are a technical writer. Create clear, comprehensive documentation.

Focus on:

- Clear explanations
- Proper structure
- Code examples
- User-friendly language
```

#### Security auditor

```markdown
---
description: Performs security audits and identifies vulnerabilities
mode: subagent
tools:
  write: false
  edit: false
---

You are a security expert. Focus on identifying potential security issues.

Look for:

- Input validation vulnerabilities
- Authentication and authorization flaws
- Data exposure risks
- Dependency vulnerabilities
- Configuration security issues
```

---

## Slash Commands

In addition to commands you define yourself, MiMo Code ships with a set of built-in slash commands. When using the TUI, type `/` followed by a command name to quickly execute actions.

> **NOTE**: Custom commands can override built-in commands. If you define a custom command with the same name, it will override the built-in command.

### connect

Add a provider to MiMo Code. Allows you to select from available providers and add their API keys.

```bash
/connect
```

### compact

Compact the current session. _Alias_: `/summarize`

```bash
/compact
```

**Keybind:** `ctrl+x c`

### details

Toggle tool execution details.

```bash
/details
```

**Keybind:** `ctrl+x d`

### editor

Open external editor for composing messages. Uses the editor set in your `EDITOR` environment variable.

```bash
/editor
```

**Keybind:** `ctrl+x e`

### exit

Exit MiMo Code. _Aliases_: `/quit`, `/q`

```bash
/exit
```

**Keybind:** `ctrl+x q`

### export

Export current conversation to Markdown and open in your default editor.

```bash
/export
```

**Keybind:** `ctrl+x x`

### help

Show the help dialog.

```bash
/help
```

**Keybind:** `ctrl+x h`

### init

Guided setup for creating or updating `AGENTS.md`.

```bash
/init
```

**Keybind:** `ctrl+x i`

### models

List available models.

```bash
/models
```

**Keybind:** `ctrl+x m`

### new

Start a new session. _Alias_: `/clear`

```bash
/new
```

**Keybind:** `ctrl+x n`

### redo

Redo a previously undone message. Only available after using `/undo`.

> **TIP**: Any file changes will also be restored. Internally, this uses Git to manage the file changes. So your project **needs to be a Git repository**.

```bash
/redo
```

**Keybind:** `ctrl+x r`

### sessions

List and switch between sessions. _Aliases_: `/resume`, `/continue`

```bash
/sessions
```

**Keybind:** `ctrl+x l`

### share

Share current session.

```bash
/share
```

**Keybind:** `ctrl+x s`

### themes

List available themes.

```bash
/themes
```

**Keybind:** `ctrl+x t`

### thinking

Toggle the visibility of thinking/reasoning blocks in the conversation.

> **NOTE**: This command only controls whether thinking blocks are **displayed** - it does not enable or disable the model's reasoning capabilities.

### undo

Undo last message in the conversation. Removes the most recent user message, all subsequent responses, and any file changes.

> **TIP**: Any file changes made will also be reverted. Internally, this uses Git to manage the file changes. So your project **needs to be a Git repository**.

```bash
/undo
```

**Keybind:** `ctrl+x u`

### unshare

Unshare current session.

---

## CLI Options

This page lists the flags and arguments accepted by each MiMo Code CLI command.

### tui

```bash
mimo [project]
```

| Flag         | Short | Description                                |
| ------------ | ----- | ------------------------------------------ |
| `--continue` | `-c`  | Continue the last session                  |
| `--session`  | `-s`  | Session ID to continue                     |
| `--fork`     |       | Fork the session when continuing           |
| `--prompt`   |       | Prompt to use                              |
| `--model`    | `-m`  | Model to use in the form of provider/model |
| `--agent`    |       | Agent to use                               |
| `--port`     |       | Port to listen on                          |
| `--hostname` |       | Hostname to listen on                      |

### attach

```bash
mimo attach [url]
```

| Flag        | Short | Description                       |
| ----------- | ----- | --------------------------------- |
| `--dir`     |       | Working directory to start TUI in |
| `--session` | `-s`  | Session ID to continue            |

### github run

```bash
mimo github run
```

| Flag      | Description                            |
| --------- | -------------------------------------- |
| `--event` | GitHub mock event to run the agent for |
| `--token` | GitHub personal access token           |

### models

```bash
mimo models [provider]
```

| Flag        | Description                                                  |
| ----------- | ------------------------------------------------------------ |
| `--refresh` | Refresh the models cache from models.dev                     |
| `--verbose` | Use more verbose model output (includes metadata like costs) |

### run

```bash
mimo run [message..]
```

| Flag                             | Short | Description                                                          |
| -------------------------------- | ----- | -------------------------------------------------------------------- |
| `--command`                      |       | The command to run, use message for args                             |
| `--continue`                     | `-c`  | Continue the last session                                            |
| `--session`                      | `-s`  | Session ID to continue                                               |
| `--fork`                         |       | Fork the session when continuing                                     |
| `--share`                        |       | Share the session                                                    |
| `--model`                        | `-m`  | Model to use in the form of provider/model                           |
| `--agent`                        |       | Agent to use                                                         |
| `--file`                         | `-f`  | File(s) to attach to message                                         |
| `--format`                       |       | Format: default (formatted) or json (raw JSON events)                |
| `--title`                        |       | Title for the session                                                |
| `--attach`                       |       | Attach to a running mimocode server                                  |
| `--port`                         |       | Port for the local server                                            |
| `--dangerously-skip-permissions` |       | Auto-approve permissions that are not explicitly denied (dangerous!) |

### serve

```bash
mimo serve
```

| Flag         | Description                                |
| ------------ | ------------------------------------------ |
| `--port`     | Port to listen on                          |
| `--hostname` | Hostname to listen on                      |
| `--mdns`     | Enable mDNS discovery                      |
| `--cors`     | Additional browser origin(s) to allow CORS |

### session list

```bash
mimo session list
```

| Flag          | Short | Description                          |
| ------------- | ----- | ------------------------------------ |
| `--max-count` | `-n`  | Limit to N most recent sessions      |
| `--format`    |       | Output format: table or json (table) |

### stats

```bash
mimo stats
```

| Flag        | Description                               |
| ----------- | ----------------------------------------- |
| `--days`    | Show stats for the last N days (all time) |
| `--tools`   | Number of tools to show (all)             |
| `--models`  | Show model usage breakdown                |
| `--project` | Filter by project                         |

### web

```bash
mimo web
```

| Flag         | Description                                |
| ------------ | ------------------------------------------ |
| `--port`     | Port to listen on                          |
| `--hostname` | Hostname to listen on                      |
| `--mdns`     | Enable mDNS discovery                      |
| `--cors`     | Additional browser origin(s) to allow CORS |

### acp

```bash
mimo acp
```

| Flag         | Description           |
| ------------ | --------------------- |
| `--cwd`      | Working directory     |
| `--port`     | Port to listen on     |
| `--hostname` | Hostname to listen on |

### uninstall

```bash
mimo uninstall
```

| Flag            | Short | Description                                 |
| --------------- | ----- | ------------------------------------------- |
| `--keep-config` | `-c`  | Keep configuration files                    |
| `--keep-data`   | `-d`  | Keep session data and snapshots             |
| `--dry-run`     |       | Show what would be removed without removing |
| `--force`       | `-f`  | Skip confirmation prompts                   |

### upgrade

```bash
mimo upgrade [target]
```

| Flag       | Short | Description                                                       |
| ---------- | ----- | ----------------------------------------------------------------- |
| `--method` | `-m`  | The installation method that was used; curl, npm, pnpm, bun, brew |

### Global Flags

Apply to all commands.

| Flag           | Short | Description                          |
| -------------- | ----- | ------------------------------------ |
| `--help`       | `-h`  | Display help                         |
| `--version`    | `-v`  | Print version number                 |
| `--print-logs` |       | Print logs to stderr                 |
| `--log-level`  |       | Log level (DEBUG, INFO, WARN, ERROR) |

---

## Keybinds

MiMo Code has a list of keybinds that you can customize through `tui.json`.

### Leader key

MiMo Code uses a `leader` key for most keybinds. This avoids conflicts in your terminal.

By default, `ctrl+x` is the leader key and most actions require you to first press the leader key and then the shortcut. For example, to start a new session you first press `ctrl+x` and then press `n`.

You don't need to use a leader key for your keybinds but we recommend doing so.

Some navigation keybinds intentionally do not use the leader key by default. For subagent sessions, the defaults are `session_child_first` = `<leader>down`, `session_child_cycle` = `right`, `session_child_cycle_reverse` = `left`, and `session_parent` = `up`.

### Disable keybind

You can disable a keybind by adding the key to `tui.json` with a value of "none".

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/tui.json",
  "keybinds": {
    "session_compact": "none"
  }
}
```

### Desktop prompt shortcuts

The MiMo Code desktop app prompt input supports common Readline/Emacs-style shortcuts for editing text.

| Shortcut | Action                                   |
| -------- | ---------------------------------------- |
| `ctrl+a` | Move to start of current line            |
| `ctrl+e` | Move to end of current line              |
| `ctrl+b` | Move cursor back one character           |
| `ctrl+f` | Move cursor forward one character        |
| `alt+b`  | Move cursor back one word                |
| `alt+f`  | Move cursor forward one word             |
| `ctrl+d` | Delete character under cursor            |
| `ctrl+k` | Kill to end of line                      |
| `ctrl+u` | Kill to start of line                    |
| `ctrl+w` | Kill previous word                       |
| `alt+d`  | Kill next word                           |
| `ctrl+t` | Transpose characters                     |
| `ctrl+g` | Cancel popovers / abort running response |

### Shift+Enter

Some terminals don't send modifier keys with Enter by default. You may need to configure your terminal to send `Shift+Enter` as an escape sequence.

#### Windows Terminal

Open your `settings.json` at:

```
%LOCALAPPDATA%\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\settings.json
```

Add this to the root-level `actions` array:

```json
"actions": [
  {
    "command": {
      "action": "sendInput",
      "input": "\u001b[13;2u"
    },
    "id": "User.sendInput.ShiftEnterCustom"
  }
]
```

Add this to the root-level `keybindings` array:

```json
"keybindings": [
  {
    "keys": "shift+enter",
    "id": "User.sendInput.ShiftEnterCustom"
  }
]
```

Save the file and restart Windows Terminal or open a new tab.

---

## MCP Servers

You can add external tools to MiMo Code using the _Model Context Protocol_, or MCP. MiMo Code supports both local and remote servers.

Once added, MCP tools are automatically available to the LLM alongside built-in tools.

### Caveats

When you use an MCP server, it adds to the context. This can quickly add up if you have a lot of tools. So we recommend being careful with which MCP servers you use.

> **TIP**: MCP servers add to your context, so you want to be careful with which ones you enable. Certain MCP servers, like the GitHub MCP server, tend to add a lot of tokens and can easily exceed the context limit.

### Enable

You can define MCP servers in your MiMo Code Config under `mcp`. Add each MCP with a unique name.

```jsonc
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "name-of-mcp-server": {
      // ...
      "enabled": true,
    },
    "name-of-other-mcp-server": {
      // ...
    },
  },
}
```

You can also disable a server by setting `enabled` to `false`.

### Overriding remote defaults

Organizations can provide default MCP servers via their `.well-known/mimocode` endpoint. These servers may be disabled by default, allowing users to opt-in to the ones they need.

To enable a specific server from your organization's remote config, add it to your local config with `enabled: true`:

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "jira": {
      "type": "remote",
      "url": "https://jira.example.com/mcp",
      "enabled": true
    }
  }
}
```

### Local

Add local MCP servers using `type` to `"local"` within the MCP object.

```jsonc
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "my-local-mcp-server": {
      "type": "local",
      // Or ["bun", "x", "my-mcp-command"]
      "command": ["npx", "-y", "my-mcp-command"],
      "enabled": true,
      "environment": {
        "MY_ENV_VAR": "my_env_var_value",
      },
    },
  },
}
```

For example, here's how you can add the test `@modelcontextprotocol/server-everything` MCP server.

```jsonc
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "mcp_everything": {
      "type": "local",
      "command": ["npx", "-y", "@modelcontextprotocol/server-everything"],
    },
  },
}
```

And to use it I can add `use the mcp_everything tool` to my prompts.

```txt
use the mcp_everything tool to add the number 3 and 4
```

#### Local Options

| Option        | Type    | Required | Description                                                                         |
| ------------- | ------- | -------- | ----------------------------------------------------------------------------------- |
| `type`        | String  | Y        | Type of MCP server connection, must be `"local"`.                                   |
| `command`     | Array   | Y        | Command and arguments to run the MCP server.                                        |
| `environment` | Object  |          | Environment variables to set when running the server.                               |
| `enabled`     | Boolean |          | Enable or disable the MCP server on startup.                                        |
| `timeout`     | Number  |          | Timeout in ms for fetching tools from the MCP server. Defaults to 5000 (5 seconds). |

### Remote

Add remote MCP servers by setting `type` to `"remote"`.

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "my-remote-mcp": {
      "type": "remote",
      "url": "https://my-mcp-server.com",
      "enabled": true,
      "headers": {
        "Authorization": "Bearer MY_API_KEY"
      }
    }
  }
}
```

#### Remote Options

| Option    | Type    | Required | Description                                                                         |
| --------- | ------- | -------- | ----------------------------------------------------------------------------------- |
| `type`    | String  | Y        | Type of MCP server connection, must be `"remote"`.                                  |
| `url`     | String  | Y        | URL of the remote MCP server.                                                       |
| `enabled` | Boolean |          | Enable or disable the MCP server on startup.                                        |
| `headers` | Object  |          | Headers to send with the request.                                                   |
| `oauth`   | Object  |          | OAuth authentication configuration.                                                 |
| `timeout` | Number  |          | Timeout in ms for fetching tools from the MCP server. Defaults to 5000 (5 seconds). |

### OAuth

MiMo Code automatically handles OAuth authentication for remote MCP servers. When a server requires authentication, MiMo Code will:

1. Detect the 401 response and initiate the OAuth flow
2. Use **Dynamic Client Registration (RFC 7591)** if supported by the server
3. Store tokens securely for future requests

#### Automatic

For most OAuth-enabled MCP servers, no special configuration is needed. Just configure the remote server:

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "my-oauth-server": {
      "type": "remote",
      "url": "https://mcp.example.com/mcp"
    }
  }
}
```

If the server requires authentication, MiMo Code will prompt you to authenticate when you first try to use it. If not, you can manually trigger the flow with `mimo mcp auth <server-name>`.

#### Pre-registered

If you have client credentials from the MCP server provider, you can configure them:

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "my-oauth-server": {
      "type": "remote",
      "url": "https://mcp.example.com/mcp",
      "oauth": {
        "clientId": "{env:MY_MCP_CLIENT_ID}",
        "clientSecret": "{env:MY_MCP_CLIENT_SECRET}",
        "scope": "tools:read tools:execute"
      }
    }
  }
}
```

#### Authenticating

You can manually trigger authentication or manage credentials.

```bash
# Authenticate with a specific MCP server
mimo mcp auth my-oauth-server

# List all MCP servers and their auth status
mimo mcp list

# Remove stored credentials
mimo mcp logout my-oauth-server
```

The `mcp auth` command will open your browser for authorization. After you authorize, MiMo Code will store the tokens securely in `~/.local/share/mimocode/mcp-auth.json`.

#### Disabling OAuth

If you want to disable automatic OAuth for a server (e.g., for servers that use API keys instead), set `oauth` to `false`:

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "my-api-key-server": {
      "type": "remote",
      "url": "https://mcp.example.com/mcp",
      "oauth": false,
      "headers": {
        "Authorization": "Bearer {env:MY_API_KEY}"
      }
    }
  }
}
```

#### OAuth Options

| Option         | Type            | Description                                                                      |
| -------------- | --------------- | -------------------------------------------------------------------------------- |
| `oauth`        | Object \| false | OAuth config object, or `false` to disable OAuth auto-detection.                 |
| `clientId`     | String          | OAuth client ID. If not provided, dynamic client registration will be attempted. |
| `clientSecret` | String          | OAuth client secret, if required by the authorization server.                    |
| `scope`        | String          | OAuth scopes to request during authorization.                                    |

#### Debugging

If a remote MCP server is failing to authenticate, you can diagnose issues with:

```bash
# View auth status for all OAuth-capable servers
mimo mcp auth list

# Debug connection and OAuth flow for a specific server
mimo mcp debug my-oauth-server
```

### Manage

Your MCPs are available as tools in MiMo Code, alongside built-in tools. So you can manage them through the MiMo Code config like any other tool.

#### Global

This means that you can enable or disable them globally.

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "my-mcp-foo": {
      "type": "local",
      "command": ["bun", "x", "my-mcp-command-foo"]
    },
    "my-mcp-bar": {
      "type": "local",
      "command": ["bun", "x", "my-mcp-command-bar"]
    }
  },
  "tools": {
    "my-mcp-foo": false
  }
}
```

We can also use a glob pattern to disable all matching MCPs.

```json
{
  "tools": {
    "my-mcp*": false
  }
}
```

#### Per agent

If you have a large number of MCP servers you may want to only enable them per agent and disable them globally. To do this:

1. Disable it as a tool globally.
2. In your agent config, enable the MCP server as a tool.

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "my-mcp": {
      "type": "local",
      "command": ["bun", "x", "my-mcp-command"],
      "enabled": true
    }
  },
  "tools": {
    "my-mcp*": false
  },
  "agent": {
    "my-agent": {
      "tools": {
        "my-mcp*": true
      }
    }
  }
}
```

#### Glob patterns

The glob pattern uses simple regex globbing patterns:

- `*` matches zero or more of any character (e.g., `"my-mcp*"` matches `my-mcp_search`, `my-mcp_list`, etc.)
- `?` matches exactly one character
- All other characters match literally

> **NOTE**: MCP server tools are registered with server name as prefix, so to disable all tools for a server simply use: `"mymcpservername_*": false`

### Examples

#### Sentry

Add the [Sentry MCP server](https://mcp.sentry.dev) to interact with your Sentry projects and issues.

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "sentry": {
      "type": "remote",
      "url": "https://mcp.sentry.dev/mcp",
      "oauth": {}
    }
  }
}
```

After adding the configuration, authenticate with Sentry:

```bash
mimo mcp auth sentry
```

This will open a browser window to complete the OAuth flow and connect MiMo Code to your Sentry account.

Once authenticated, you can use Sentry tools in your prompts to query issues, projects, and error data.

```txt
Show me the latest unresolved issues in my project. use sentry
```

#### Context7

Add the [Context7 MCP server](https://github.com/upstash/context7) to search through docs.

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "context7": {
      "type": "remote",
      "url": "https://mcp.context7.com/mcp"
    }
  }
}
```

If you have signed up for a free account, you can use your API key and get higher rate-limits.

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "context7": {
      "type": "remote",
      "url": "https://mcp.context7.com/mcp",
      "headers": {
        "CONTEXT7_API_KEY": "{env:CONTEXT7_API_KEY}"
      }
    }
  }
}
```

Add `use context7` to your prompts to use Context7 MCP server.

```txt
Configure a Cloudflare Worker script to cache JSON API responses for five minutes. use context7
```

Alternatively, you can add something like this to your AGENTS.md:

```md
When you need to search docs, use `context7` tools.
```

#### Grep by Vercel

Add the [Grep by Vercel](https://grep.app) MCP server to search through code snippets on GitHub.

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "mcp": {
    "gh_grep": {
      "type": "remote",
      "url": "https://mcp.grep.app"
    }
  }
}
```

Since we named our MCP server `gh_grep`, you can add `use the gh_grep tool` to your prompts to get the agent to use it.

```txt
What's the right way to set a custom domain in an SST Astro component? use the gh_grep tool
```

---

## Rules

You can provide custom instructions to mimocode by creating an `AGENTS.md` file. This is similar to Cursor's rules. It contains instructions that will be included in the LLM's context to customize its behavior for your specific project.

### Initialize

To create a new `AGENTS.md` file, you can run the `/init` command in MiMo Code.

> **TIP**: You should commit your project's `AGENTS.md` file to Git.

`/init` scans the important files in your repo, may ask a couple of targeted questions when the codebase cannot answer them, and then creates or updates `AGENTS.md` with concise project-specific guidance.

It focuses on the things future agent sessions are most likely to need:

- build, lint, and test commands
- command order and focused verification steps when they matter
- architecture and repo structure that are not obvious from filenames alone
- project-specific conventions, setup quirks, and operational gotchas
- references to existing instruction sources like Cursor or Copilot rules

If you already have an `AGENTS.md`, `/init` will improve it in place instead of blindly replacing it.

### Example

You can also just create this file manually. Here's an example of some things you can put into an `AGENTS.md` file.

```markdown
# SST v3 Monorepo Project

This is an SST v3 monorepo with TypeScript. The project uses bun workspaces for package management.

## Project Structure

- `packages/` - Contains all workspace packages (functions, core, web, etc.)
- `infra/` - Infrastructure definitions split by service (storage.ts, api.ts, web.ts)
- `sst.config.ts` - Main SST configuration with dynamic imports

## Code Standards

- Use TypeScript with strict mode enabled
- Shared code goes in `packages/core/` with proper exports configuration
- Functions go in `packages/functions/`
- Infrastructure should be split into logical files in `infra/`

## Monorepo Conventions

- Import shared modules using workspace names: `@my-app/core/example`
```

### Types

mimocode also supports reading the `AGENTS.md` file from multiple locations. And this serves different purposes.

#### Project

Place an `AGENTS.md` in your project root for project-specific rules. These only apply when you are working in this directory or its sub-directories.

#### Global

You can also have global rules in a `~/.config/mimocode/AGENTS.md` file. This gets applied across all mimocode sessions.

Since this isn't committed to Git or shared with your team, we recommend using this to specify any personal rules that the LLM should follow.

#### Claude Code Compatibility

For users migrating from Claude Code, MiMo Code supports Claude Code's file conventions as fallbacks:

- **Project rules**: `CLAUDE.md` in your project directory (used if no `AGENTS.md` exists)
- **Global rules**: `~/.claude/CLAUDE.md` (used if no `~/.config/mimocode/AGENTS.md` exists)
- **Skills**: `~/.claude/skills/` — see Agent Skills for details

To disable Claude Code compatibility, set one of these environment variables:

```bash
export MIMOCODE_DISABLE_CLAUDE_CODE=1        # Disable all .claude support
export MIMOCODE_DISABLE_CLAUDE_CODE_PROMPT=1 # Disable only ~/.claude/CLAUDE.md
export MIMOCODE_DISABLE_CLAUDE_CODE_SKILLS=1 # Disable only .claude/skills
```

### Precedence

When mimocode starts, it looks for rule files in this order:

1. **Local files** by traversing up from the current directory (`AGENTS.md`, `CLAUDE.md`)
2. **Global file** at `~/.config/mimocode/AGENTS.md`
3. **Claude Code file** at `~/.claude/CLAUDE.md` (unless disabled)

The first matching file wins in each category. For example, if you have both `AGENTS.md` and `CLAUDE.md`, only `AGENTS.md` is used. Similarly, `~/.config/mimocode/AGENTS.md` takes precedence over `~/.claude/CLAUDE.md`.

### Custom Instructions

You can specify custom instruction files in your `mimocode.json` or the global `~/.config/mimocode/mimocode.json`. This allows you and your team to reuse existing rules rather than having to duplicate them to AGENTS.md.

Example:

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "instructions": [
    "CONTRIBUTING.md",
    "docs/guidelines.md",
    ".cursor/rules/*.md"
  ]
}
```

You can also use remote URLs to load instructions from the web.

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "instructions": [
    "https://raw.githubusercontent.com/my-org/shared-rules/main/style.md"
  ]
}
```

Remote instructions are fetched with a 5 second timeout.

All instruction files are combined with your `AGENTS.md` files.

### Referencing External Files

While mimocode doesn't automatically parse file references in `AGENTS.md`, you can achieve similar functionality in two ways:

#### Using mimocode.json

The recommended approach is to use the `instructions` field in `mimocode.json`:

```json
{
  "$schema": "https://mimo.xiaomi.com/mimocode/config.json",
  "instructions": [
    "docs/development-standards.md",
    "test/testing-guidelines.md",
    "packages/*/AGENTS.md"
  ]
}
```

#### Manual Instructions in AGENTS.md

You can teach mimocode to read external files by providing explicit instructions in your `AGENTS.md`. Here's a practical example:

```markdown
# TypeScript Project Rules

## External File Loading

CRITICAL: When you encounter a file reference (e.g., @rules/general.md), use your Read tool to load it on a need-to-know basis. They're relevant to the SPECIFIC task at hand.

Instructions:

- Do NOT preemptively load all references - use lazy loading based on actual need
- When loaded, treat content as mandatory instructions that override defaults
- Follow references recursively when needed

## Development Guidelines

For TypeScript code style and best practices: @docs/typescript-guidelines.md
For React component architecture and hooks patterns: @docs/react-patterns.md
For REST API design and error handling: @docs/api-standards.md
For testing strategies and coverage requirements: @test/testing-guidelines.md

## General Guidelines

Read the following file immediately as it's relevant to all workflows: @rules/general-guidelines.md.
```

---

_This documentation was scraped from https://mimo.xiaomi.com/mimocode/start on 2026-06-30._
