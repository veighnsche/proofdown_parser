# Build Evidence — {{ commit }}

<grid cols=3 gap=16>
  <card title="Unit tests">
    <artifact.json id="tests-summary.json" collapsed=true depth=3 json_pointer="/summary" caption="Aggregated results" />
    <artifact.table id="tests-summary.json" kind="unit_tests" limit=100 sort_by="status" caption="Suites and cases" />
  </card>
  <card title="Coverage">
    <artifact.json id="coverage-summary.json" collapsed=true depth=2 caption="Project coverage" />
    <artifact.table id="coverage-summary.json" kind="coverage" />
    <artifact.link id="coverage-html.zip" title="Open full HTML report" caption="HTML bundle" />
  </card>
  <card title="Security">
    <artifact.table id="sarif-summary.json" kind="security" limit=200 caption="Findings by rule" />
    <artifact.link id="sarif.sarif.json" title="Download full SARIF" download=true />
  </card>
</grid>
