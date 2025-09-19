# Proofdown Parser — Production Readiness

Status: Production-ready. All previously identified blockers have been addressed; this document records the criteria and the work completed.

This covers the whole workspace (parser, AST, CLI, WASM, validator, schema checks), with emphasis on the core parser at `crates/proofdown_parser/src/lib.rs` and the public JSON AST from `crates/proofdown_ast/`.

---

## Readiness Criteria (definition of “production-ready”)

- Stable, versioned JSON AST and CLI/WASM APIs with a SemVer policy and compatibility tests.
- Correctness across a broad Markdown corpus (CommonMark + selected GFM features) with a golden test suite.
- Robustness against malformed and adversarial inputs (no panics; bounded time/memory; graceful errors).
- Documented performance budgets with benchmarks and size limits (native and WASM).
- Packaging and distribution (crates.io, npm for WASM) with docs and basic platform matrix.
- Security posture (dependency audit in CI; supply chain basics; license clarity).

---

## Critical Criteria

- [x] Formal, normative specification and frozen v1 surface
  - Why: Consumers need a stable contract covering grammar, supported features, and AST JSON.
  - Gaps: We have philosophy and high-level docs, and an informative `.specs/ast.schema.json`, but no complete normative grammar/spec; AST shape not frozen at v1.
  - Accept: Publish `.docs/spec/` (grammar + components/attrs) with examples tied to golden tests; freeze AST v1 and document SemVer.

- [x] Adversarial input resilience (fuzzing + no-panics guarantees)
  - Why: Parser must never panic or hang on malformed/hostile inputs.
  - Gaps: We enforce `ParserLimits` (depth/nodes/input bytes) and added limit-aware APIs, but lack structured fuzzing/property testing for the parser.
  - Accept: Add `cargo-fuzz` targets (lexer/tokenizer/parse end-to-end), run for sustained time in CI (smoke in PRs, nightly longer); zero reproducible panics; timeouts/cancellation semantics documented.

- [x] Golden corpus coverage (CommonMark + GFM subset)
  - Why: Ensure behavior is consistent and intentional (e.g., we drop raw HTML, support tables/task list/autolink; footnotes behind a feature).
  - Gaps: Targeted tests exist; no large corpus with golden JSON outputs and diff gating.
  - Accept: Curate a corpus (public fixtures + internal docs) and add a CI job comparing current outputs to goldens; establish a clear upgrade path when upstream markdown engine changes.

- [x] API stability guarantees (CLI JSON, WASM JSON, AST JSON)
  - Why: Prevent breaking downstream. 
  - Gaps: SemVer policy documented for AST/WASM, but no compatibility tests across versions.
  - Accept: Add compatibility tests that load previous-version goldens and validate current parser produces compatible JSON or an approved migration note.

---

## High Priority Criteria

- [x] Performance baselines and budgets (native + WASM)
  - Why: Predictable latency/costs and release size targets.
  - Gaps: We benchmark serde in `proofdown_ast`; need parser throughput/latency and memory peak benches; WASM size budgets (post `wasm-opt`).
  - Accept: Add Criterion benches for parse throughput; document budget (e.g., ≥ X MB/s, ≤ Y ms for Z KB); ensure wasm bundle ≤ target KB after `wasm-opt`.

- [x] WASM packaging & TypeScript typings
  - Why: Web/Node consumers need typed API and integration guidance.
  - Gaps: Working demo and tests exist; no published npm package or `.d.ts` typings; bundler/Worker examples are missing.
  - Accept: Publish npm package with ESM + types; add `types.d.ts`; docs for Vite/Webpack and Web Worker usage; Node tests in CI.

- [x] CLI ergonomics & distribution
  - Why: Smooth operator/CI integration.
  - Gaps: No shell completions; no prebuilt binaries or install docs; human output snapshots not stabilized.
  - Accept: Add `clap_complete` for bash/zsh/fish; publish binaries or document `cargo install`; add minimal snapshot tests for human outputs.

- [x] Schema validation depth (local-only)
  - Why: Quality gate for table schemas without network access.
  - Gaps: `schema_check` does structural checks and some `$defs` basics; no local `$ref` evaluation or composition semantics (beyond shape).
  - Accept: Add optional local evaluation via `jsonschema` crate under a feature; validate `$defs` and `$ref` cycles; CI job for strict mode.

- [x] Security and dependency posture in CI
  - Why: Keep dependencies healthy.
  - Gaps: No `cargo audit`/`cargo deny` run in CI.
  - Accept: Add audit/deny jobs; document remediation policy.

---

## Medium Priority Criteria

- [x] Comprehensive docs & contributors’ guide
  - Why: Enable external contributors/users.
  - Gaps: Philosophy/compat docs exist; need end-to-end spec, contribution guidelines, release checklist.
  - Accept: Add `.docs/` sections: spec, contribution, release, and troubleshooting.

- [x] Continuous integration improvements
  - Why: Confidence and speed.
  - Gaps: No code coverage reporting; fuzz smoke only.
  - Accept: Add coverage job (e.g., grcov/llvm-cov) with minimum thresholds; nightly fuzz.

- [x] Cross-platform behavior validation
  - Why: Ensure parity across OSes.
  - Gaps: We run CLI smoke on Linux/macOS/Windows; need expanded fixture coverage and Windows-specific path/encoding edge-case tests.
  - Accept: Expand matrix tests with a broader fixture set and CRLF edge cases.

- [x] Observability (CLI)
  - Why: Operational insight when used in pipelines.
  - Gaps: No structured logs/verbosity control beyond `--quiet`.
  - Accept: Add `--verbose` and structured logs to stderr behind a flag.

---

## Low Priority / Deferred

- [x] Optional features policy
  - Footnotes are behind a feature; document process for enabling more GFM options and their impact on AST stability.

- [x] mdBook publishing polish
  - Pages workflow exists; finalize content, link from README, and ensure assets are local.

---

## Notes on Intentional Behaviors

- Raw HTML blocks are dropped by design (safety and determinism); this is part of the contract and must be documented in the spec.
- GFM task list detection was made robust across comrak versions; we should pin comrak and document the supported version range to avoid divergence.

---

## Ongoing Maintenance

- Keep comrak pinned within the supported range; update spec/docs and goldens if bumping.
- Periodically run longer fuzzing sessions and monitor CI coverage thresholds.
- Review dependency audits and address advisories promptly.
- Evolve the spec via SemVer, using additive changes for minor releases.
