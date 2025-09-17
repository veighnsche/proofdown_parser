# IaC & Policy-as-Code Testing Artifacts

Infrastructure as Code (IaC) and policy-as-code testing validates that infrastructure definitions and configurations conform to security, reliability, and cost policies before deployment.

## Common producers

- Terraform
  - `terraform plan` JSON output, `terraform show -json` (plan and state)
  - Terratest (Go) with JUnit XML results
- Policy & security scanners for IaC
  - Checkov (Terraform, CloudFormation, Kubernetes, etc.) — JSON, SARIF, JUnit XML
  - tfsec (Terraform) — JSON, SARIF
  - Terrascan — JSON, SARIF
  - KICS — JSON, SARIF
  - OPA/Rego via Conftest — JSON outputs, JUnit
- Kubernetes configuration validation
  - kubeconform/kubeval — JSON result sets
  - Gatekeeper/OPA & Kyverno — violation reports (YAML/JSON)

## Artifact types

- Plan/state: Terraform plan/state in JSON for policy evaluation
- Lint/scan results: JSON/SARIF with rule IDs, severities, resources
- Test results: JUnit XML from Terratest/Conftest
- Cost estimates: Infracost JSON summaries

## Examples

Terraform plan (JSON excerpt):
```json
{ "format_version": "1.0", "planned_values": { "root_module": { "resources": [ { "address": "aws_s3_bucket.logs" } ] } } }
```

Checkov JSON (excerpt):
```json
{ "results": { "failed_checks": [ { "check_id": "CKV_AWS_20", "resource": "aws_s3_bucket.logs" } ] } }
```

Conftest JUnit (excerpt):
```xml
<testsuite name="policy" tests="12" failures="1" time="0.12">
  <testcase classname="aws/s3" name="bucket-logging-enabled">
    <failure message="policy violation"/>
  </testcase>
</testsuite>
```

## Proofdown viewer mapping

- JSON results (Checkov/tfsec/Terrascan/Conftest) → `artifact.json` (collapsed) + `artifact.table` summarizing violations by severity/rule.
- Terraform plan/state JSON → `artifact.json` with filtered diffs for reviewer context; full files via `artifact.link`.
- JUnit XML → convert to JSON summary; preserve XML via `artifact.link`.
- Cost reports (Infracost JSON) → `artifact.json` + small deltas table.

## Implications for Proofdown Language Spec

- Keep grammar minimal; treat plans/states and scanner outputs as artifacts ingested as JSON. No Terraform/YAML syntax in Proofdown itself.
- Standardize rollups via `artifact.table` (rule, severity, resource, count) and diff summaries (added/changed/destroyed) as renderer behavior.
- Link large plan/state files via `artifact.link`; show compact, redacted JSON previews to keep pages deterministic.
- Prefer digest-addressed artifacts and default-collapsed JSON for large scans.
- Parser remains syntax-only; SSG normalizes Checkov/tfsec/Terrascan/Conftest outputs and Terraform JSON into stable summaries.

## References

- Terraform JSON plans: https://developer.hashicorp.com/terraform/cli/commands/show#json-output
- Terratest: https://terratest.gruntwork.io/
- Checkov: https://www.checkov.io/
- tfsec: https://aquasecurity.github.io/tfsec/latest/
- Terrascan: https://runterrascan.io/
- KICS: https://docs.kics.io/
- Conftest (OPA): https://www.conftest.dev/
- kubeconform: https://github.com/yannh/kubeconform
- Gatekeeper (OPA): https://open-policy-agent.github.io/gatekeeper/website/
- Kyverno: https://kyverno.io/docs/
- Infracost: https://www.infracost.io/docs/
