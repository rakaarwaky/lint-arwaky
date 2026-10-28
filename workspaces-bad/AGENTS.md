---
trigger: always
description: "Gap workspace agent guide."
---
# AGENTS

## Agent Overview
This workspace is a scanner fixture; do not add implementation code.

## Runtime

| Aspect | Detail|
|---|---|
| Language | None; fixture only|
| Environment | No build or test commands are defined|
| Artifacts | None|
| Package Manager | None|
| Version Probe | None|
| Setup | None|
| Activation | None|

## Project Structure

| Path | Purpose|
|---|---|
| crates/ | Member fixture crates|
| modules/ | Member fixture modules|
| packages/ | Member fixture packages|

## Commands

| Category | Command | Scope | CI Gate|
|---|---|---|---|
| Scan | `lint-arwaky-cli docs .` | whole tree | —|

## Documentation Awareness

| Document | Agent Must|
|---|---|
| PRD | Read before touching the fixture documents|

## Reference

| Artifact | Purpose | Link|
|---|---|---|
| PRD | Product requirements | [PRD.md](PRD.md)|
| FRD | Feature requirements | [FRD.md](FRD.md)|
| DATA | Data contracts | [DATA.md](DATA.md)|
| DESIGN | Design contracts | [DESIGN.md](DESIGN.md)|
| ARCHITECTURE | Layer contract | [ARCHITECTURE.md](ARCHITECTURE.md)|
