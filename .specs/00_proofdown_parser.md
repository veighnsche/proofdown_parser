# Proofdown Parser — v1 Spec Outline

Status: Draft (Standalone)

This document defines the requirements and design for the `proofdown_parser` crate. It specializes in parsing Proofdown (Component Markdown, CMD) into a deterministic AST for rendering and fragment extraction.

## Scope and Standalone Note

This document is self-contained and normative for the `proofdown_parser` submodule. It includes the essential grammar, component registry, link macro ABNF, bounds, and error model so that the team owning this crate can work independently.

## Goals

- Deterministically parse Proofdown into a typed AST.
- Enforce a whitelisted component set and attribute schema.
- Provide precise, human-readable errors (line/column, kind, context).
- Offer a zero-IO core (pure parse) suitable for WASM.
- Bound resource usage for safety (depth, nodes, input size).

## Non-Goals

- Rendering HTML (handled by `renderers`).
- Fetching includes or artifacts (delegated to caller/Worker).
- HTML sanitization (renderers/Worker concern).

## v1 Scope (Minimum)

- Blocks: headings `#..####`, paragraphs, component blocks.
- Components:
  - Structural: `grid`, `section`, `card` (minimum set used by examples)
  - Artifact viewers: `artifact.summary`, `artifact.table`, `artifact.json`, `artifact.markdown`, `artifact.image`, `artifact.link`
- Attributes: strings only; enforce known keys per component, with simple type bounds.
- Link macro: `[[a:artifact_id | Optional Title]]` — optional for v1 if not needed by examples.
- Includes: Out of scope for v1 (placeholder for v1.x).
- Limits (defaults): depth ≤ 16, nodes ≤ 50k, input ≤ 1 MiB.

## Grammar (Informal)

- Heading: `^#{1,4}\s+TEXT$`
- Paragraph: non-empty line until blank or component start.
- Component (block):
  - Start tag: `<name key="value" ...>` or `<name key="value" ... />`
  - End tag: `</name>` (required unless self-closing)
  - Children: zero or more blocks until matching end tag.
- Attributes: key=value, values quoted with `"..."` or bare until whitespace/`>`.
- Whitelist: `grid|section|card|artifact.(summary|table|json|markdown|image|link)`.

## File & Versioning

- Encoding: UTF-8, normalized newlines `\n`.
- File extension: `.pml` (Proof Markup Language).
- Versioning: Grammar stability is governed by the top-level Index `version`. Unknown major versions MUST be rejected by callers.

## Syntax Overview

Proofdown combines:

- Markdown-like blocks: paragraphs, headings (`#`..`####`), lists, code fences, and inline code.
- A minimal HTML-like component syntax for typed blocks/inlines, e.g., `<grid cols=3> ... </grid>`.
- Interpolation of verified Index fields via `{{ field }}` (text-only, HTML-escaped).
- A link macro `[[...]]` with artifact/repo-aware protocols.

## AST (Sketch)

```rust
pub struct Document { pub blocks: Vec<Block> }

pub enum Block {
    Heading { level: u8, text: String },
    Paragraph(String),
    Component(Component),
}

pub struct Component {
    pub name: String,            // e.g., "artifact.summary"
    pub attrs: Vec<Attr>,        // validated later
    pub children: Vec<Block>,    // empty for self-closing
    pub self_closing: bool,
}

pub struct Attr { pub key: String, pub value: String }

pub enum ErrorKind {
    UnterminatedTag { name: String },
    UnexpectedClose { name: String },
    UnknownComponent { name: String },
    UnknownAttribute { name: String, key: String },
    AttrType { key: String, expected: &'static str },
    DepthExceeded,
    SizeLimit,
    Syntax,
}

pub struct ParseError { pub line: usize, pub col: usize, pub kind: ErrorKind, pub msg: String }
```

## API Surface

- `parse(input: &str) -> Result<Document, ParseError>`
- `validate(doc: &Document, limits: &Limits) -> Result<(), ParseError>`
- Optional: `wasm_parse(input: &str) -> JsValue` via `wasm-bindgen` (v1.x)

## Determinism & Limits

- Pure functions; no environment access.
- Deterministic traversal order and attribute normalization.
- Configurable limits; defaults provided; all limit breaches yield `ErrorKind` with location.

## Security Model

- Parser only recognizes a fixed component whitelist.
- No script execution or HTML; output is a structured AST.
- Sanitization of rendered text is the responsibility of renderers/Worker.

## Testing

- Golden fixtures derived from `/.specs/10_proofdown.md` examples.
- Error cases: unknown component, bad nesting, bad attributes, depth exceeded.
- Determinism: parse -> serialize AST (stable JSON) -> compare across runs.

## Roadmap

- v1.0:
  - Minimal grammar + components (as used in `examples/minimal`).
  - Whitelist + attribute validation and limits.
  - Rust API + unit tests.
- v1.1:
  - Link macro parsing.
  - WASM bindings.
  - Expanded components (`tabs`, `gallery`, `section`).
- v1.2:
  - Includes with cycle detection and base path policy.

## Link Macro (ABNF)

```
link      = "[[" SP* target SP* ( "|" SP* label SP* )? "]]"
label     = 1*( %x21-7E ) ;  ; printable ASCII (trimmed)

target    = a_target / repo_target / src_target / doc_target / gh_target / ci_target / sym_target / path_shorthand

a_target      = "a:" id
repo_target   = "repo:" path [ linefrag ]
src_target    = "src:" path [ linefrag ]
doc_target    = "doc:#" anchor
gh_target     = "gh:" ( "issue:" 1*DIGIT / "pr:" 1*DIGIT )
ci_target     = "ci:" ( "run" / "job:" jobname )
sym_target    = "sym:" path "::" symbol

path_shorthand = path [ linefrag ] ; interpreted as repo:<path>

id         = 1*( ALPHA / DIGIT / "-" / "_" )
path       = 1*( ALPHA / DIGIT / "/" / "." / "-" / "_" )
anchor     = 1*( ALPHA / DIGIT / "-" / "_" )
jobname    = 1*( ALPHA / DIGIT / "-" / "_" )
symbol     = 1*( VCHAR )
linefrag   = "#L" 1*DIGIT [ "-L" 1*DIGIT ]
```

Semantics:

- `path_shorthand` MUST validate under repository resolution rules (commit-pinned, no traversal).
- Invalid targets are hard errors.

## Component Registry and Bounds (v1)

Structural:

- `grid(cols=1..6, gap=0..64)`
- `section(title: string)`
- `card(title: string)`

Artifact viewers:

- `artifact.summary(id)` — expects `summary:test` JSON
- `artifact.table(id)` — expects coverage JSON
- `artifact.json(id, collapsed=true|false, depth=0..8)`
- `artifact.markdown(id)`
- `artifact.image(id, alt, max_height=128..2048)`
- `artifact.link(id, download=true|false, title)`

Repository viewers (optional in v1 if artifacts exist to resolve):

- `repo.code(path, range, lang, highlight)` — range line count SHOULD be ≤ 400
- `repo.link(path, label, lines)`
- `repo.tree(path, depth=1..5, include, exclude)`
- `repo.diff(base_id, head_id, path, context=0..10)`
- `repo.symbol(path, name)`

All unknown attributes MUST error. Bounds are enforced at validation time.

## Attributes & Types

- Integers: decimal, bounded per component (e.g., `cols=3`, `gap=16`).
- Booleans: `true|false`.
- Enums: validated against a fixed set (e.g., `kind="llm-proof"`).
- Strings: quoted; bare allowed if matches `[A-Za-z0-9._\-/:]+`.
- Unknown/malformed attributes MUST be rejected.
- Glob patterns: components that accept `include`/`exclude` MUST validate patterns against a safe subset (no `..` traversal; anchored under repo root).
- Enforced bounds: components MUST enforce reasonable bounds (e.g., `repo.tree.depth <= 5`).

## Security & Verification

- Parser MUST error on unknown components/attributes.
- Parser has no IO; callers must ensure referenced resources exist in a verified Index.
- All text is sanitized at render time; this crate outputs only structured AST.

## Example

```pml
# QA Evidence for {{ commit }}

<grid cols=3 gap=16>
  <card title="Tests">
    <artifact.summary id="tests-summary" />
    [[a:coverage | Full Coverage Report]]
  </card>
  <card title="Coverage">
    <artifact.table id="coverage" />
  </card>
  <card title="Key Failures">
    <artifact.markdown id="failures" />
  </card>
</grid>
```
