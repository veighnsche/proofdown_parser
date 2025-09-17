# Proofdown Table Row Schemas

This directory hosts minimal, informative JSON Schemas for common `artifact.table` kinds defined in the Proofdown v2 spec. They document the expected shape of row-oriented summaries used for deterministic, interoperable rendering.

- Audience: producers (SSGs, CI jobs) and validators.
- Status: informative; engines may validate against stricter local schemas.

## How to use

- When emitting summary JSON to be consumed by `artifact.table`, prefer one of these shapes.
- Keep fields stable across runs/tools. Extra fields are allowed (schema uses `additionalProperties: true` per row).
- Pair with `artifact.json` for a collapsed tree and `artifact.link` for heavy/HTML artifacts.
- Use the `kind` attribute on `artifact.table` to signal which schema you target.

## Schemas

- Unit tests: `unit_tests.schema.json`
- Coverage: `coverage.schema.json`
- Security: `security.schema.json`
- E2E: `e2e.schema.json`
- IaC: `iac.schema.json`
- A11y: `a11y.schema.json`
- Performance: `performance.schema.json`
- SBOM: `sbom.schema.json`
- Provenance: `provenance.schema.json`
- DB migrations: `db_migrations.schema.json`
- Lint: `lint.schema.json`
- Contracts: `contracts.schema.json`
- Fuzzing: `fuzzing.schema.json`
- Mobile: `mobile.schema.json`
- Mutation: `mutation.schema.json`
- Snapshots: `snapshots.schema.json`
- API/Integration: `api.schema.json`
- BDD: `bdd.schema.json`
- Visual: `visual.schema.json`

## References

- Spec v2: ../03_proofdown_language_v2.md
- Artifact-first semantics: ../02_artifact_first_language_spec.md
- Authoring guide: ../../.docs/proofdown-authoring-guide.md
