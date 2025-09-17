# Why Proofdown — Philosophy and Purpose

Make proofs the product. Put specs, contracts, tests, artifacts, and documented proof ahead of code.

Proofdown is a purpose-built markup for rendering and reviewing testing evidence that a build actually produced. It exists because general-purpose documentation formats are too permissive and too ambiguous to support verifiable, deterministic, tamper‑evident proof pages authored at AI speed.

## Related documents

- Language v1: [../.specs/01_proofdown_language_v1.md](../.specs/01_proofdown_language_v1.md)
- Artifact-first semantics: [../.specs/02_artifact_first_language_spec.md](../.specs/02_artifact_first_language_spec.md)
- Language v2 (additive): [../.specs/03_proofdown_language_v2.md](../.specs/03_proofdown_language_v2.md)
- Authoring guide: [./proofdown-authoring-guide.md](./proofdown-authoring-guide.md)
- Testing artifacts catalog: [./testing-artifacts/README.md](./testing-artifacts/README.md)
- Table row schemas (JSON): [../.specs/schemas/README.md](../.specs/schemas/README.md)

## The trust gap (and why documentation isn’t enough)

AI-generated code is pervasive. Velocity is no longer the bottleneck—trust is. Traditional documentation, even when thorough, is:

- Loose: Markdown and friends allow near-infinite dialects and renderer-specific behaviors. The same text does not always mean the same thing.
- Impure: HTML injection, templating, and hidden scriptable surfaces make rendering non-deterministic and unsafe in adversarial contexts.
- Decoupled: Docs often claim “what happened” without binding to the exact artifacts a CI run produced.
- Ephemeral: Links rot, assets move, and what the reader sees can drift from what the build emitted.

When code is authored or modified by AI, these failure modes compound. We need a medium that carries proof—not just prose.

## The minimal, verifiable contract

Provenance (the system) centers a signed Index that enumerates which artifacts exist for a commit and addresses each by digest. A worker verifies the Index signature and each artifact’s digest before rendering.

Proofdown (the language) is the human-facing layer that:

- References artifacts by their digests, not by mutable paths.
- Presents claims, tests, and evidence in a structure optimized for review.
- Is deterministic to parse and deterministic to render.
- Is safe by construction—no scripting, no HTML injection surfaces, and a constrained set of viewers.

Together, these properties let readers verify “what CI produced” and reviewers assess “does the evidence support the claim?”

## Why a new markup?

Could we just constrain Markdown? In practice, no:

- Markdown is under-specified. Dialects differ on lists, code fences, links, extensions, and HTML passthrough. Tightening it enough to be deterministic yields a different language.
- Deterministic ASTs matter. We need an AST that is stable across engines, as the AST drives validation, rendering, and policy enforcement.
- Safety first. Proofdown forbids entire classes of footguns (HTML, script, inline styles) rather than enumerating exceptions.
- Contract awareness. Artifact references and result blocks must be explicit, typed, and machine-checkable against the signed Index.

Proofdown is small on purpose. Minimal grammar, maximal determinism.

## Ethos and priority order

SPECS → CONTRACTS → TESTING ARTIFACTS → DOCUMENTED PROOF → CODE

We prioritize verifiable evidence and human-readable documentation over implementation. AI-authored code must ship with artifacts and a reviewable proof page, or it does not ship.

## Design goals

- Determinism: A given source must parse to exactly one AST and render identically across compliant engines.
- Safety: No scriptable surfaces. No raw HTML. Strict escaping rules. Constrained viewers for binary or rich artifacts.
- Artifact-first: All evidence is content-addressed. The language prefers digest-addressed links and fails closed on mismatch.
- Human review optimized: Emphasize claims, assumptions, tests, and outcomes in a scan-friendly structure.
- AI-writable, human-editable: Grammar that is simple, repetitive, and hard to misuse. Diff-friendly and idempotent under reformatting.
- Reproducibility: Canonicalization of whitespace, line endings, and headings to make hashing, signing, and caching reliable.
- Versioned and test-driven: Spec versions are explicit. A normative test corpus defines truth; implementations conform by passing it.
- Composability: Small set of block types that compose without surprising interactions.
- Accessibility and longevity: Rendered output must remain legible and navigable without client-side JavaScript.

## Non-goals

- Not a general web publishing system. Complex layout, theming, and interactivity live outside Proofdown.
- Not a templating language. No variable substitution, loops, or conditionals.
- Not HTML with rules. HTML passthrough and inline styling are out of scope.
- Not CI. Proofdown documents present evidence; they do not run tests.
- Not a replacement for specifications. Proofdown hosts and links to them; it does not define them.

## Tenets that shape the syntax

- Canonical headings: One space after the marker, bounded depth, and explicit constraints to avoid dialect drift.
- Bounded lists: Clear rules to avoid ambiguous list continuation and nesting edge cases.
- Typed references: Artifact links declare their type and intended viewer; verification happens before render.
- Evidence blocks: Structured containers for test results and attachments, with optional status and summary.
- Minimal inline features: Bold, italics, code spans, and links—no surprises, no hidden HTML.
- Strict errors: Fail fast on ambiguity. Warns are rare by design; either it parses or it doesn’t.

## Safety and verification model

- All external references are content-addressed or blocked.
- Viewers are allowlisted and parameterized; no user-provided scripts.
- Rendering is pure: the same input and artifact set always produce the same output.
- The signed Index is the source of truth. Documents cannot reference artifacts the Index does not enumerate.

## Authoring model (humans and AI)

- Predictable patterns: Repeated, easy-to-imitate structures make AI output consistent and reviewable.
- Guardrails over heuristics: The parser encodes the rules so authors don’t need to memorize dialect quirks.
- Diffable edits: Local, line-based edits should not reflow unrelated content. Canonical formatting supports stable diffs.
- Review posture: Claims lead, evidence follows, and open questions or assumptions are explicit.

## Interoperability and scope boundaries

- Parser and AST are versioned and tested. The parser in this repository is maintained as an external submodule; changes follow the spec and the test corpus.
- Rendering is intentionally boring. The SSG integrates viewer feature-gating and artifact verification; Proofdown supplies the AST and semantics.
- Cloud deployment verifies first, then shows. The worker refuses to render on digest or signature mismatch.

## Success criteria

- A reviewer can load a proof page and trust that every artifact shown is exactly what CI produced.
- AI systems can generate valid Proofdown reliably using a small set of patterns.
- Implementations converge because the spec is minimal, the AST is deterministic, and the test corpus is normative.
- Over time, most conversation moves from “does it compile?” to “does the evidence prove the claim?”

## Appendix: guardrails for evolution

- Backwards compatibility by default; explicit spec version gates breaking changes.
- Every grammar rule carries at least one normative test.
- Extensions are proposed as viewers or block types with clear safety and determinism analysis—not ad hoc syntax.
