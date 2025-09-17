# Proofdown Language — Artifact‑First Viewer Semantics (v1)

Status: v1.0.0-draft

Scope: This specification codifies the artifact-first behaviors of Proofdown’s core viewers and layout primitives, derived from the testing artifacts investigation in `.docs/testing-artifacts/`. It complements the base grammar in `01_proofdown_language_v1.md` by defining deterministic, safe, and AI-authorable semantics for rendering CI artifacts.

Non-goal: This document does not redefine the parser grammar. It specifies viewer semantics, attribute bounds, verification rules, and category idioms to ensure consistent rendering of testing evidence.

## Related documents

- v1 Grammar: [01_proofdown_language_v1.md](./01_proofdown_language_v1.md)
- v2 Spec (additive): [03_proofdown_language_v2.md](./03_proofdown_language_v2.md)
- Authoring guide (LLM-friendly): [../.docs/proofdown-authoring-guide.md](../.docs/proofdown-authoring-guide.md)
- Testing artifacts catalog: [../.docs/testing-artifacts/README.md](../.docs/testing-artifacts/README.md)
- Table schemas (JSON): [./schemas/](./schemas/)

## 1. Artifact addressing and verification (normative)

- Every `artifact.*` viewer MUST reference an artifact by an `id` attribute. The `id` MUST resolve through the signed Index to a specific digest (e.g., SHA‑256) and media type.
- Renderers/Workers MUST verify the Index signature and artifact digest before display. On mismatch or missing artifact, rendering MUST fail closed with a clear error.
- Artifact references are immutable. Regeneration of reports MUST produce new digests; stale `id`s MUST not silently update to new content.

## 2. Determinism and safety (normative)

- Proofdown pages MUST render deterministically for a given set of verified artifacts.
- No scriptable surfaces are allowed in viewers; HTML bundles are linked, not embedded.
- Large artifacts MUST be shown via summaries and safe previews (e.g., collapsed JSON, bounded images) with the full payload exposed only as a downloadable link.

## 3. Core viewers (normative)

This section specifies the canonical viewers surfaced by artifacts across testing disciplines. All attributes are case-sensitive. Unknown attributes MUST be rejected at validation time by the renderer/validator.

### 3.1 `artifact.json`

Displays a structured JSON artifact.

Required:

- `id`: string — Index key for a JSON artifact.

Optional (bounded):

- `collapsed`: `true|false` (default: `true`).
- `depth`: integer `0..8` (default: `3`) — maximum nesting shown when collapsed.
- `json_pointer`: string — JSON Pointer to a subtree (RFC 6901). If present, the view MUST render only the addressed sub-document.

Behavior:

- Rendering MUST be pretty-printed with stable key ordering where applicable.
- Very large nodes beyond `depth` MUST be elided with an explicit indicator.
- Binary or non-UTF‑8 content MUST NOT be displayed in this viewer.

### 3.2 `artifact.table`

Displays a tabular rollup from a JSON artifact that was pre-normalized by the build/SSG (e.g., coverage rollups, test summaries, security findings).

Required:

- `id`: string — Index key for a JSON summary artifact (row-oriented or easily projected to rows).

Optional (bounded):

- `columns`: comma-separated list of column keys (ASCII; renderer MUST ignore keys not present in rows). If omitted, a sensible default per `kind` SHOULD be used.
- `limit`: integer `1..5000` (default: `100`).
- `sort_by`: string — a column key.
- `sort_dir`: `asc|desc` (default: `desc`).
- `kind`: string hint (e.g., `unit_tests`, `coverage`, `security`, `lint`, `contracts`, `fuzzing`, `performance`, `e2e`, `visual`, `a11y`, `mobile`, `db_migrations`, `iac`, `sbom`, `provenance`, `chaos`, `data_quality`). Renderers MAY use this to choose default columns.

Behavior:

- The viewer MUST NOT execute queries; it only projects and orders rows provided in the referenced summary JSON.
- When `limit` truncates rows, the viewer MUST show a truncation notice.

### 3.3 `artifact.markdown`

Displays pre-rendered Markdown as plain content (sanitized at render time). Useful for short logs, migration snippets, and curated notes extracted from artifacts.

Required:

- `id`: string — Index key to a Markdown/text artifact (UTF‑8).

Behavior:

- Renderer MUST sanitize content to prevent HTML/script execution. Inline HTML received from artifacts MUST be escaped.
- Large files SHOULD be linked instead; this viewer is intended for short, human-readable excerpts.

### 3.4 `artifact.image`

Displays an image artifact (e.g., screenshots, visual diffs).

Required:

- `id`: string — Index key for a PNG/JPEG/WebP image.
- `alt`: string — human-readable alternative text (accessibility requirement).

Optional (bounded):

- `max_height`: integer pixels `128..2048` (default: `640`).

Behavior:

- Images MUST be displayed with CSS scaling to the `max_height` bound while preserving aspect ratio. Original files are available via a built-in download action.

### 3.5 `artifact.link`

Presents a safe, verified download link for artifacts that are unsuitable for inline viewing (e.g., HTML bundles, trace zips, videos, `.xcresult`).

Required:

- `id`: string — Index key for the artifact.

Optional:

- `title`: string — link label.
- `download`: `true|false` (default: `false`) — if `true`, hint UAs to download rather than navigate.

Behavior:

- Renderer MUST ensure links point to verified, content-addressed resources. Links MUST open in a sandboxed context or download directly.

## 4. Structural and composition primitives (normative)

Layout primitives enable artifact composition without requiring new syntax for galleries or dashboards.

- `grid(cols=1..6, gap=0..64)` — container for cards/images/tables.
- `card(title: string)` — panel for grouping related artifacts.

Behavior:

- Composition MUST be purely presentational; no script execution or dynamic resizing beyond CSS.

## 5. Category idioms (normative conventions)

The following idioms reflect common, cross-language practices from the investigation. They are conventions, not additional syntax.

- Unit tests: `artifact.json` (summary) + `artifact.table` (suite/case rollups). Link raw XML via `artifact.link` if needed.
- Coverage: `artifact.json` (rollups) + `artifact.table`; link HTML reports via `artifact.link`.
- E2E: `artifact.json` + screenshot galleries using `grid`/`card` + `artifact.image`; traces/videos as `artifact.link`.
- Visual testing: baseline/actual/diff triplets laid out with `grid` + `artifact.image`; dashboard links via `artifact.link`.
- BDD/Gherkin: `.feature` files as artifacts via `artifact.link` or pre-rendered as `artifact.markdown`; JSON summaries via `artifact.json` + `artifact.table`.
- Integration/API: `artifact.json` (collection results) + `artifact.table`; HAR and large payloads via `artifact.link`.
- Performance/load: `artifact.json` (aggregates) + `artifact.table`; large time-series and HTML dashboards linked.
- Security/SAST/DAST/SCA: SARIF/tool JSON via `artifact.json` + `artifact.table`; HTML/PDF dashboards linked.
- Static analysis/lint: JSON or Checkstyle-normalized JSON via `artifact.json` + `artifact.table`.
- Contracts: Pact/OpenAPI/AsyncAPI docs linked; verification summaries via `artifact.json` + `artifact.table`.
- Fuzzing: crash reproducers via `artifact.link`, sanitizer excerpts via `artifact.markdown`, coverage rollups via `artifact.json`.
- Mobile: `.xcresult` via `artifact.link` with `artifact.json` summary; screenshots via `artifact.image`.
- Accessibility: axe/Lighthouse JSON via `artifact.json` + `artifact.table`; screenshots optional.
- Mutation: Stryker JSON via `artifact.json` + `artifact.table`; PIT HTML linked.
- Snapshots: `artifact.image` triplets and `artifact.json` summary of changed/added/removed.
- SBOM & provenance: JSON via `artifact.json` + `artifact.table` rollups; signature/provenance bundles linked.
- Chaos/resilience: CRDs/YAML linked; results via `artifact.json` + `artifact.table`; dashboards linked.
- Data quality & ML: validations/evaluations in `artifact.json` + `artifact.table`; plots via `artifact.image`; dashboards linked.
- IaC/policy-as-code: plan/state JSON and scan results via `artifact.json` + `artifact.table`; large plans linked.
- DB migrations: short SQL via `artifact.markdown`, full scripts linked; TAP/JUnit summaries via `artifact.json`.

## 6. Attribute bounds and defaults (normative)

- `artifact.json.depth`: `0..8` (default `3`).
- `artifact.json.collapsed`: default `true`.
- `artifact.image.max_height`: `128..2048` (default `640`).
- `artifact.table.limit`: `1..5000` (default `100`).
- Viewers MUST reject out-of-range values with a validation error.

## 7. Accessibility requirements (normative)

- `artifact.image` requires `alt` text.
- Tables SHOULD have an accessible caption (use the surrounding `card(title=...)` or a preceding heading).
- Renderer MUST ensure text contrast and keyboard navigation without client-side scripts.

## 8. Failure and fallback behavior (normative)

- Unknown or missing artifacts (by `id`) MUST render a clear error with the referenced `id` and not a partial/empty widget.
- Type mismatch (e.g., non-JSON in `artifact.json`) MUST error.
- Oversized artifacts SHOULD be linked with guidance rather than rendered inline.

## 9. Normalization responsibilities (non-normative but expected)

To keep the language and parser small, the SSG/validator is expected to:

- Normalize tool-specific outputs to common JSON summaries for `artifact.table`.
- Generate per-category summaries (e.g., unit test rollups, coverage by file, SARIF rule counts) with stable schemas.
- Produce digest-addressed artifacts and enumerate them in the signed Index.

## 10. AI authoring guidance (normative conventions)

- Prefer the trio `artifact.json` + `artifact.table` + `artifact.link` for most evidence.
- Use `grid`/`card` to arrange screenshots and related artifacts; do not invent new syntax.
- Default to `collapsed=true` and small `depth` for large JSON; raise `depth` only for small, human-scale trees.
- Always provide `alt` for images and short titles for links.

## 11. Examples (informative)

Minimal artifact-first panel:

```pml
# CI Evidence — Build {{ commit }}

<grid cols=3 gap=16>
  <card title="Unit tests">
    <artifact.json id="tests-summary.json" collapsed=true depth=3 />
    <artifact.table id="tests-summary.json" columns="suite,name,status,duration_ms" sort_by="status" />
  </card>
  <card title="Coverage">
    <artifact.json id="coverage-summary.json" collapsed=true depth=2 />
    <artifact.table id="coverage-summary.json" columns="package,lines_pct,branches_pct" sort_by="lines_pct" />
    <artifact.link id="coverage-html.zip" title="Open full HTML report" />
  </card>
  <card title="Screenshots (failures)">
    <artifact.image id="login-fail-diff.png" alt="Login diff" max_height=480 />
    <artifact.image id="checkout-fail-diff.png" alt="Checkout diff" max_height=480 />
  </card>
</grid>
```

## 12. Compatibility

- This document extends v1 semantics without altering the base grammar. All additions are viewer behaviors and attribute bounds.
- Future v1.x may add new viewers or attributes if they preserve determinism and safety.
