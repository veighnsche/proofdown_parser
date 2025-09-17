# BDD: Gherkin & Cucumber Artifacts

Behavior-Driven Development (BDD) uses Gherkin feature files to specify behavior in business-readable language. Execution produces machine-readable results that CI can collect.

## Common producers

- Cucumber (JVM/JS/Ruby), Behave (Python), SpecFlow (.NET), Behat (PHP)
- Cucumber-compat runners in Cypress/Playwright via plugins

## Artifact types

- Specifications: `.feature` files (plain text Gherkin)
- Execution results: Cucumber JSON, JUnit XML, HTML reports
- Attachments: screenshots, logs (via formatter attachments)

## Formats and typical files

- Gherkin Features: `*.feature`
- Cucumber JSON: `cucumber.json` (consolidated) or `cucumber-*.json`
- JUnit XML: `TESTS-*.xml`
- HTML report folders (framework-specific)

Cucumber JSON (schema evolves; example excerpt):
```json
{
  "uri": "features/login.feature",
  "elements": [
    {
      "name": "Successful login",
      "steps": [
        { "name": "Given I am on the login page", "result": { "status": "passed", "duration": 123456 } }
      ]
    }
  ]
}
```

## Proofdown viewer mapping

- `.feature` specs → render as text via `artifact.markdown` (if pre-rendered) or link via `artifact.link`; consider also showing parsed outline as `artifact.json`.
- Cucumber JSON → `artifact.json` (collapsed) + `artifact.table` for scenario/step rollups.
- JUnit XML → convert to JSON summary → `artifact.json`; preserve XML via `artifact.link`.
- Screenshots → `artifact.image` (grid of failing steps); Logs → `artifact.markdown` or `artifact.link`.

## Implications for Proofdown Language Spec

- Do not add Gherkin syntax to the language; treat `.feature` files as artifacts (text or link) to keep grammar minimal.
- Standardize scenario/step rollups through `artifact.table` conventions (columns such as feature, scenario, status, duration).
- Encourage JSON summaries over HTML reports for deterministic rendering; large HTML reports via `artifact.link`.
- Attach step-level screenshots with `artifact.image` inside a `grid`/`card` layout; no new block types needed.
- Enforce digest-addressed references and collapsed JSON by default for large suites.

## Related specs and schemas

- Proofdown v1 grammar: ../../.specs/01_proofdown_language_v1.md
- Artifact-first semantics: ../../.specs/02_artifact_first_language_spec.md
- Proofdown v2 (additive): ../../.specs/03_proofdown_language_v2.md
- Authoring guide: ../proofdown-authoring-guide.md
- Table schema (bdd): ../../.specs/schemas/bdd.schema.json

## References

- Gherkin reference: https://cucumber.io/docs/gherkin/reference/
- Gherkin overview: https://cucumber.io/docs/gherkin/
- Cucumber reporting: https://cucumber.io/docs/cucumber/reporting/
- Cucumber JSON schema (legacy/ongoing): https://github.com/cucumber/cucumber-json-schema
