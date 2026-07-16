---
name: fix-primitive-to-vo
version: 1.0.0
category: refactoring
tags: [aes, primitive, vo, aes401, aes402]
triggers:
  - "fix primitive to vo"
  - "replace primitive with vo"
dependencies: []
related:
  - fix-import-separation
---

# fix-primitive-to-vo

## Rules

- Entity fields MUST use VOs, not primitives
- Contract signatures MUST use VOs
- VOs MUST validate on construction

## Purpose

Replace `str`, `int`, `float`, `bool` in entity fields and contract signatures with validated VOs.

## When to Use

- Method signature has raw primitives
- Entity field uses primitive type

## The Fundamental Question

> **"Is this a raw primitive?"**

If yes -> **Replace with domain VO**

## Workflow

### Step 1: Find Primitives

Read code and find primitives in signatures/entity fields.

### Step 2: Create/Find VO

Create or find existing VO.

### Step 3: Replace

Replace primitive with VO.
