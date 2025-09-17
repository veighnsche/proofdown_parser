## Security & Supply Chain

<grid cols=2 gap=16>
  <card title="Code scanning (SARIF)">
    <artifact.table id="sarif-summary.json" kind="security" limit=200 caption="Findings by rule" />
    <artifact.link id="sarif.sarif.json" title="Download full SARIF" download=true />
  </card>
  <card title="SBOM & Provenance">
    <artifact.table id="sbom-summary.json" kind="sbom" caption="Components by ecosystem" />
    <artifact.json id="provenance.json" collapsed=true depth=1 caption="Build attestation" />
    <artifact.link id="sbom.cyclonedx.json" title="Full CycloneDX" />
  </card>
</grid>
