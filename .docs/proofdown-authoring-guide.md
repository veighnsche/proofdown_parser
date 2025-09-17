# Authoring Proofdown for CI Artifacts — LLM‑Friendly Patterns

Make proofs the product. This guide shows the best, AI‑authorable way to present CI/testing artifacts in Proofdown with deterministic, safe, and reviewer‑friendly patterns.

It distills the artifact survey in `.docs/testing-artifacts/` and the semantics in `.specs/02_artifact_first_language_spec.md` into concrete templates and do/don’t rules.

## Related specs and schemas

- v1 grammar: [../.specs/01_proofdown_language_v1.md](../.specs/01_proofdown_language_v1.md)
- v2 (additive semantics & viewers): [../.specs/03_proofdown_language_v2.md](../.specs/03_proofdown_language_v2.md)
- Artifact-first semantics: [../.specs/02_artifact_first_language_spec.md](../.specs/02_artifact_first_language_spec.md)
- Table schemas (JSON): [../.specs/schemas/](../.specs/schemas/)


## Core principles

- Prefer JSON summaries for inline views; link large/interactive bundles.
- Use the trio: `artifact.json` + `artifact.table` + `artifact.link`.
- Compose with `grid` + `card` for scan‑friendly sections.
- Keep JSON collapsed by default (`collapsed=true`, small `depth`).
- Always provide `alt` for images; bound image size with `max_height`.
- Reference artifacts by Index `id` (content‑addressed) — never by mutable paths.
 - Use `json_pointer` on `artifact.json` to focus large documents to a subtree.
 - For short plaintext excerpts (logs/SQL), use `artifact.text` with `max_lines`.


## Quick start templates

### Minimal evidence panel

```pml
# QA Evidence — {{ commit }}

<grid cols=3 gap=16>
  <card title="Unit tests">
    <artifact.json id="tests-summary.json" collapsed=true depth=3 />
    <artifact.table id="tests-summary.json" columns="suite,name,status,duration_ms" sort_by="status" />
  </card>
  <card title="Coverage">
    <artifact.json id="coverage-summary.json" collapsed=true depth=2 />
    <artifact.table id="coverage-summary.json" columns="package,lines_pct,branches_pct" sort_by="lines_pct" />
    <artifact.link id="coverage-html.zip" title="Open full HTML report" />
  </card>
  <card title="Key failures">
    <artifact.image id="login-diff.png" alt="Login visual diff" max_height=480 />
  </card>
</grid>
```

### E2E run with screenshots and traces

```pml
## E2E — Checkout Flow

<grid cols=2 gap=16>
  <card title="Results">
    <artifact.json id="e2e-summary.json" collapsed=true depth=2 />
    <artifact.table id="e2e-summary.json" columns="spec,scenario,status,duration_ms" sort_by="status" />
    <artifact.link id="playwright-report.zip" title="Open Playwright HTML report" />
    <artifact.link id="playwright-trace.zip" title="Download trace.zip" />
  </card>
  <card title="Failed screenshots">
    <artifact.image id="checkout-step3-diff.png" alt="Checkout step 3 diff" max_height=480 />
    <artifact.image id="order-summary-diff.png" alt="Order summary diff" max_height=480 />
  </card>
</grid>
```

### Security and SBOM rollups

```pml
## Security & Supply Chain

<grid cols=2 gap=16>
  <card title="Code scanning (SARIF)">
    <artifact.json id="sarif-summary.json" collapsed=true depth=2 />
    <artifact.table id="sarif-summary.json" columns="rule,severity,file,count" sort_by="severity" />
  </card>
  <card title="SBOM & Provenance">
    <artifact.json id="sbom-summary.json" collapsed=true depth=2 />
    <artifact.table id="sbom-summary.json" columns="ecosystem,count" />
    <artifact.json id="provenance.json" collapsed=true depth=1 />
    <artifact.link id="sbom.cyclonedx.json" title="Full CycloneDX" />
  </card>
</grid>
```


## Category snippets (copy/paste)

- Unit tests: `tests-summary.json` + `artifact.table(suite,name,status,duration_ms)`; link `TEST-*.xml` if needed.
- Coverage: `coverage-summary.json` + `artifact.table(package,lines_pct,branches_pct)`; link `coverage-html.zip`.
- E2E: `e2e-summary.json` + screenshots; link `report.zip`, `trace.zip`.
- Visual: triplets with `artifact.image` in a `grid` (baseline, actual, diff); link hosted dashboards.
- BDD: link `.feature` files; show `cucumber-summary.json` + rollups.
- API/Integration: `api-summary.json` + selected request/response JSON; link HAR.
- Performance: `perf-summary.json` + endpoint table; link dashboards and time series.
- SAST/DAST/SCA: `sarif-summary.json` + rule/severity rollups; link HTML/PDF if needed.
- Lint: `lint-summary.json` + rule/severity rollups.
- Contracts: link Pact/OpenAPI; show `contract-verification.json`.
- Fuzzing: `fuzz-summary.json` + links to reproducers; link `llvm-cov` HTML.
- Mobile: `.xcresult` as link + `mobile-summary.json`; screenshots grid.
- Accessibility: `a11y-summary.json` + counts by impact; failing node screenshots.
- Mutation: `mutation-summary.json` + outcomes; link HTML report.
- Snapshots: `snapshots-summary.json` (added/changed/removed) + image diffs.
- SBOM/Provenance: summaries + links to full JSON.
- Chaos: `chaos-summary.json` + verdicts; link dashboards.
- Data quality/ML: validations/evaluations; confusion/ROC/PR images; link HTML.
- IaC/Policy: plan/state diffs + scanner summary; link full plan/state.
- DB migrations: short SQL via `artifact.markdown`; link full scripts; TAP/JUnit summary.


## Naming and structure conventions

- Use explicit component titles: short nouns (“Unit tests”, “Coverage”, “Security”).
- JSON artifact names end with `-summary.json` for inline views; raw sources are linked separately.
- Prefer stable column names that map across tools (e.g., `suite`, `name`, `status`, `duration_ms`).
- Keep pages short: 2–3 cards per row, small `depth`, and bounded image sizes.


## Do and Don’t (LLM‑oriented)

- Do keep headings top‑level only (`#`, `##`) and use `grid` + `card` for layout.
- Do prefer `artifact.json` + `artifact.table` for evidence; use `artifact.link` for heavy or interactive assets.
- Do add `alt` to every `artifact.image`; keep `max_height` reasonable (e.g., 480).
- Do collapse large JSON and set `depth` ≤ 3.
- Don’t inline HTML dashboards, videos, or large logs — always link.
- Don’t invent new syntax; stick to the minimal component set.


## LLM prompting tips

Use direct, constrained prompts that enumerate exactly what to emit. Example:

```
Write Proofdown that:
- Uses a 3‑column grid with cards titled Unit tests, Coverage, Security
- Adds artifact.json + artifact.table for tests-summary.json (columns: suite,name,status,duration_ms)
- Adds artifact.json + artifact.table for coverage-summary.json (columns: package,lines_pct,branches_pct)
- Adds artifact.json + artifact.table for sarif-summary.json (columns: rule,severity,file,count)
- Links coverage-html.zip
- Uses collapsed=true and small depth (2–3) for all artifact.json
- Adds alt text and max_height=480 to images if any
```


## Why this pattern works (external best practices)

- CI platforms increasingly standardize on JSON reports for machine consumption and UI rendering (e.g., GitLab artifacts reports, Playwright JSON reporters).
- Security/code scanning expects SARIF JSON (e.g., GitHub Code Scanning) for deterministic ingestion and UI.
- Coverage and test results benefit from small JSON summaries (Istanbul/nyc, JaCoCo summaries) with optional HTML bundles for drilling down.

References:
- GitLab artifacts reports: https://docs.gitlab.com/ci/yaml/artifacts_reports/
- Unit test reports (JUnit): https://docs.gitlab.com/ci/testing/unit_test_reports/
- Playwright reporters: https://playwright.dev/docs/test-reporters
- SARIF support (GitHub Code Scanning): https://docs.github.com/en/code-security/code-scanning/integrating-with-code-scanning/sarif-support-for-code-scanning
- CTRF JSON test results schema: https://medium.com/@ma11hewthomas/its-finally-here-a-json-test-results-data-format-f485b77bbdbc
 - Proofdown table schemas: ../.specs/schemas/


## Checklist (paste into PRs)

- Headings present, cards titled, and layout readable on narrow screens.
- All inline JSON is collapsed with reasonable `depth`.
- Tables use clear, consistent columns and include limits on row count.
- Heavy/interactive artifacts are links, not inline.
- Images include `alt` and have bounded `max_height`.
- All artifact `id`s resolve to signed, digest‑addressed entries in the Index.
