---
name: fix-bypass-comments
version: 1.0.0
category: refactoring
tags: [aes, bypass, comments, aes304]
triggers:
  - "fix bypass comments"
  - "remove noqa"
  - "remove type ignore"
dependencies: []
related:
  - module_logic_validator
---

# fix-bypass-comments

## Rules

- NO `# type: ignore` allowed
- NO `# noqa` allowed
- NO `unwrap()` allowed
- Fix the root cause instead

## Purpose

Remove `type: ignore`, `noqa`, `unwrap()` and fix the underlying issue.

## When to Use

- File has bypass comments

## The Fundamental Question

> **"Is there a bypass comment?"**

If yes -> **Fix root cause and remove comment**

## Workflow

### Step 1: Find Bypass Comments

Read code and find bypass comments.

### Step 2: Fix Root Cause

Fix underlying type/error.

### Step 3: Remove Comment

Remove the bypass comment.
