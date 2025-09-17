# Security Testing Artifacts (SAST, DAST, SCA)

Security testing artifacts include static code analysis findings, dynamic application security test (DAST) reports, and software composition analysis (SCA) vulnerability scans. These artifacts are often consumed by code scanning dashboards and policy engines.

## Common producers

- SAST: CodeQL, Semgrep, SonarQube, ESLint security rules, Bandit (Python), SpotBugs/FindSecBugs (JVM)
- DAST: OWASP ZAP, Burp Suite Enterprise/CI
- SCA/Vuln: Trivy, Grype, Snyk, Dependabot (alerts), Anchore Enterprise

## Formats and typical files

- SARIF 2.1.0 (JSON) — common interchange format for static analysis results
- Tool-native JSON: Semgrep JSON, Trivy JSON, Grype JSON, ZAP JSON
- HTML/PDF dashboards (for human browsing)

Examples:
- SARIF excerpt:
```json
{
  "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
  "version": "2.1.0",
  "runs": [ { "tool": { "driver": { "name": "CodeQL" } }, "results": [ { "ruleId": "js/injection", "level": "error" } ] } ]
}
```

- ZAP JSON excerpt:
```json
{ "site": [{ "alerts": [{ "name": "X-Content-Type-Options Header Missing", "risk": "Low" }]}] }
```

## Proofdown viewer mapping

- SARIF/JSON results → `artifact.json` (collapsed) + `artifact.table` for rule/level/severity rollups.
- Large HTML dashboards → package and publish; link via `artifact.link` and summarize key counts in a table.
- Attach per-rule examples as small JSON snippets (redacted) to aid review.

## Implications for Proofdown Language Spec

- Treat SARIF and tool-native JSON as first-class via `artifact.json`; avoid embedding HTML/PDF to keep rendering deterministic and safe.
- Encourage standardized rollups (rule, severity, file, count) using `artifact.table` with renderer-level options; no new grammar needed.
- Enforce collapsed views and optional `depth` for very large result sets; large attachments remain `artifact.link`.
- Keep parser syntax-only; normalization from SARIF/tool JSON into summaries happens in SSG/validator.
- Prefer digest-addressed artifacts to ensure immutability and verifiability.

## References

- SARIF support (GitHub): https://docs.github.com/en/code-security/code-scanning/integrating-with-code-scanning/sarif-support-for-code-scanning
- Trivy reporting: https://trivy.dev/latest/docs/configuration/reporting/
- Grype outputs: https://github.com/anchore/grype
- OWASP ZAP report formats: https://www.zaproxy.org/docs/desktop/addons/report-generation/report-traditional-json/
- Semgrep: https://semgrep.dev/docs/
- CodeQL: https://codeql.github.com/docs/
- SonarQube analysis: https://docs.sonarsource.com/
