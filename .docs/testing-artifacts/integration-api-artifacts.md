# Integration & API Testing Artifacts

Integration and API tests validate service contracts, workflows, and data exchange across components. Artifacts range from machine-readable result files to request/response captures and contract assertions.

## Common producers

- Postman + Newman (CLI) — JSON, JUnit, HTML via reporters
- REST Assured (JVM) — JUnit XML, Allure results
- SuperTest, Frisby, Chakram (Node.js) — Mocha/Jest reporters (JUnit, JSON)
- Karate — Cucumber JSON, JUnit XML, HTML
- Dredd — API Blueprint/OpenAPI validation logs & JSON
- Tavern (Python) — JUnit XML, logs

## Artifact types

- Test results: JUnit XML, JSON summaries per collection/spec
- HTTP captures: HAR files, custom JSON bodies, cURL logs
- Contract validation logs: mismatches, schema diffs
- HTML dashboards: Allure, custom reporter HTML

## Typical files

- `newman/*.json`, `newman/*.junit.xml`
- `karate-reports/*.json`, `karate-summary.html`
- `TEST-*.xml` JUnit/NUnit outputs
- `*.har` request/response archives

## Example: Newman JSON (excerpt)
```json
{
  "run": {
    "stats": { "tests": { "total": 12, "failed": 1 } },
    "failures": [ { "source": "Login.postman_collection.json", "error": { "name": "AssertionError" } } ]
  }
}
```

## Proofdown viewer mapping

- Results JSON → `artifact.json` (collapsed) + `artifact.table` rollups per collection/suite.
- JUnit XML → convert to JSON summary → `artifact.json`; preserve XML via `artifact.link`.
- HAR and large payloads → `artifact.link`; include selected examples as `artifact.json` with redactions.
- HTML dashboards (Allure/Karate) → `artifact.link` to bundle + a summary table.

## Implications for Proofdown Language Spec

- Normalize diverse API runners to JSON summaries; keep the grammar free of tool-specific constructs.
- HAR, trace, and large payload bodies should be linked, not inlined, to preserve deterministic rendering and size limits.
- Encourage standardized rollups (collection, request, status, duration) via `artifact.table` conventions; renderer-level behavior, not grammar.
- Allow redacted examples in `artifact.json` to show representative request/response bodies; keep full artifacts behind `artifact.link`.
- Enforce digest-addressed artifacts and default-collapsed JSON for large suites.

## Related specs and schemas

- Proofdown v1 grammar: ../../.specs/01_proofdown_language_v1.md
- Artifact-first semantics: ../../.specs/02_artifact_first_language_spec.md
- Proofdown v2 (additive): ../../.specs/03_proofdown_language_v2.md
- Authoring guide: ../proofdown-authoring-guide.md
- Table schema (api): ../../.specs/schemas/api.schema.json

## References

- Newman built-in reporters: https://learning.postman.com/docs/collections/using-newman-cli/newman-built-in-reporters/
- Newman (npm): https://www.npmjs.com/package/newman
- Dredd: https://dredd.org/en/latest/
- Karate: https://github.com/karatelabs/karate
