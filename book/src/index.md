# The Proofdown Language

Proofdown is a minimal, deterministic markup to present CI/testing evidence as structured, linkable, and reviewable content. It is designed for:

- Determinism: Golden AST JSON guards the grammar and enforces byte-for-byte stability.
- Safety: No scripting or raw HTML; viewer bounds and artifact integrity enforced downstream.
- Authorability: Simple, LLM-friendly patterns with `grid`/`card` composition and `artifact.*` viewers.

This book documents the Proofdown language (v1 grammar) and additive v2 semantics (viewers/attributes), the validator rules, limits, and integration contract with `provenance_ssg`.

Quick links:
- Language Inventory — comprehensive list of constructs and attributes
- Grammar (v1) — headings, paragraphs, components
- Additive Semantics (v2) — pointers, captions, text viewer
- Components and Artifact Viewers — details and constraints
- Limits, Safety, Testing, Schemas
- SSG Contract — stable API and AST JSON for downstream consumers
