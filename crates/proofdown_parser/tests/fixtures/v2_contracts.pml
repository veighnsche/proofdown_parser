## Contract Verification

<grid cols=2 gap=16>
  <card title="Pact interactions">
    <artifact.json id="pact.json" collapsed=true depth=2 caption="Pact (consumer→provider)" />
    <artifact.table id="pact-summary.json" kind="contracts" caption="Interactions" />
  </card>
  <card title="Provider verification">
    <artifact.table id="pact-verify.json" kind="unit_tests" caption="Verification tests" />
    <artifact.link id="pact-broker-url.txt" title="Open Pact Broker" />
  </card>
</grid>
