# Unit Test Results Artifacts

Unit test runners across languages produce machine-readable reports that CI systems can parse and present. These artifacts typically summarize suites, cases, pass/fail, errors, durations, and logs.

## Common producers

- Java/JVM: JUnit 4/5, TestNG, Spock
- .NET: MSTest, NUnit, xUnit
- JavaScript/TypeScript: Jest, Mocha, Vitest, Jasmine (often via adapters)
- Python: pytest, unittest, nose
- Ruby: RSpec, Minitest
- Go: `go test` (stdout, -json), Ginkgo
- Rust: `cargo test` (harness JSON via `-- -Z unstable-options --format json`), `cargo2junit`
- C/C++: GoogleTest (gtest), CppUnit (JUnit XML), Catch2 (JUnit XML)

## Formats and extensions

- JUnit-style XML (de facto standard)
  - Filename patterns: `TEST-*.xml`, `*.junit.xml`
  - Schema: not centrally standardized; widely adopted structure
- NUnit XML (v2/v3)
- TRX (Visual Studio Test Results)
- TAP (Test Anything Protocol)
- Allure Results (JSON files per test, plus attachments)
- Go test JSON (`go test -json`)
- Rust harness JSON (via unstable flag) → convertible to JUnit XML

## Example: JUnit-style XML (excerpt)

```xml
<testsuite name="Example" tests="3" failures="1" time="0.123">
  <testcase classname="pkg.ExampleTest" name="works" time="0.010"/>
  <testcase classname="pkg.ExampleTest" name="fails" time="0.005">
    <failure message="expected true" type="AssertionError">stacktrace...</failure>
  </testcase>
  <testcase classname="pkg.ExampleTest" name="skipped" time="0.001">
    <skipped/>
  </testcase>
</testsuite>
```

## Proofdown viewer mapping

- JUnit/NUnit/TRX/TAP parsable JSON summaries → `artifact.json` (with `depth` limited) and `artifact.table` for suite/case rollups.
- Raw XML → `artifact.link` for download, plus an extracted JSON summary.
- Allure results → link to generated HTML report via `artifact.link`, and include `artifact.json` summary.

## Implications for Proofdown Language Spec

- Prefer `artifact.json` + `artifact.table` as the primary pattern for test results; enforce attributes like `collapsed=true` and `depth<=8` for large trees.
- Keep XML inputs out of the renderer by normalizing to JSON in the SSG; Proofdown remains HTML/script-free and data-first.
- Consider v1.x table enhancements (sorting by status/duration) as renderer options; the language spec remains minimal and deterministic.
- Encourage digest-addressed artifacts (`id` → Index digest) to avoid path drift; parser stays syntax-only.
- Large attachments/logs should use `artifact.link`; avoid inlining large text to respect size limits and deterministic rendering.

## References

- JUnit XML format overview: https://github.com/testmoapp/junitxml
- NUnit test result XML: https://docs.nunit.org/articles/nunit/technical-notes/usage/Test-Result-XML-Format.html
- TRX publishing (Azure Pipelines): https://learn.microsoft.com/azure/devops/pipelines/tasks/reference/publish-test-results-v2
- TAP spec: https://testanything.org/
- Allure result format: https://allurereport.org/docs/how-it-works-test-result-file/
- Go test JSON: https://pkg.go.dev/testing
- cargo test JSON and conversion: https://crates.io/crates/cargo2junit
