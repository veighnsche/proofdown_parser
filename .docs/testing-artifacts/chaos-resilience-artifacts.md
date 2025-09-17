# Chaos & Resilience Testing Artifacts

Chaos engineering validates system resilience by injecting controlled failures and measuring steady-state behavior. Artifacts include experiment definitions, run results, metrics, and dashboards.

## Common producers

- Gremlin — experiments (Scenarios), attack results, logs
- LitmusChaos — Kubernetes Custom Resources (YAML) and result CRs
- Chaos Mesh — experiments and result CRDs
- AWS FIS — experiment templates and execution logs

## Artifact types

- Experiment manifests: YAML (Kubernetes CRDs) or JSON templates
- Run results: JSON/CRD status, pass/fail against steady-state hypothesis
- Metrics & SLOs: time-series exports (Prometheus), dashboards (Grafana JSON)
- Logs & events: Kubernetes events, application logs during chaos window

## Examples

LitmusChaos result (excerpt):
```yaml
apiVersion: litmuschaos.io/v1alpha1
kind: ChaosResult
status:
  experimentStatus:
    phase: Completed
    verdict: Pass
```

## Proofdown viewer mapping

- Manifests (YAML) → render via `artifact.markdown` (pre-rendered) or link via `artifact.link`.
- Run results (JSON/CRD) → `artifact.json` with verdict, duration, targets; roll up experiments in `artifact.table`.
- Grafana dashboards (JSON) → `artifact.json` (collapsed) + links/screenshots via `artifact.image`.

## Implications for Proofdown Language Spec

- Treat CRDs/YAML as artifacts; do not introduce YAML parsing in the grammar. Keep the parser syntax-only and deterministic.
- Standardize experiment rollups (experiment, target, hypothesis, verdict, duration) via `artifact.table`; behavior belongs in renderer.
- Link dashboards and time-series visualizations; embed only static screenshots via `artifact.image` to keep pages script-free.
- Encourage digest-addressed references for manifests and results to ensure immutability and traceability.
- Default to collapsed JSON for large result sets; allow optional depth bounds to protect render size limits.

## References

- Gremlin: https://www.gremlin.com/docs/
- LitmusChaos: https://litmuschaos.io/docs/
- Chaos Mesh: https://chaos-mesh.org/docs/
- AWS Fault Injection Simulator: https://docs.aws.amazon.com/fis/latest/userguide/
