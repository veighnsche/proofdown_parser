# TODO — schema_check

Scope: Validate Proofdown table schemas (JSON Schema files) for basic correctness in CI and optionally perform deeper validation where safe. Keep this crate fast, non-networked, and deterministic.

## Now / Next / Later

- [x] NOW: Improve checks without external network or heavy dependencies
  - [x] Validate that `$schema` is present and references Draft 2020-12 or 2019-09
  - [x] Validate that `type: "object"` root exists and `properties` is an object
  - [x] Check that `required` keys (if present) are a subset of `properties`
  - [x] Warn (non-fatal) on unknown top-level keys (e.g., typos)
  - [x] Add `--strict` flag to turn warnings into errors (optional)
- [ ] NEXT: `$ref` and composition (local-only)
  - [ ] Support local `$ref` resolution within the same document (`#/$defs/...`)
  - [ ] Validate `$defs` objects and resolve simple circular references (detect cycles)
  - [ ] Composition keywords: `allOf`, `oneOf`, `anyOf` (structure only; not full evaluation)
- [ ] LATER: JSON Schema evaluation (optional)
  - [ ] Add optional feature `full` using `jsonschema` crate for local evaluation only
  - [ ] Disallow network fetches; no remote `$ref` resolution
  - [ ] CI job to run strict validation on PRs if feature is enabled

## CLI & UX

- [x] Accept directory path and print a summary
- [x] Add `--json` output with file-by-file status
- [x] Add `--strict` mode to make warnings fatal (exit non-zero)
- [x] Provide counts: files seen, passed, warned, failed

## Tests

- [ ] Unit tests for `$schema` presence and draft checks
- [ ] Tests for `properties` shape and `required` subset
- [ ] Tests for `$defs` structure and local `$ref` basics
- [ ] Tests for warning vs. strict behavior

## CI

- [x] Run schema checks in main workflow
- [ ] Add a separate job for `--strict` on default schemas (when ready)

## Docs

- [ ] Document intended scope and non-goals (no network, deterministic)
- [ ] Provide examples of passing/failing schemas
- [ ] Explain how this interacts with Proofdown validator (advisory only)

## Security & Safety

- [x] No network calls; local files only
- [x] Deterministic processing order (walkdir)
- [ ] Memory/time limits for pathological JSON sizes

## Non-goals

- No remote `$ref` resolution.
- No schema evaluation with side effects.
