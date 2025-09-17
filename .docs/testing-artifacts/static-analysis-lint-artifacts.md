# Static Analysis & Linting Artifacts

Static analysis and linting detect code issues without executing the program. Artifacts usually include rule violations, file/line positions, severities, and suggestions. Many tools support machine-readable formats (JSON, XML, SARIF).

## Common producers

- JavaScript/TypeScript: ESLint, TypeScript compiler diagnostics, SonarJS
- Python: Pylint, Flake8, Bandit, mypy
- JVM: Checkstyle, PMD, SpotBugs/FindBugs
- C/C++: Clang-Tidy, Cppcheck
- .NET: Roslyn analyzers, StyleCop, FxCop
- Ruby: RuboCop; PHP: PHP_CodeSniffer (PHPCS)

## Formats and extensions

- JSON (preferred for machine parsing)
- Checkstyle XML (de facto standard for lints)
- SARIF 2.1.0 JSON (interchange for static analysis)
- JUnit XML (some linters expose violations as test failures)

## Examples

ESLint JSON (excerpt):
```json
[
  {
    "filePath": "src/app.js",
    "messages": [
      { "ruleId": "no-unused-vars", "severity": 2, "message": "'x' is assigned a value but never used.", "line": 10, "column": 7 }
    ],
    "errorCount": 1,
    "warningCount": 0
  }
]
```

Checkstyle XML (excerpt):
```xml
<checkstyle>
  <file name="src/Foo.java">
    <error line="42" column="13" severity="warning" message="'if' is not followed by whitespace" source="com.puppycrawl.tools.checkstyle"/>
  </file>
</checkstyle>
```

## Proofdown viewer mapping

- JSON and SARIF → `artifact.json` (collapsed) with `artifact.table` summarizing counts by rule/severity.
- Checkstyle XML → convert to JSON summary; preserve original via `artifact.link`.
- Large HTML dashboards → `artifact.link`, with a compact issues table in-page.

## Implications for Proofdown Language Spec

- Prefer SARIF/JSON ingestion; avoid HTML passthrough to keep pages deterministic and safe.
- Standardize table rollups (rule, severity, file, count) via `artifact.table` conventions; implement behavior in renderer, not grammar.
- For very large result sets, enforce `collapsed=true` and optional `depth` to maintain usability within size limits.
- Keep parser syntax-only; SSGs normalize tool formats (ESLint JSON, Checkstyle XML, SARIF) to summaries.
- Encourage digest-addressed artifacts (immutable) to ensure verifiable evidence.

## Related specs and schemas

- Proofdown v1 grammar: ../../.specs/01_proofdown_language_v1.md
- Artifact-first semantics: ../../.specs/02_artifact_first_language_spec.md
- Proofdown v2 (additive): ../../.specs/03_proofdown_language_v2.md
- Authoring guide: ../proofdown-authoring-guide.md
- Table schema (lint): ../../.specs/schemas/lint.schema.json

## References

- ESLint formatters: https://eslint.org/docs/latest/use/formatters/
- Checkstyle (Maven plugin usage): https://maven.apache.org/plugins/maven-checkstyle-plugin/usage.html
- SARIF guidance (GitHub): https://docs.github.com/en/code-security/code-scanning/integrating-with-code-scanning/sarif-support-for-code-scanning
- SonarQube docs: https://docs.sonarsource.com/
