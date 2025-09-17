# Testing Artifacts — Catalog and Formats

This directory catalogs common testing artifacts across languages, frameworks, and disciplines. Each page explains what the artifact is, who produces it, typical file formats, and how to present it in Proofdown.

Use it as a reference when mapping CI outputs to Proofdown viewers.

## Related specs and guides

- Proofdown v1 grammar: [../../.specs/01_proofdown_language_v1.md](../../.specs/01_proofdown_language_v1.md)
- Artifact-first semantics: [../../.specs/02_artifact_first_language_spec.md](../../.specs/02_artifact_first_language_spec.md)
- Proofdown v2 (additive): [../../.specs/03_proofdown_language_v2.md](../../.specs/03_proofdown_language_v2.md)
- Authoring guide: [../proofdown-authoring-guide.md](../proofdown-authoring-guide.md)
- Table schemas (JSON): [../../.specs/schemas/](../../.specs/schemas/)

## Index

- [Unit test results](./unit-test-results.md)
- [Code coverage](./coverage-artifacts.md)
- [End-to-end (web) tests](./e2e-web-testing-artifacts.md)
- [Visual regression tests](./visual-testing-artifacts.md)
- [BDD: Gherkin & Cucumber](./bdd-gherkin-cucumber-artifacts.md)
- [Integration & API tests](./integration-api-artifacts.md)
- [Performance & load testing](./performance-load-artifacts.md)
- [Security: SAST/DAST & scanning](./security-sast-dast-artifacts.md)
- [Static analysis & linting](./static-analysis-lint-artifacts.md)
- [Contract testing](./contract-testing-artifacts.md)
- [Fuzzing](./fuzzing-artifacts.md)
- [Mobile (iOS/Android) testing](./mobile-testing-artifacts.md)
- [Accessibility testing](./accessibility-testing-artifacts.md)
- [Mutation testing](./mutation-testing-artifacts.md)
- [Snapshot testing](./snapshot-testing-artifacts.md)
- [Supply chain: SBOM & provenance](./sbom-provenance-artifacts.md)
- [Chaos & resilience testing](./chaos-resilience-artifacts.md)
- [Data quality & ML testing](./data-quality-ml-artifacts.md)
- [IaC & policy-as-code](./iac-policy-artifacts.md)
- [Database migration testing](./database-migration-artifacts.md)

## Proofdown viewer mapping (quick guide)

- JSON-like structured data → `artifact.json` (optionally collapsed/limited depth)
- Tables and matrices → `artifact.table`
- Pre-rendered Markdown → `artifact.markdown`
- Images and screenshots → `artifact.image`
- Binary/HTML bundles or non-supported types → `artifact.link` (download), or render screenshots/JSON summaries alongside

When possible, prefer digest-addressed JSON, CSV, and text formats over HTML for deterministic, safe rendering.
