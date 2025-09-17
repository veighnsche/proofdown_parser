# End-to-End (Web) Testing Artifacts

End-to-end (E2E) tests drive a real browser to validate end-user flows. Artifacts generally include structured test results, screenshots, videos, HAR/trace files, and HTML reports.

## Common producers

- Cypress (Mocha/JUnit reporters; screenshots/videos folders)
- Playwright Test (JSON/HTML/JUnit reporters; trace.zip files)
- Selenium/WebDriver-based frameworks (JUnit/NUnit/TestNG XML, Allure results)
- WebdriverIO, TestCafe, Nightwatch (various reporters: JSON, JUnit, HTML, Allure)

## Artifact formats

- Test results: JUnit XML, JSON summaries (framework-specific)
- Rich reports: HTML bundles (Allure, Playwright HTML)
- Media: PNG screenshots, MP4/WEBM videos (Cypress), trace archives (Playwright `trace.zip`)
- Logs: stdout/stderr, network HAR files

## Typical files

- `junit.xml`, `TEST-*.xml` — machine-readable results
- `playwright-report/` — self-contained HTML report folder
- `playwright-trace.zip` — interactive trace viewer input
- `cypress/screenshots/**.png` — failure screenshots
- `cypress/videos/**.mp4` — recorded videos

## Proofdown viewer mapping

- Structured results (JSON) → `artifact.json` with collapsed view and rollup `artifact.table`.
- JUnit XML → convert to JSON summary for `artifact.json`; preserve original via `artifact.link`.
- Screenshots → `artifact.image` (limit `max_height` for quick scanning); galleries via `grid/card`.
- Videos and traces → `artifact.link` (download) and link to hosted HTML viewers if available.
- HTML reports → `artifact.link` to the bundle; include a JSON summary alongside.

## Implications for Proofdown Language Spec

- Prefer JSON summaries for test results; keep HTML/trace/video as linked artifacts to preserve deterministic rendering.
- Encourage screenshot galleries using existing blocks (`grid`/`card` + `artifact.image`); no new syntax needed.
- Standardize viewer attributes such as `max_height` and `collapsed` to keep large runs scannable (renderer-level behavior, not parse-time).
- Treat traces (`trace.zip`) and videos as opaque via `artifact.link`; any interactive viewer is allowlisted outside the language grammar.
- Enforce content-addressed artifact references; language stays minimal while SSG normalizes tool-specific formats to JSON.

## Related specs and schemas

- Proofdown v1 grammar: ../../.specs/01_proofdown_language_v1.md
- Artifact-first semantics: ../../.specs/02_artifact_first_language_spec.md
- Proofdown v2 (additive): ../../.specs/03_proofdown_language_v2.md
- Authoring guide: ../proofdown-authoring-guide.md
- Table schema (e2e): ../../.specs/schemas/e2e.schema.json

## References

- Cypress reporters: https://docs.cypress.io/app/tooling/reporters
- Playwright reporters: https://playwright.dev/docs/test-reporters
- Allure result format: https://allurereport.org/docs/how-it-works-test-result-file/
