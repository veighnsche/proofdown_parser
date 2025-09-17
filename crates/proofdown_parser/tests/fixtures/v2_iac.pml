## IaC — Terraform & Policy

<grid cols=2 gap=16>
  <card title="Plan preview">
    <artifact.json id="tf-plan-summary.json" collapsed=true depth=2 caption="Added/changed/destroyed" />
    <artifact.link id="terraform-plan.json" title="Full plan (JSON)" />
  </card>
  <card title="Policy scan">
    <artifact.table id="checkov-summary.json" kind="iac" sort_by="severity" caption="Violations by rule" />
    <artifact.link id="checkov-full.json" title="Download full JSON" />
  </card>
</grid>
