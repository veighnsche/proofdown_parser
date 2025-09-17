# Proofdown Language — v2 Specification

Status: v2.0.0-draft (additive over v1; compatible with all valid v1 documents)

Scope: v2 refines the artifact-first semantics based on the testing artifacts investigation, adds a small number of viewer capabilities and accessibility attributes, and introduces normative row conventions for common artifact categories. The grammar remains minimal and deterministic.

Non-goal: v2 does not introduce templating, scripting, or HTML passthrough. Viewer options remain bounded and verification-first.

## Related documents

- v1 Grammar: [01_proofdown_language_v1.md](./01_proofdown_language_v1.md)
- Artifact-first semantics (v1): [02_artifact_first_language_spec.md](./02_artifact_first_language_spec.md)
- Authoring guide: [../.docs/proofdown-authoring-guide.md](../.docs/proofdown-authoring-guide.md)
- Testing artifacts catalog: [../.docs/testing-artifacts/README.md](../.docs/testing-artifacts/README.md)
- Table schemas (JSON): [./schemas/](./schemas/)

## 1. Compatibility and delta from v1 (informative)

- Grammar is unchanged (headings, paragraphs, components). All valid v1 sources are valid v2 sources.
- `artifact.json` gains a normative `json_pointer` attribute (introduced as a convention in v1 semantics) to scope JSON to a subtree.
- `artifact.table` gains optional `caption` and support for column selectors using JSON Pointer (static projection only).
- `artifact.image` gains optional `caption` (in addition to required `alt`).
- New viewer: `artifact.text` for short UTF‑8 plaintext excerpts (monospace), sanitized.
- New normative conventions for `artifact.table kind` default columns for common categories (unit tests, coverage, security, etc.).
- Accessibility, bounds, and failure behaviors are reiterated with stronger requirements.

## 2. Artifact addressing and verification (normative)

Unchanged from v1, reiterated for emphasis:

- All `artifact.*` viewers MUST reference an `id` that resolves via the signed Index to a content digest and media type.
- Engines MUST verify the Index signature and artifact digest prior to rendering. On mismatch/missing, error and fail closed.
- Artifact references are immutable; engines MUST NOT silently substitute newer content.

## 3. Determinism and safety (normative)

- Rendering of a given document and artifact set MUST be deterministic.
- No scriptable surfaces; HTML bundles and traces MUST be linked via `artifact.link`.
- Large/complex artifacts MUST be summarized inline and downloadable in full.

## 4. Core viewers (normative)

### 4.1 `artifact.json`

Required:

- `id`: string — Index key to a JSON artifact.

Optional (bounded):

- `collapsed`: `true|false` (default: `true`).
- `depth`: integer `0..8` (default: `3`).
- `json_pointer`: string — RFC 6901 pointer to a subtree to render.
- `caption`: string — short accessible caption (supplements surrounding headings/cards).

Behavior:

- Pretty-printed, stable key ordering where applicable.
- Nodes beyond `depth` MUST be elided with an indicator.
- Non-JSON artifacts MUST error.

### 4.2 `artifact.table`

Required:

- `id`: string — Index key for a JSON summary artifact.

Optional (bounded):

- `columns`: comma-separated list of column selectors. Each selector MAY be a simple key (e.g., `status`) or an RFC 6901 JSON Pointer (e.g., `/suite/name`). If the selector resolves to non-scalar, the cell MUST display an ellipsis indicator.
- `limit`: integer `1..5000` (default: `100`).
- `sort_by`: string — selector as above; if not present in rows, ignored.
- `sort_dir`: `asc|desc` (default: `desc`).
- `kind`: string hint enumerated in §6 (e.g., `unit_tests`, `coverage`, `security`, ...).
- `caption`: string — accessible table caption.

Behavior:

- Static projection only. The viewer MUST NOT evaluate expressions, filters, or aggregations.
- When truncating, a truncation notice MUST be shown.
- If both `columns` and `kind` are absent, engines MAY use a sensible default for known `kind` inferred from artifact metadata.

### 4.3 `artifact.markdown`

Required:

- `id`: string — Index key to pre-rendered Markdown/text (UTF‑8).

Optional:

- `caption`: string — accessible caption.

Behavior:

- Renderer MUST sanitize or escape inline HTML from the artifact.
- Large files SHOULD be linked rather than inlined.

### 4.4 `artifact.image`

Required:

- `id`: string — Index key to PNG/JPEG/WebP image.
- `alt`: string — required alternative text.

Optional (bounded):

- `max_height`: integer `128..2048` (default: `640`).
- `caption`: string — short caption beneath the image.

Behavior:

- Scale to `max_height` with preserved aspect.

### 4.5 `artifact.link`

Required:

- `id`: string — Index key for any downloadable artifact.

Optional:

- `title`: string — link label.
- `download`: `true|false` (default: `false`).
- `caption`: string — short description.

Behavior:

- Links MUST be verified, content-addressed, and sandboxed.

### 4.6 `artifact.text` (new)

Displays short plaintext excerpts (logs, SQL fragments) with monospace styling.

Required:

- `id`: string — Index key to UTF‑8 text.

Optional (bounded):

- `max_lines`: integer `1..500` (default: `100`). Lines beyond this MUST be elided.
- `caption`: string — short caption.

Behavior:

- Render as preformatted, escaped text. Intended for short excerpts; large logs SHOULD be linked.

## 5. Structural primitives (normative)

Unchanged from v1: `grid(cols=1..6, gap=0..64)` and `card(title)`. Composition is presentational and script-free.

## 6. Normative table kinds and column conventions (normative "SHOULD")

Renderers SHOULD adopt the following default columns when `kind` is supplied and `columns` is omitted. Producers SHOULD emit summaries whose rows provide these keys (names can be flat or via JSON Pointer selectors):

- `unit_tests`: `suite`, `name`, `status`, `duration_ms`.
- `coverage`: `package`, `file?`, `lines_pct`, `branches_pct`.
- `e2e`: `spec`, `scenario`, `status`, `duration_ms`.
- `visual`: `story|spec`, `status`, `mismatch_pct?`, `changed?`.
- `bdd`: `feature`, `scenario`, `status`, `duration_ms`.
- `api`: `collection`, `request`, `status`, `duration_ms`.
- `performance`: `endpoint`, `rps`, `p50_ms`, `p95_ms`, `error_rate`.
- `security`: `rule`, `severity`, `file|resource`, `count`.
- `lint`: `rule`, `severity`, `file`, `count`.
- `contracts`: `provider`, `path`, `method`, `verification_status`.
- `fuzzing`: `target`, `crashes`, `hangs`, `coverage_pct?`.
- `mobile`: `device`, `test`, `status`, `duration_ms`.
- `a11y`: `rule`, `impact`, `nodes_count`, `page`.
- `mutation`: `file`, `killed`, `survived`, `score`.
- `snapshots`: `file|test`, `added`, `changed`, `removed`.
- `sbom`: `ecosystem`, `package`, `version`, `license?`, `severity_counts?`.
- `provenance`: `subject`, `sha256`, `builder`, `buildType?`.
- `chaos`: `experiment`, `target`, `hypothesis`, `verdict`.
- `data_quality`: `check`, `dataset`, `status`, `metric?`.
- `iac`: `rule`, `severity`, `resource`, `file`.
- `db_migrations`: `migration_id`, `description`, `state`, `duration_ms?`.

Notes:

- `?` indicates optional. `a|b` indicates a producer MAY supply either.
- These are conventions to improve interoperability and AI authoring; engines MUST NOT reject tables that lack these keys.

## 7. Bounds and validation (normative)

- `artifact.json.depth`: `0..8` (default `3`).
- `artifact.table.limit`: `1..5000` (default `100`).
- `artifact.image.max_height`: `128..2048` (default `640`).
- `artifact.text.max_lines`: `1..500` (default `100`).
- Out-of-range values MUST raise a validation error and prevent rendering.

## 8. Accessibility (normative)

- `artifact.image` requires `alt`; captions are recommended across viewers.
- Tables SHOULD have a caption (attribute or surrounding heading/card title).
- Renderers MUST ensure keyboard navigation and sufficient contrast without client-side scripts.

## 9. Failure and fallback (normative)

- Missing/unknown `id` or digest mismatch MUST error visibly.
- Type mismatch (e.g., binary in `artifact.json`) MUST error.
- Oversized artifacts SHOULD be shown as links with context.

## 10. Normalization responsibilities (informative)

- SSGs/validators SHOULD continue to normalize tool outputs into stable JSON summaries to avoid viewer query semantics.
- Where producers provide only raw formats (e.g., JUnit XML, Checkstyle XML), SSGs SHOULD emit canonical JSON for `artifact.json`/`artifact.table`.

## 11. AI authoring guidance (informative)

- Prefer the trio `artifact.json` + `artifact.table` + `artifact.link`; use `artifact.image` for screenshots/diffs.
- Collapse large JSON (`collapsed=true`, `depth` 2–3). Set table `limit` reasonably (≤100 by default).
- Supply `kind` for common summaries to unlock default columns.
- Provide `alt` and, where helpful, `caption`.

## 12. Examples (informative)

```pml
# CI Evidence — {{ commit }}

<grid cols=3 gap=16>
  <card title="Unit tests">
    <artifact.json id="tests-summary.json" collapsed=true depth=3 json_pointer="/summary" caption="Overall results" />
    <artifact.table id="tests-summary.json" kind="unit_tests" limit=100 sort_by="status" caption="Suites and cases" />
  </card>
  <card title="Coverage">
    <artifact.json id="coverage-summary.json" collapsed=true depth=2 />
    <artifact.table id="coverage-summary.json" kind="coverage" />
    <artifact.link id="coverage-html.zip" title="Open full HTML report" />
  </card>
  <card title="Security">
    <artifact.table id="sarif-summary.json" kind="security" limit=200 />
    <artifact.link id="sarif.sarif.json" title="Download full SARIF" download=true />
  </card>
</grid>

## Failures — screenshots
<grid cols=3 gap=12>
  <artifact.image id="login-diff.png" alt="Login diff" max_height=480 caption="Login" />
  <artifact.image id="checkout-diff.png" alt="Checkout diff" max_height=480 caption="Checkout" />
  <artifact.image id="summary-diff.png" alt="Summary diff" max_height=480 caption="Summary" />
</grid>
```

## 13. Versioning and adoption

- v2 is additive and compatible with v1. Renderers MAY advertise support for v2 features (e.g., `artifact.text`, `caption`, pointer columns) via capabilities.
- Documents SHOULD declare their intended spec version at the top (out-of-band or via repository policy) until an in-file mechanism is standardized.

## 14. Minimal JSON schemas for common table kinds (informative)

These minimal, non-exhaustive JSON Schemas document expected shapes for row-oriented summaries consumed by `artifact.table`. Producers SHOULD approximate these shapes to maximize interoperability. Implementations MAY validate against stricter, local schemas.

### 14.1 Unit tests (kind: unit_tests)

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "rows": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "suite": {"type": "string"},
          "name": {"type": "string"},
          "status": {"type": "string", "enum": ["passed", "failed", "skipped", "xfail"]},
          "duration_ms": {"type": "number", "minimum": 0}
        },
        "required": ["suite", "name", "status"]
      }
    }
  },
  "required": ["rows"]
}
```

### 14.2 Coverage (kind: coverage)

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "rows": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "package": {"type": "string"},
          "file": {"type": ["string", "null"]},
          "lines_pct": {"type": "number", "minimum": 0, "maximum": 100},
          "branches_pct": {"type": "number", "minimum": 0, "maximum": 100}
        },
        "required": ["package", "lines_pct"]
      }
    }
  },
  "required": ["rows"]
}
```

### 14.3 Security findings (kind: security)

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "rows": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "rule": {"type": "string"},
          "severity": {"type": "string", "enum": ["critical", "high", "medium", "low", "note"]},
          "file": {"type": ["string", "null"]},
          "resource": {"type": ["string", "null"]},
          "count": {"type": "integer", "minimum": 1}
        },
        "required": ["rule", "severity", "count"]
      }
    }
  },
  "required": ["rows"]
}
```

### 14.4 E2E (kind: e2e)

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "rows": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "spec": {"type": "string"},
          "scenario": {"type": "string"},
          "status": {"type": "string", "enum": ["passed", "failed", "skipped"]},
          "duration_ms": {"type": "number", "minimum": 0}
        },
        "required": ["spec", "scenario", "status"]
      }
    }
  },
  "required": ["rows"]
}
```

### 14.5 IaC (kind: iac)

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "rows": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "rule": {"type": "string"},
          "severity": {"type": "string"},
          "resource": {"type": "string"},
          "file": {"type": "string"}
        },
        "required": ["rule", "severity", "resource"]
      }
    }
  },
  "required": ["rows"]
}
```

## 15. Conformance examples (informative)

### 15.1 Minimal v2 document using captions, pointers, and kinds

```pml
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
```

### 15.2 E2E with screenshots and trace links

```pml
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
```

### 15.3 IaC & Policy with plan preview and scan rollup

```pml
## IaC — Terraform & Policy

<grid cols=2 gap=16>
  <card title="Plan preview">
    <artifact.json id="tf-plan-summary.json" collapsed=true depth=2 caption="Added/changed/destroyed" />
    <artifact.link id="terraform-plan.json" title="Full plan (JSON)" />
  </card>
  <card title="Policy scan">
    <artifact.table id="checkov-summary.json" kind="iac" sort_by="severity" />
    <artifact.link id="checkov-full.json" title="Download full JSON" />
  </card>
</grid>
```

### 15.4 DB migrations with short SQL and TAP summary

```pml
## Database migrations

<grid cols=2 gap=16>
  <card title="Status">
    <artifact.table id="migrations-summary.json" kind="db_migrations" />
  </card>
  <card title="SQL snippets">
    <artifact.text id="migration-001.sql" max_lines=50 caption="001-add-index.sql" />
    <artifact.link id="migrations.zip" title="All migration scripts" />
  </card>
</grid>
```

### 15.5 Accessibility and Visual testing

```pml
## A11y & Visual

<grid cols=2 gap=16>
  <card title="Accessibility">
    <artifact.table id="a11y-summary.json" kind="a11y" />
    <artifact.link id="lighthouse-report.html.zip" title="Lighthouse HTML" />
  </card>
  <card title="Visual diffs">
    <artifact.image id="login-diff.png" alt="Login diff" max_height=480 caption="Login" />
    <artifact.image id="dashboard-diff.png" alt="Dashboard diff" max_height=480 caption="Dashboard" />
  </card>
</grid>
```
