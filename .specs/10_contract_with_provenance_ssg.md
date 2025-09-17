# Contract: `proofdown_parser` ↔ `provenance_ssg`

Status: Draft (v1.0.0-rc)

This is the inter-team, versioned contract between the Proofdown Parser submodule and the Provenance Static Site Generator (SSG). It freezes the Rust API, data model, determinism, and error behavior that `provenance_ssg` relies on, so the parser team and external consultants can iterate independently without breaking the outer repo.

## 1) Scope

- The parser crate parses Proofdown (CMD) text into a deterministic AST. It performs syntax recognition and minimal structure checks.
- Semantic validation (component whitelist, attribute bounds, artifact ID existence, repo linking policy) is performed by the SSG (or a separate validator crate) and is NOT a responsibility of the parser under this contract.
- This contract covers the Rust API used by the SSG, stable AST shape (including serde JSON layout for golden tests), error surface, limits knobs, and feature flags relevant to SSG builds.

## 2) Crate & Build Targets

- Crate name (Cargo): `proofdown_parser`
- Rust target: `std` (no-std not required by SSG)
- Optional future target: `wasm32-unknown-unknown` via a sibling crate (not required by this contract)
- The crate MUST compile with Rust 1.75+ (or the repo toolchain in `rust-toolchain.toml`)

## 3) Dependency Shape (outer workspace)

In the outer workspace, `provenance_ssg` depends on the parser via an optional feature:

```toml
# crates/provenance_ssg/Cargo.toml
[dependencies]
proofdown_parser = { path = "../proofdown_parser/crates/proofdown_parser", optional = true }

[features]
external_pml = ["proofdown_parser"]
```

The SSG gates Proofdown parsing behind `cfg(feature = "external_pml")`. When disabled, SSG provides a fallback index page without Proofdown.

## 4) Public API Surface (stable)

The following items SHALL be considered stable and relied upon by the SSG. Additive changes are allowed (new enum variants, new fields with defaults) but MUST NOT break existing behavior.

```rust
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Document { pub blocks: Vec<Block> }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Block {
    Heading { level: u8, text: String },
    Paragraph(String),
    Component(Component),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Component {
    pub name: String,           // e.g., "grid", "card", "artifact.table"
    pub attrs: Vec<Attr>,       // order preserved; keys are case-sensitive
    pub children: Vec<Block>,   // empty for self-closing components
    pub self_closing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Attr { pub key: String, pub value: String }

// Parse Proofdown (UTF-8, \n newlines). Deterministic & pure (no I/O, no randomness).
pub fn parse(input: &str) -> Result<Document>;

// Convenience helper (stable): returns first matching attribute value if present.
pub fn find_attr<'a>(attrs: &'a [Attr], key: &str) -> Option<&'a str>;
```

Notes:
- AST is stable under serde JSON for golden testing:
  - `Block` uses `#[serde(tag = "type")]` with values: `"Heading"`, `"Paragraph"`, `"Component"`.
  - Field names are stable as shown.
- `parse()` MUST be deterministic: same input ⇒ same AST byte-for-byte under serde JSON.
- `parse()` returns only syntax/structure errors; semantic errors are out-of-scope here.

## 5) Error Model (forward contract)

While the MVP may use `anyhow::Error`, the stable contract requires transitioning to a structured error type by v1.0. The shape below is the target and shall be backwards-compatible to adopt:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorKind {
    Syntax,            // malformed tag, unterminated, unexpected close
    LimitExceeded,     // depth/nodes/size limit breached
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize,   // 1-based
    pub col: usize,    // 1-based, column in Unicode scalar values
    pub kind: ErrorKind,
    pub msg: String,   // human-oriented detail
}
```

Contracted behavior:
- All syntax failures MUST provide a best-effort line/column and a helpful message.
- Limits (see §6) MUST error with `LimitExceeded`.
- Unknown components/attributes SHALL NOT be errors at parse time (they are semantic); the SSG handles them.

## 6) Limits & Determinism

- Input encoding: UTF-8; newlines normalized to `\n` by the caller; parser treats `\r\n` as `\n`.
- Default limits (configurable in a follow-up API; initially constants):
  - max_depth: 16 nested components
  - max_nodes: 50,000 blocks
  - max_input_bytes: 1 MiB
- If a limit is exceeded, parser MUST return an error (see §5). No partial ASTs are emitted.
- The parser MUST be pure and side-effect free. No file/network I/O, no time, no RNG.

## 7) Compatibility & SemVer

- Versioning: SemVer.
  - Patch (x.y.z): bug fixes; no API or JSON AST changes.
  - Minor (x.y.0): additive (new components recognized syntactically, new enum variants, new helper fns). Must not break JSON AST of existing inputs.
  - Major (x.0.0): breaking changes (renames, JSON layout changes). Requires SSG coordination and a migration note. Avoid within a release cycle.
- The AST JSON layout is part of the compatibility surface. Any change requires at least a minor bump with backward-compatible defaults.

## 8) Whitespace, Text, and Escaping

- Heading text is trimmed of leading/trailing ASCII whitespace after the `#...<space>` prefix.
- Paragraphs are formed from non-empty lines until a blank or a component start `<`.
- Component attribute values preserve content exactly between quotes; bare values are parsed until whitespace or `>`.
- The parser MUST NOT perform HTML escaping; escaping is a renderer concern.

## 9) Minimal Component Recognition (syntax only)

At parse time, the following are recognized as components syntactically (names are case-sensitive strings):
- Structural: `grid`, `section`, `card`
- Artifact viewers: `artifact.summary`, `artifact.table`, `artifact.json`, `artifact.markdown`, `artifact.image`, `artifact.link`

Parser responsibility is to populate `Component { name, attrs, children, self_closing }`; it does not validate name/attrs. The SSG is free to reject unknown names during render.

## 10) Performance

- Expected linear time with respect to input size in the common case.
- The parser must avoid catastrophic backtracking or unbounded recursion; depth-limited descent.

## 11) Testing & Golden Artifacts

- The parser team SHALL publish golden AST JSONs for representative inputs in the submodule.
- The SSG team MAY use these for integration tests; byte-for-byte equality of serialized AST is the determinism gate.
- Error tests: malformed tags, unterminated elements, over-depth documents ⇒ structured `ParseError` with line/col.

## 12) Integration Responsibilities

- SSG loads Proofdown text and calls `proofdown_parser::parse(&str)`.
- On success, SSG renders with its own whitelist and artifact context. Unknown component names/attrs MUST be rejected by SSG at render time.
- On failure, SSG presents a clear error page sourced from `ParseError` (line/col/kind/message when available).

## 13) Change Management

- Any change to items in §4–§8 requires a PR labeled `contract-change` in the submodule and a corresponding bump in `provenance_ssg` if needed.
- Breaking changes require a migration note and joint approval from both teams.

---

Appendix A: Example serde JSON for a simple document

```json
{
  "blocks": [
    { "type": "Heading", "level": 1, "text": "QA Evidence for {{ commit }}" },
    { "type": "Component", "name": "grid", "attrs": [
      { "key": "cols", "value": "3" },
      { "key": "gap",  "value": "16" }
    ], "children": [
      { "type": "Component", "name": "card", "attrs": [ {"key": "title", "value": "Tests"} ], "children": [
        { "type": "Component", "name": "artifact.summary", "attrs": [ {"key": "id", "value": "tests-summary"} ], "children": [], "self_closing": true }
      ], "self_closing": false }
    ], "self_closing": false }
  ]
}
```
