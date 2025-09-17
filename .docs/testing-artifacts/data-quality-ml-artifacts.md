# Data Quality & ML Testing Artifacts

Data pipelines and ML systems produce artifacts validating schema integrity, expectations, drift, and model performance. These artifacts are critical to ensure that training/serving data and models meet quality gates.

## Data quality (DQ) artifact producers

- Great Expectations — validation results and Data Docs
- Soda Core / Soda Cloud — scan results, checks, metrics
- Deequ — constraint checks and metrics (Spark)
- dbt tests — JUnit-style results, JSON manifests/run results

## ML testing & evaluation producers

- TensorFlow Model Analysis (TFMA) — evaluation JSON, plots
- scikit-learn — classification/regression reports, confusion matrices (exported JSON/CSV/images)
- Evidently AI — data and model drift reports (JSON/HTML)
- MLflow — run metadata, metrics, artifacts, model registry

## Artifact types

- Validation results: JSON (pass/fail per expectation/check), JUnit XML summaries, HTML dashboards
- Metrics: CSV/JSON time series for quality or evaluation metrics
- Visualization: PNG/SVG charts (confusion matrix, ROC, PR curves), HTML reports
- Manifests: dbt `manifest.json`, `run_results.json`

## Examples

Great Expectations validation result (excerpt):
```json
{
  "statistics": { "successful_expectations": 23, "unsuccessful_expectations": 1 },
  "results": [ { "expectation_config": { "expect_column_values_to_not_be_null": {"column": "id"}}, "success": true } ]
}
```

Evidently AI drift report (excerpt):
```json
{ "metrics": [ { "metric": "DataDriftMetric", "result": { "dataset_drift": true, "share_of_drifted_features": 0.32 } } ] }
```

## Proofdown viewer mapping

- JSON validation/evaluation outputs → `artifact.json` (collapsed) + `artifact.table` summarizing pass/fail and key metrics.
- Confusion matrices and curves → `artifact.image` grids for quick visual inspection.
- HTML dashboards (Evidently/GE Data Docs) → `artifact.link` with a compact summary in-page.
- dbt manifests and run results → `artifact.json` with counts per model/test; link to generated docs site as needed.

## Implications for Proofdown Language Spec

- Prioritize JSON summaries over HTML dashboards to keep pages deterministic; large HTML bundles should be linked with `artifact.link`.
- Standardize table rollups (checks passed/failed, dataset drift flags, per-model metrics) via `artifact.table` conventions; behavior belongs in renderer.
- Encourage `artifact.image` for common ML visuals (confusion matrix, ROC/PR). No new block types are required.
- Keep grammar minimal; the parser remains syntax-only while SSG normalizes GE/Soda/dbt/TFMA outputs.
- Default to collapsed JSON with optional `depth` for big artifacts (dbt manifests, TFMA results) and use digest-addressed references.

## References

- Great Expectations validation results: https://docs.greatexpectations.io/
- Soda Core scan results: https://github.com/sodadata/soda-core/blob/main/docs/scan-core.md
- Deequ: https://github.com/awslabs/deequ
- TFMA: https://www.tensorflow.org/tfx/guide/tfma
- MLflow Tracking: https://mlflow.org/docs/latest/tracking.html
- Evidently: https://docs.evidentlyai.com/
- dbt manifests and results: https://docs.getdbt.com/reference/artifacts/manifest-json
