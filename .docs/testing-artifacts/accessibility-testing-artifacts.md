# Accessibility (A11y) Testing Artifacts

Automated accessibility tools scan pages and components for rule violations (WCAG, ARIA). Artifacts are typically JSON results with per-rule details and node-level evidence; some tools also emit HTML reports.

## Common producers

- axe-core (Deque) — JSON results; adapters for Cypress/Playwright/Selenium
- Lighthouse — accessibility category in JSON/HTML
- Pa11y — JSON/HTML reports
- WAVE API, SonarQube a11y rules

## Formats

- axe-core JSON: `violations`, `passes`, `incomplete` arrays with node selectors and HTML snippets
- Lighthouse JSON: `categories.accessibility` score and audits
- Pa11y JSON/HTML

## Examples

axe-core JSON (excerpt):
```json
{
  "violations": [
    {
      "id": "color-contrast",
      "impact": "serious",
      "nodes": [ { "html": "<a>...</a>", "target": ["#cta"], "failureSummary": "Fix contrast" } ]
    }
  ]
}
```

## Proofdown viewer mapping

- JSON results → `artifact.json` (collapsed) with `artifact.table` summarizing counts by impact/severity.
- HTML dashboards → `artifact.link` with a short summary table.
- Screenshots of failing nodes (if captured) → `artifact.image` alongside JSON evidence.

## Implications for Proofdown Language Spec

- Keep rendering deterministic: prefer JSON summaries and avoid embedding HTML widgets; link dashboards via `artifact.link`.
- Provide standard table rollups (rule id, impact/severity, affected pages/elements) via `artifact.table`; implement interactivity in renderer, not grammar.
- Encourage masking/redaction of sensitive HTML snippets in JSON before display; treat raw HTML as data and escape at render time.
- Large violation sets should default to collapsed JSON with optional `depth`; screenshots should be bounded with `max_height`.
- Maintain minimal grammar; the parser remains syntax-only while SSG normalizes axe/Lighthouse/Pa11y outputs.

## Related specs and schemas

- Proofdown v1 grammar: ../../.specs/01_proofdown_language_v1.md
- Artifact-first semantics: ../../.specs/02_artifact_first_language_spec.md
- Proofdown v2 (additive): ../../.specs/03_proofdown_language_v2.md
- Authoring guide: ../proofdown-authoring-guide.md
- Table schema (a11y): ../../.specs/schemas/a11y.schema.json

## References

- axe-core API: https://github.com/dequelabs/axe-core/blob/develop/doc/API.md
- axe DevTools JSON reporting: https://docs.deque.com/devtools-for-web/4/en/node-reporter/
- Lighthouse: https://developer.chrome.com/docs/lighthouse/overview/
- Pa11y: https://pa11y.org/
