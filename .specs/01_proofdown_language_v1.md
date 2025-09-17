# Proofdown Language — v1 Specification

Status: v1.0.0-draft (normative for language behavior; parser and SSG may partition responsibilities per contract)

Audience: language authors, parser implementers, SSG/renderers, and AI systems authoring Proofdown.

## Related documents

- v2 Spec (additive): [03_proofdown_language_v2.md](./03_proofdown_language_v2.md)
- Artifact-first semantics: [02_artifact_first_language_spec.md](./02_artifact_first_language_spec.md)
- Authoring guide (LLM-friendly): [../.docs/proofdown-authoring-guide.md](../.docs/proofdown-authoring-guide.md)
- Testing artifacts catalog: [../.docs/testing-artifacts/README.md](../.docs/testing-artifacts/README.md)
- Table schemas (JSON): [./schemas/](./schemas/)

## 1. Purpose and design goals

Proofdown is a minimal, deterministic, and safe markup optimized to present verifiable testing evidence. It is designed to be easily authored by AI while remaining predictable and reviewable by humans.

Design goals:

- Determinism: The same source yields one AST and renders identically across compliant engines.
- Safety: No scriptable surfaces; no raw HTML; constrained, typed viewers for artifacts.
- Artifact-first: Content-addressed references (by digest via the Index) are first-class.
- AI-authorable: Small grammar, repetitive patterns, and strict, fail-fast rules.
- Review-friendly: Emphasize claims, tests, outcomes; keep formatting boring and consistent.

Non-goals:

- General-purpose web publishing (complex layouts and interactivity are out-of-scope).
- Templating language (no variables/loops/conditions in v1; see §11 for text interpolation posture).
- HTML passthrough.

## 2. Source file and encoding

- File extension: `.pml` (Proof Markup Language).
- Encoding: UTF-8.
- Newlines: inputs may contain `\n` or `\r\n`; parsers MUST treat `\r\n` as `\n` during tokenization.
- Canonicalization intent: conforming tools SHOULD preserve semantically equivalent formatting (e.g., heading spacing) to produce stable diffs.

## 3. Document model (blocks)

A document is a sequence of block nodes:

- Heading (level 1–4) — top-level only.
- Paragraph.
- Component (HTML-like, typed, may be nested).

Inside components, children MAY be paragraphs and components. Headings are not recognized inside components in v1 (lines beginning with `#` are treated as paragraph text there).

## 4. Lexical conventions and whitespace

- Indentation: Spaces or tabs may precede component start tags; indentation is not semantically significant.
- Blank lines: One or more blank lines separate paragraphs and are otherwise ignored.
- Paragraph joining: Consecutive non-empty text lines join into a single paragraph with a single ASCII space between lines. Leading/trailing ASCII spaces on each line are trimmed before joining.
- Text preservation: Within a paragraph line, interior spaces are preserved as-is after trimming.

## 5. Headings (top-level only)

A top-level heading is recognized only at the document root.

Syntax (regex-style):

- `^#{1,4}[ ]+(?P<text>.+)$`

Rules:

- One or more ASCII spaces MUST follow the `#…` marker.
- The heading text is trimmed of leading/trailing ASCII whitespace after the marker and collected as plain text; no inline formatting is parsed in v1.
- Lines like `#Title` (no space) are NOT headings and are parsed as paragraph text.
- Headings MUST NOT appear inside components in v1; if present, they parse as paragraph text.

AST mapping:

- `{ "type": "Heading", "level": <u8>, "text": <string> }`

## 6. Paragraphs

A paragraph is formed by one or more consecutive non-empty lines that do not begin a component start or end tag and, at top-level only, do not match a heading.

Rules:

- Lines are trimmed of leading/trailing ASCII spaces and joined with a single ASCII space.
- A blank line terminates a paragraph.
- Inside components, lines beginning with `#` remain paragraph text (no nested headings in v1).

AST mapping:

- `{ "type": "Paragraph", "text": <string> }`

## 7. Components (typed blocks)

Components provide structured, typed containers and viewers. Their syntax resembles simplified HTML without HTML semantics.

### 7.1 Names

- `name = segment ( "." segment )*`
- `segment = [a-z] [a-z0-9-]*`
- Names are case-sensitive; only lowercase a–z, digits, and hyphen are allowed in segments. Dots separate namespaces (e.g., `artifact.summary`).

### 7.2 Start and end tags

- Start tag forms:
  - `<name attr-list>`
  - `<name attr-list />` (self-closing)
- End tag: `</name>`
- `attr-list` is zero or more attributes separated by ASCII whitespace.
- Indentation (spaces/tabs) is allowed before `<`.

### 7.3 Attributes

- Form: `key=value`
- `key = [A-Za-z_][A-Za-z0-9._-]*`
- `value` may be:
  - Quoted: `"…"` (double quotes). No escape sequences in v1; the closing `"` terminates the value. Embedded `"` are not allowed.
  - Bare: one or more of `[A-Za-z0-9._\-/:]+`. Parsing stops at whitespace or `>` or `/>`.
- Attribute order MUST be preserved in the AST.
- Unknown or duplicate attribute keys are not a parse-time error; semantic validation is performed by the renderer/validator.

AST mapping for components:

- `{ "type": "Component", "name": <string>, "attrs": [ {"key": <string>, "value": <string>}... ], "children": [Block...], "self_closing": <bool> }`

### 7.4 Nesting and children

- Components may be nested arbitrarily up to the configured depth limit (default 16; see §10). Children are block nodes (Paragraph or Component in v1).
- A self-closing component MUST have an empty `children` array and `self_closing=true`.
- A non-self-closing start tag MUST be matched by a corresponding end tag with the same `name`.

### 7.5 Errors (syntactic)

- Unterminated component: missing matching end tag.
- Mismatched close: closing tag name does not match the innermost open component.
- Malformed attribute token: missing `=` or invalid characters in key/value.

Error reporting MUST include best-effort 1-based `line`, `col`, `kind`, and a helpful `msg`.

## 8. Interpolation and link macro (v1 posture)

- Text interpolation `{{ field }}`: treated as literal text by the parser. A renderer MAY substitute these placeholders from the signed Index using text-only, HTML-escaped replacement. The language reserves `{{ … }}` for this purpose; parsers MUST NOT interpret it specially in v1.
- Link macro `[[ … ]]`: reserved for v1.1. ABNF is defined in the parser outline and MAY be implemented by renderers/parsers as an additive feature without breaking v1.0 inputs. In v1.0, `[[ … ]]` is plain text.

## 9. Component registry (v1 semantics)

The language defines a minimal, safe set of semantic components. The parser only recognizes component syntax; semantic validation (names, attributes, bounds) is performed by the renderer/validator.

Structural:

- `grid(cols=1..6, gap=0..64)` — Container arranging children in a CSS grid-like layout.
- `section(title: string)` — Logical grouping with a title.
- `card(title: string)` — Panel containing children.

Artifact viewers:

- `artifact.summary(id)` — Summary of a test run; expects `summary:test` JSON.
- `artifact.table(id)` — Tabular coverage; expects coverage JSON.
- `artifact.json(id, collapsed=true|false, depth=0..8)` — JSON viewer.
- `artifact.markdown(id)` — Pre-rendered Markdown artifact (rendered safely by viewer).
- `artifact.image(id, alt, max_height=128..2048)` — Image viewer.
- `artifact.link(id, download=true|false, title)` — Canonical download link.

Repository viewers (optional in v1 if repository context exists):

- `repo.code(path, range, lang, highlight)` — Line range SHOULD be ≤ 400.
- `repo.link(path, label, lines)`
- `repo.tree(path, depth=1..5, include, exclude)`
- `repo.diff(base_id, head_id, path, context=0..10)`
- `repo.symbol(path, name)`

Semantic rules (validator/renderer):

- Unknown component names or attributes MUST be rejected.
- Attribute bounds MUST be enforced as indicated.
- Artifact `id` MUST resolve to an entry in the signed Index, and viewer types MUST match declared artifact media/kind.

## 10. Limits and determinism

Defaults (configurable in implementations):

- `max_depth = 16` — Maximum nested component depth. Depth count includes the current open component.
- `max_nodes = 50_000` — Maximum total blocks in the document.
- `max_input_bytes = 1 MiB` — Maximum input size.

Breaches:

- MUST produce an error with `kind = "LimitExceeded"` and an explanatory message (e.g., `"depth exceeded: 18 > 16"`).

Determinism:

- Parsing MUST be pure (no environment I/O) and deterministic.
- Attribute order is preserved; child order is preserved.
- Serializing the AST to JSON with stable field ordering MUST be byte-for-byte stable for the same input.

## 11. Security posture

- No raw HTML; `<...>` constructs are Proofdown components only.
- No script execution; viewers are allowlisted by the renderer.
- External references MUST be content-addressed or blocked by the renderer; the language encourages digest-addressed references via the Index.

## 12. Conformance — reference behaviors from tests

The following behaviors are normative and demonstrated by the test corpus under `crates/proofdown_parser/tests/cases/`:

- Heading recognition requires a space after `#` at top-level only: `# Title` ⇒ Heading; `#Title` ⇒ Paragraph.
- Paragraph joining coalesces consecutive lines into one paragraph with a single space: `Hello\nworld` ⇒ `"Hello world"`.
- CRLF newlines are normalized: lines ending with `\r\n` parse identically to `\n`.
- Components support bare and quoted attribute values: `<grid cols=3 gap="16" />` preserves `"16"` as `"16"`.
- Self-closing components set `self_closing=true` and have empty children.
- Nested headings are not recognized inside components in v1: `# Heading inside` within a component is paragraph text.
- Errors:
  - Mismatched close produces `Syntax` with message like `"unexpected close </card>"`.
  - Unterminated component reports the line/col of the early end-of-input with message like `"unterminated component <grid>"`.
  - Malformed attribute token reports `Syntax` with message like `"malformed attribute near 'cols': expected key=value"`.
  - Depth limit excess produces `LimitExceeded` with an explanatory message.

## 13. AST shape (serde JSON)

A conforming parser exposes an AST with the following stable JSON layout (excerpts):

```json
{
  "blocks": [
    { "type": "Heading", "level": 1, "text": "…" },
    { "type": "Paragraph", "text": "…" },
    { "type": "Component", "name": "grid", "attrs": [
      { "key": "cols", "value": "3" }
    ], "children": [ /* Block… */ ], "self_closing": false }
  ]
}
```

Attribute entries:

```json
{ "key": "id", "value": "tests-summary" }
```

## 14. Examples (normative)

Minimal document:

```pml
# QA Evidence for {{ commit }}

<grid cols=3 gap=16>
  <card title="Tests">
    <artifact.summary id="tests-summary" />
  </card>
  <card title="Coverage">
    <artifact.table id="coverage" />
  </card>
  <card title="Key Failures">
    <artifact.markdown id="failures" />
  </card>
</grid>
```

Component nesting with paragraphs:

```pml
<grid cols=2>
  <card title="A">
    Hello
  </card>
  <card title="B">
    # Heading inside
  </card>
</grid>
```

## 15. AI authoring guidelines

- Prefer top-level headings only; do not emit headings inside components.
- Always put at least one space after `#` for headings.
- Use self-closing form for leaf viewers: `<artifact.summary id="…" />`.
- Keep attributes simple: double-quoted strings or bare tokens (`[A-Za-z0-9._-/:]+`).
- Separate paragraphs with a blank line; avoid trailing spaces.
- Indent children by two spaces for readability (not required but recommended).
- Do not emit raw HTML; only use Proofdown components.

## 16. Versioning and evolution

- This document defines Proofdown language v1.0.0. Backwards-compatible additions (e.g., new components, optional link macro) MAY appear in v1.x.
- Breaking grammar changes require a major version bump and coordination with parser and SSG.
- Every grammar rule SHOULD have a corresponding test case or golden artifact.

---

Appendix A: ABNF fragments (informative)

```
heading     = 1*"#" 1*SP text
text        = 1*( VCHAR / WSP )  ; implementation trims leading/trailing SP

name        = segment *( "." segment )
segment     = %x61-7A *( %x61-7A / %x30-39 / "-" )  ; a-z, a-z0-9-

start-tag   = "<" name *( 1*WSP attr ) WSP? "/>" / "<" name *( 1*WSP attr ) WSP? ">"
end-tag     = "</" name WSP? ">"

attr        = key "=" value
key         = ALPHA *( ALPHA / DIGIT / "." / "_" / "-" )
value       = dquote *( %x20-21 / %x23-5B / %x5D-7E ) dquote / bare
bare        = 1*( ALPHA / DIGIT / "." / "_" / "-" / "/" / ":" )

dquote      = '"'
```
