# Performance & Load Testing Artifacts

Performance artifacts capture latency, throughput, error rates, and resource consumption under load. They are used to validate SLAs and to track regressions.

## Common producers

- Apache JMeter — JTL (CSV/XML), HTML dashboard
- Gatling — Static HTML reports, JSON stats
- k6 — real-time JSON stream, summary, custom summaries; JUnit for pass/fail
- Locust — CSV/JSON stats exports
- Artillery — JSON and HTML reports
- Lighthouse — JSON and HTML performance audits (web)
- WebPageTest — JSON/XML run results, filmstrips, videos

## Artifact types

- Metrics time series and aggregates: p50/p90/p95/p99 latencies, RPS, error rates
- Per-endpoint and per-scenario breakdowns
- Trace artifacts: HAR files, request/response samples
- HTML dashboards for exploration

## Typical files

- JMeter: `results.jtl` (CSV or XML), `report/` (HTML)
- Gatling: `target/gatling/<simulation>/index.html`
- k6: `--out json=results.json`, custom summary outputs, JUnit XML
- Lighthouse: `report.json`, `report.html`
- WebPageTest: `results.json`, `video.mp4`

## Examples

JMeter CSV (excerpt):

```
timeStamp,elapsed,label,responseCode,success,bytes
1694800000000,123,GET /api/users,200,true,532
```

k6 JSON (excerpt):

```json
{ "type": "Point", "metric": "http_req_duration", "data": { "value": 120.3 } }
```

## Proofdown viewer mapping

- Summary JSON → `artifact.json` (collapsed) + `artifact.table` for endpoint stats.
- HTML dashboards (JMeter/Gatling/Lighthouse) → `artifact.link` plus a summary table in-page.
- Time series (large) → downsampled JSON for quick view; full datasets via `artifact.link`.
- Videos/filmstrips → `artifact.link`; showcase key frames via `artifact.image`.

## Implications for Proofdown Language Spec

- Time-series data sets can be huge; encourage downsampled/aggregated JSON for `artifact.json` and link full datasets via `artifact.link`.
- Consider renderer-level hints for `artifact.table` (e.g., percentile columns, unit labels) without expanding language grammar.
- Videos and filmstrips remain linked artifacts; prefer small preview images for deterministic pages.
- Lighthouse and other HTML bundles should not be embedded; link bundles and present a small JSON summary to keep output safe.
- Keep parser syntax-only; normalization to common JSON is an SSG responsibility.

## Related specs and schemas

- Proofdown v1 grammar: ../../.specs/01_proofdown_language_v1.md
- Artifact-first semantics: ../../.specs/02_artifact_first_language_spec.md
- Proofdown v2 (additive): ../../.specs/03_proofdown_language_v2.md
- Authoring guide: ../proofdown-authoring-guide.md
- Table schema (performance): ../../.specs/schemas/performance.schema.json

## References

- JMeter JTL: <https://cwiki.apache.org/confluence/display/jmeter/JtlFiles>
- Gatling reports: <https://docs.gatling.io/reference/stats/reports/>
- k6 results output: <https://grafana.com/docs/k6/latest/get-started/results-output/>
- Lighthouse JSON output: <https://developer.chrome.com/docs/lighthouse/overview/>
- WebPageTest API: <https://docs.webpagetest.org/api/reference/>
