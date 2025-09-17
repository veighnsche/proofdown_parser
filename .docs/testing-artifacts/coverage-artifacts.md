# Code Coverage Artifacts

Coverage artifacts quantify which parts of the codebase were executed during tests. They are consumed by CI dashboards and quality gates, and can be rendered as HTML, JSON, or XML.

## Common producers

- JVM: JaCoCo (XML, HTML), Cobertura XML, Clover XML
- JavaScript/TypeScript: Istanbul/nyc (coverage.json, lcov.info, HTML), Jest (Istanbul under the hood)
- Go: `go test -coverprofile` (text profile), `go tool cover` to HTML
- Python: Coverage.py (XML: Cobertura, JSON, HTML)
- C/C++: gcov + lcov (lcov.info), llvm-cov (JSON, text), cppcov tools
- .NET: Coverlet (Cobertura, OpenCover), Visual Studio (coverage)
- Rust: `grcov` + LLVM profile data (lcov.info), `cargo tarpaulin` (XML: Cobertura)

## Formats and extensions

- LCOV info (`lcov.info`): line-oriented text format.
- Istanbul JSON (`coverage/coverage-final.json`), summary JSON.
- JaCoCo XML (`jacoco.xml`).
- Cobertura XML (`cobertura.xml`).
- Clover XML (`clover.xml`).
- Go cover profile (`coverage.out`).

## Examples

LCOV (excerpt):
```
TN:
SF:/path/file.js
DA:10,1
DA:11,0
LF:20
LH:15
end_of_record
```

JaCoCo XML (excerpt):
```xml
<report name="Project">
  <package name="com/example">
    <class name="Foo" sourcefilename="Foo.java">
      <method name="bar" line="42"/>
    </class>
  </package>
</report>
```

Istanbul summary JSON (excerpt):
```json
{
  "total": {"lines": {"pct": 92.3}, "branches": {"pct": 85.0}}
}
```

## Proofdown viewer mapping

- LCOV, JaCoCo, Cobertura, Clover → parse to normalized JSON → `artifact.json` and `artifact.table` for package/class/file rollups.
- HTML coverage → store as artifact bundle; link via `artifact.link`, and include JSON summary for quick review.
- Go cover profile → convert to LCOV or summary JSON; present both.

## Implications for Proofdown Language Spec

- Keep coverage rendering deterministic by favoring normalized JSON summaries over raw HTML; use `artifact.json` + `artifact.table` as the default pattern.
- Consider standard viewer hints for coverage rollups (e.g., group-by: package/file) as attributes on `artifact.table`, implemented in the renderer, not the grammar.
- Enforce safe size bounds via `collapsed=true` and optional `depth` to avoid giant trees; large HTML reports should be linked via `artifact.link`.
- Do not interpret tool-specific formats at parse time; the parser remains syntax-only. Normalization to a common JSON shape happens in the SSG.
- Prefer digest-addressed artifacts to prevent drift when reports are regenerated.

## References

- LCOV format (gcovr): https://gcovr.com/en/stable/output/lcov.html
- LCOV docs: https://lcov.readthedocs.io/
- Istanbul formats: https://istanbul.js.org/docs/advanced/alternative-reporters/
- Istanbul coverage.json: https://github.com/gotwarlost/istanbul/blob/master/coverage.json.md
- JaCoCo report: https://www.eclemma.org/jacoco/trunk/doc/report-mojo.html
- Cobertura (gcovr): https://gcovr.com/en/stable/output/cobertura.html
- Clover (gcovr): https://gcovr.com/en/stable/output/clover.html
- Go coverage: https://go.dev/doc/build-cover
