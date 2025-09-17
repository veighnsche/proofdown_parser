## E2E — Checkout

<grid cols=2 gap=16>
  <card title="Results">
    <artifact.json id="e2e-summary.json" collapsed=true depth=2 caption="Run summary" />
    <artifact.table id="e2e-summary.json" kind="e2e" />
    <artifact.link id="playwright-report.zip" title="Open HTML report" />
    <artifact.link id="playwright-trace.zip" title="Download trace.zip" />
  </card>
  <card title="Screenshots">
    <artifact.image id="checkout-step3-diff.png" alt="Checkout diff" max_height=480 caption="Step 3" />
    <artifact.image id="order-summary-diff.png" alt="Summary diff" max_height=480 caption="Summary" />
  </card>
</grid>
