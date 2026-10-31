# ARCHITECTURE

## Purpose
Fixture workspace; the AES layer contract is deliberately stripped here so the scanner finds its shape violations.

## Workspace Organization
Members live under crates/, modules/, and packages/; each carries FRD.md and BACKLOG.md.

## Naming Convention
Crate names use the `<domain>-lint-arwaky` suffix scheme the root workspace uses.

## Vertical Slicing Layout
Every feature is a vertical slice crossing the layers listed below.

## Taxonomy Layer
crates/shared defines the VOs every other layer names.

## Contract Layer
Rules crates expose narrow protocol traits; nothing reaches across features.

## Utility Layer
Filesystem and config-system provide the IO and configuration primitives.

## Capabilities Layer
The dispatcher composes capabilities out of the contract traits.

## Agent Layer
This fixture defines no agent surface.

## Surface Layer
The CLI is the only surface the fixture exposes.

## Root Layer
Root documents (PRD, ROADMAP, AGENTS) pin the state vocabulary.
