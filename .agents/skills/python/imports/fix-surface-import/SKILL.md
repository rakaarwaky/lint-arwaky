---
name: fix-surface-import
version: 1.0.0
category: refactoring
tags: [aes, surface, import, aes201]
triggers:
  - "fix surface import"
  - "surface import violation"
dependencies: []
related:
  - module_logic_validator
---

# fix-surface-import

## Rules

- Surfaces MUST use aggregate contracts via DI
- Surfaces MUST NOT import capabilities or infrastructure directly

## Purpose

Prevent CLI/web controllers from importing capabilities/infrastructure directly.

## When to Use

- Surface file imports `capabilities_*` or `infrastructure_*`

## The Fundamental Question

> **"Does this surface import concrete classes?"**

If yes -> **Use aggregate contract via DI**

## Workflow

### Step 1: Find Forbidden Imports

Read surface file, ask: Does it import capabilities_ or infrastructure_?

### Step 2: Replace with Aggregate Contract

Change import to aggregate contract.

### Step 3: Wire via DI Container

Wire dependencies through container.