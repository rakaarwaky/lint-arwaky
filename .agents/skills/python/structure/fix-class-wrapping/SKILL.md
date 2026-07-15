---
name: fix-class-wrapping
version: 1.0.0
category: refactoring
tags: [aes, class, wrapping, aes303]
triggers:
  - "fix class wrapping"
  - "wrap functions in class"
dependencies: []
related:
  - module_logic_validator
---

# fix-class-wrapping

## Rules

- ALL functions must be inside a class
- No module-level `def` statements

## Purpose

Ensure all code lives within class scope (AES303).

## When to Use

- File has module-level `def` outside any class

## The Fundamental Question

> **"Are there standalone functions?"**

If yes -> **Wrap into class**

## Workflow

### Step 1: Find Module-Level Functions 

Read code and find standalone functions.

### Step 2: Wrap into Class 

Wrap into class (static methods if needed).

### Step 3: Verify

```bash
python .agents/skills/structure/fix-class-wrapping/fix_class_wrapping.py modules/animator
```
