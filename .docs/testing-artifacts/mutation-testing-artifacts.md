# Mutation Testing Artifacts

Mutation testing flips or removes code constructs to ensure tests fail appropriately, revealing gaps in test suites. Artifacts summarize mutants generated, killed, survived, and timeouts.

## Common producers

- Stryker (JavaScript/TypeScript, .NET, JVM)
- PIT (PITest) for Java
- Infection (PHP)
- Mutmut (Python)

## Formats

- Stryker Mutation Testing JSON (standard schema) + HTML report
- PIT XML + HTML reports

## Example: Stryker JSON (excerpt)
```json
{
  "$schema": "http://stryker-mutator.io/report.schema.json",
  "files": { "src/foo.js": { "language": "javascript", "mutants": [ { "id": "1", "status": "Killed" } ] } },
  "summary": { "killed": 10, "survived": 2, "timeout": 0, "noCoverage": 1 }
}
```

## Proofdown viewer mapping

- Stryker JSON → `artifact.json` (collapsed) with `artifact.table` showing mutant outcomes and mutation score.
- PIT XML → convert to JSON summary for `artifact.json`; link HTML via `artifact.link`.

## Implications for Proofdown Language Spec

- Keep grammar minimal; treat mutation reports as JSON artifacts. Avoid embedding interactive HTML; link full reports.
- Encourage a standard `artifact.table` schema (killed/survived/noCoverage/timeout, score) for quick review; behavior implemented in renderer.
- Large per-file/per-mutant trees should default to collapsed JSON (`collapsed=true`) with optional depth bounds.
- Parser remains syntax-only; SSG normalizes PIT XML and Stryker JSON into stable summaries.
- Prefer digest-addressed artifacts to ensure stable references across runs.

## Related specs and schemas

- Proofdown v1 grammar: ../../.specs/01_proofdown_language_v1.md
- Artifact-first semantics: ../../.specs/02_artifact_first_language_spec.md
- Proofdown v2 (additive): ../../.specs/03_proofdown_language_v2.md
- Authoring guide: ../proofdown-authoring-guide.md
- Table schema (mutation): ../../.specs/schemas/mutation.schema.json

## References

- Stryker report schema: https://github.com/stryker-mutator/mutation-testing-elements/blob/master/packages/report-schema/src/mutation-testing-report-schema.json
- Stryker docs: https://stryker-mutator.io/
- PIT: https://pitest.org/
- Infection: https://infection.github.io/
- Mutmut: https://mutmut.readthedocs.io/
