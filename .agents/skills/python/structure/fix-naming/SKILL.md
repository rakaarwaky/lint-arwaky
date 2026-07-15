# fix-naming

## Rules


| Layer          | Pattern                  | Suffix                          |
| ---------------- | -------------------------- | --------------------------------- |
| root           | `root_*_container.py`    | `_container`                    |
| taxonomy       | `taxonomy_*_vo.py`       | `_vo`, `_constant`              |
| contract       | `contract_*_protocol.py` | `_protocol`, `_port` _agregate |
| capabilities   | `capabilities_*.py`      | flexible                        |
| infrastructure | `infrastructure_*.py`    | flexible                        |

## Purpose

Rename files to follow `prefix_concept_suffix` pattern with correct layer suffix.

## When to Use

- File name doesn't follow convention
- Wrong suffix for layer type

## The Fundamental Question

> **"Does this file name follow the pattern?"**

Pattern: `prefix_concept_suffix.py`

## Workflow

### Step 1: Check Current Name

Compare file name against naming rules.

### Step 2: Rename File

Use `git mv` to rename file.

### Step 3: Update All Imports

Update all files that import from this file.
