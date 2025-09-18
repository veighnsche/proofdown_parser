# proofdown_parser

[![CI](https://img.shields.io/github/actions/workflow/status/veighnsche/proofdown_parser/ci.yml?branch=main&label=CI)](https://github.com/veighnsche/proofdown_parser/actions/workflows/ci.yml)
[![mdBook](https://img.shields.io/badge/docs-mdBook-blue.svg)](book/book/index.html)
[![GitHub Pages](https://img.shields.io/badge/docs-GitHub%20Pages-blue?logo=github)](https://veighnsche.github.io/proofdown_parser/)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
[![Rust Edition](https://img.shields.io/badge/rust-2021-orange.svg)](Cargo.toml)
[![MSRV](https://img.shields.io/badge/MSRV-1.75%2B-informational.svg)](Cargo.toml)

Proofdown Parser parses Proofdown (PML: Proof Markup Language) into a deterministic, typed AST (`proofdown_ast`) for use by validators, renderers, CLIs, and WASM targets. The language supports full Markdown (CommonMark + selected GFM: tables, strikethrough, autolink, task lists) plus a small, HTML-like component system. v2 adds purely additive semantics (captions, JSON Pointers, table column selectors, text viewer) enforced by a validator.

Highlights

- Deterministic AST JSON for golden tests and cross-platform parity
- Syntax-only parser; semantics enforced downstream by `proofdown_validate`
- Strict limits (depth, nodes, input size) and robust error model
- CLI (`pml`) for parse/validate with JSON output and stable exit codes
- WASM build for browser use; mdBook docs for language and contract

## Quick links

- mdBook (local artifact): [book/book/index.html](book/book/index.html)
- mdBook (GitHub Pages): https://veignsche.github.io/proofdown_parser/ (optional, if Pages enabled)
- Language v1 spec: [.specs/01_proofdown_language_v1.md](.specs/01_proofdown_language_v1.md)
- Additive semantics v2: [.specs/03_proofdown_language_v2.md](.specs/03_proofdown_language_v2.md)
- Artifact-first semantics: [.specs/02_artifact_first_language_spec.md](.specs/02_artifact_first_language_spec.md)
- Authoring guide (LLM-friendly): [.docs/proofdown-authoring-guide.md](.docs/proofdown-authoring-guide.md)
- Testing artifacts catalog: [.docs/testing-artifacts/README.md](.docs/testing-artifacts/README.md)
- Table row schemas (JSON): [.specs/schemas/README.md](.specs/schemas/README.md)
- SSG contract (stable API/AST): [.specs/10_contract_with_provenance_ssg.md](.specs/10_contract_with_provenance_ssg.md)

## Status

- Stable API: `parse(&str) -> Result<Document, ParseError>`, `parse_with_limits(&str, ParserLimits)`
- Determinism: enforced by golden JSON tests across v1/v2 fixtures
- Validator: `proofdown_validate` enforces whitelist and bounds (v2 features)
- CLI: `pml parse|validate` with JSON outputs; standard exit codes
- WASM: `proofdown_wasm` target; CI job builds; demo scaffold in `examples/wasm/`

## Local CI (preflight)

Run a local equivalent of the CI to avoid surprises before pushing:

```bash
bash scripts/ci_local.sh                # full run
bash scripts/ci_local.sh --skip-wasm --skip-mdbook --skip-fuzz  # faster loop
```

This mirrors: fmt, clippy, tests, schema checks, benches, CLI smoke over fixtures, optional wasm/mdBook builds, and fuzz smoke.

---

## Legacy README (historical, kept for context)

## Status

- Maturity: Production-ready parser crate with typed errors, validator, CLI, and WASM build.
- Implemented API: `parse(&str) -> Result<Document, ParseError>`, `parse_with_limits(&str, ParserLimits)`; helper `find_attr`.
- Validator: `proofdown_validate::validate(&Document, Option<&Limits>) -> Result<(), ValidateError>` enforces whitelist and attribute bounds (v2 features included).
- CLI: `pml parse|validate` with `--json`, `--pretty`, and `--limits.*` flags; documented exit codes.
- WASM: `proofdown_wasm` exports `wasm_parse(input: &str) -> String` JSON payload; CI builds wasm32 target.
- Next: link macro parsing (planned v1.1) and includes (planned v1.2).

## What is Proofdown?

Proofdown is a Markdown-like format with a small, HTML-like component system for typed, securable blocks used across Provenance. It combines:

- Markdown-style blocks: headings (`#`..`####`), paragraphs, lists, and code fences.
- Minimal component tags for structured UI blocks, e.g. `<grid cols=3> ... </grid>`.
- Optional interpolations of verified Index fields via `{{ field }}` (text-only, escaped at render time).
- A link macro `[[...]]` for artifact- and repo-aware deep links.

Files use UTF-8 with `\n` newlines and the `.pml` extension (Proof Markup Language).

## Workspace Layout

This submodule hosts a nested Cargo workspace to allow independent iteration of parser-related crates. Proposed layout (from `.plans/00_workspace_plan.md`):

```
proofdown_parser/
├─ Cargo.toml                 # [workspace] root (nested under parent repo)
├─ crates/
│  ├─ proofdown_ast/          # AST and error types, serde
│  ├─ proofdown_lexer/        # optional tokenizer (planned)
│  ├─ proofdown_parser/       # parser crate (this)
│  ├─ proofdown_validate/     # whitelist + attribute/limits
│  ├─ proofdown_wasm/         # wasm-bindgen bindings
│  └─ proofdown_cli/          # small CLI for dev/testing
└─ .specs/ and .plans/
```

Current workspace members (from root `Cargo.toml`):

```toml
[workspace]
members = [
  "crates/proofdown_ast",
  "crates/proofdown_parser",
  "crates/proofdown_validate",
  "crates/proofdown_cli",
  "crates/proofdown_wasm",
  "crates/schema_check",
  "crates/proofdown_integration_example",
]
resolver = "2"
```

Parent workspace depends on inner crates via path dependencies (do not include this nested workspace as a member in the parent):

```toml
# In the parent Cargo.toml of a consumer crate
[dependencies]
proofdown_parser = { path = "crates/proofdown_parser/crates/proofdown_parser" }
```

## Goals

- Deterministically parse Proofdown into a typed AST.
- Enforce a whitelisted component set and attribute schema (via validation pass).
- Provide precise, human-readable errors with line/column, kind, and context.
- Offer a zero-IO core suitable for WASM and sandboxed execution.
- Bound resource usage for safety (depth, nodes, input size).

## Non-Goals

- Rendering HTML (delegated to renderers/Worker).
- Fetching includes or artifacts (delegated to caller/Worker).
- HTML sanitization (render-time responsibility).

## v1 Scope (Minimum)

- Blocks: headings `#..####`, paragraphs, component blocks.
- Components (whitelisted):
  - Structural: `grid`, `section`, `card` (minimum set used by examples)
  - Artifact viewers: `artifact.summary`, `artifact.table`, `artifact.json`, `artifact.markdown`, `artifact.image`, `artifact.link`
- Attributes: strings only; enforce known keys per component with simple bounds.
- Link macro `[[a:artifact_id | Optional Title]]`: optional for 1.0; planned for 1.1.
- Includes: out of scope for 1.0 (planned for 1.2 with cycle detection).
- Default limits: depth ≤ 16, nodes ≤ 50k, input ≤ 1 MiB.

## Grammar (Informal Overview)

- Markdown blocks: headings (ATX `#..####`), paragraphs, block quotes, thematic breaks, fenced/indented code blocks, lists (unordered/ordered with start/tight), GFM task items, GFM tables (optional header, column alignment).
- Markdown inlines: text, emphasis, strong, strikethrough (GFM), inline code, soft/hard breaks, links (url/title), images (url/title, alt from inline text).
- Components (block):
  - Start tag: `<name key="value" ...>` or `<name key="value" ... />`
  - End tag: `</name>` (required unless self-closing)
  - Children: zero or more blocks (full Markdown supported) until matching end tag.
- Attributes: `key=value`, quoted with `"..."` or bare until whitespace/`>`.
  Note: the parser recognizes component syntax generically. Whitelisting component names and attribute bounds is enforced by the validator (see Validation). Raw HTML blocks/inline are dropped.

## AST Types (proofdown_ast crate)

The AST types live in `crates/proofdown_ast/src/lib.rs` and derive `serde` for stable JSON round-tripping:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Document { pub blocks: Vec<Block> }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Block {
    Heading { level: u8, inlines: Vec<Inline> },
    Paragraph { inlines: Vec<Inline> },
    BlockQuote { children: Vec<Block> },
    ThematicBreak,
    CodeBlock { info: String, text: String },
    List { kind: ListKind, start: Option<u64>, tight: bool, items: Vec<ListItem> },
    Table { align: Vec<TableAlign>, header: Option<TableRow>, rows: Vec<TableRow> },
    Component(Component),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListKind { Bullet, Ordered }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListItem { pub children: Vec<Block>, pub task: Option<bool> }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Inline {
    Text { text: String }, Emph { children: Vec<Inline> }, Strong { children: Vec<Inline> },
    Strikethrough { children: Vec<Inline> }, Code { text: String }, SoftBreak, HardBreak,
    Link { url: String, title: Option<String>, children: Vec<Inline> },
    Image { url: String, title: Option<String>, alt: String }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TableAlign { None, Left, Center, Right }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableRow { pub cells: Vec<Vec<Inline>> }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Component {
    pub name: String,
    pub attrs: Vec<Attr>,
    pub children: Vec<Block>,
    pub self_closing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Attr { pub key: String, pub value: String }
```

Note: The `proofdown_ast` crate hosts these types and the structured error model shared across crates.

## Public API

Current API (typed):

```rust
pub fn parse(input: &str) -> Result<Document, ParseError>
pub fn parse_with_limits(input: &str, limits: ParserLimits) -> Result<Document, ParseError>
```

Helper:

```rust
pub fn find_attr<'a>(attrs: &'a [Attr], key: &str) -> Option<&'a str>
```

Related crates/APIs:

- `proofdown_validate::validate(doc: &Document, limits: Option<&Limits>) -> Result<(), ValidateError>`
- WASM surface (via `wasm-bindgen`): `wasm_parse(input: &str) -> String` returning JSON `{ ok, doc|err }`

## Usage Example

```rust
use proofdown_parser::parse;

let input = r#"
# QA Evidence for {{ commit }}

<grid cols=3 gap=16>
  <card title="Tests">
    <artifact.summary id="tests-summary" />
  </card>
</grid>
"#;

let doc = parse(input)?; // Document with typed blocks/components
```

## Example Proofdown (PML)

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

## Link Macro (planned v1.1, ABNF excerpt)

```abnf
link      = "[[" SP* target SP* ( "|" SP* label SP* )? "]]"
label     = 1*( %x21-7E )

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

Semantics: `path_shorthand` validates under repository rules (commit-pinned, no traversal). Invalid targets are hard errors during parsing/validation.

## Component Registry (v1)

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

Repository viewers (optional for v1 if artifacts exist to resolve):

- `repo.code(path, range, lang, highlight)` — range line count SHOULD be ≤ 400
- `repo.link(path, label, lines)`
- `repo.tree(path, depth=1..5, include, exclude)`
- `repo.diff(base_id, head_id, path, context=0..10)`
- `repo.symbol(path, name)`

Note: These are the component names used downstream by the SSG. The parser recognizes component syntax and records `Component { name, attrs, children, self_closing }` but does not enforce a whitelist. Unknown component names/attributes are not parse errors; they are validated by the SSG or the `proofdown_validate` crate.

## Determinism & Limits

- Pure functions; no environment access.
- Deterministic traversal order and attribute normalization.
- Limits are a parser responsibility. Default limits (depth ≤ 16, nodes ≤ 50k, input ≤ 1 MiB). Breaches produce errors (structured error model implemented).

## Security Model

- Parser does not enforce a component whitelist. It recognizes component syntax only; whitelist/attribute validation happen downstream (SSG or validator crate).
- No script execution or HTML emission; output is a structured AST only.
- Text sanitization happens during render; this crate is purely structural.

## Testing Strategy

- Golden fixtures derived from `.specs/00_proofdown_parser.md` examples.
- Parser error cases: malformed tags, unterminated/mismatched components, malformed attributes, and limits exceeded.
- Unknown components/attributes are not parse errors and are covered by downstream validation tests.
- Determinism: parse → serialize AST (stable JSON) → byte-for-byte compare across runs.

## Roadmap

- 1.0:
  - Minimal grammar + components to power examples.
  - Whitelist + attribute validation and limits.
  - Rust API + unit tests and golden tests.
- 1.1:
  - Link macro parsing.
  - WASM bindings (`wasm-bindgen`).
  - Expanded components (`tabs`, `gallery`, additional `section` behaviors).
- 1.2:
  - Includes with cycle detection and base path policy.

## Testing

- Unit tests live under each crate's `tests/` directory.
- The parser crate (`proofdown_parser`) includes smoke tests and golden tests.

### Case-based Parser Test Harness

Location: `crates/proofdown_parser/tests/cases_harness.rs`

- The harness discovers case directories under `crates/proofdown_parser/tests/cases/`.
- Each case is a directory containing at least `input.pml` and one of:
  - `expected.json` if parsing should succeed, containing the AST JSON structure
  - `expected_error.json` if parsing should fail, containing the structured error JSON `{ line, col, kind, msg }`

Run all cases:

```bash
cargo test -p proofdown_parser --test cases_harness
```

Update or create goldens automatically (writes expected files):

```bash
UPDATE_GOLDEN=1 cargo test -p proofdown_parser --test cases_harness
```

Add a new case:

1. Create a new folder under `crates/proofdown_parser/tests/cases/<case_name>/`.
2. Add `input.pml` with the test document content.
3. Run with `UPDATE_GOLDEN=1` to generate `expected.json` (success) or `expected_error.json` (error), or write them manually.
4. Re-run tests normally to verify.

Seeded examples include:

- `heading_basic`, `heading_no_space`
- `paragraph_multi_line`
- `component_self_closing`, `component_nested`
- `attrs_malformed_token`
- `error_unterminated_component`, `error_mismatched_close`
- `limits_depth_exceeded`

## Building

Build and test the parser crate:

```bash
cargo build -p proofdown_parser
cargo test  -p proofdown_parser
```

## Error Model (contract §5)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorKind { Syntax, LimitExceeded }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParseError { pub line: usize, pub col: usize, pub kind: ErrorKind, pub msg: String }
```

Parser returns `ParseError` with best-effort line/col and kind on syntax and limit failures.

## CLI and WASM (implemented)

- CLI (`proofdown_cli`):
  - `pml parse <file> [--json] [--pretty] [--limits.depth=N] [--limits.nodes=N] [--limits.input-size=BYTES]` (exit codes: 0 OK, 1 parse error, 3 IO/usage)
  - `pml validate <file> [--json] [--pretty] [--limits.depth=N] [--limits.nodes=N] [--limits.input-size=BYTES]` (exit codes: 0 OK, 1 parse error, 2 validation error, 3 IO/usage)
  - Top-level: `--help|-h`, `--version|-V`
- WASM (`proofdown_wasm`):
  - `wasm_parse(input: &str) -> String` returning a JSON string `{ ok, doc|err }` (enable `wasm` feature; build for `wasm32-unknown-unknown`).

## Integration with Parent Workspace

- Do not include this nested workspace as a member of the parent.
- Depend via `path` on inner crates from the parent.
- Gate usage with features until crates stabilize.

### Feature-gated integration example (in `provenance_ssg`)

```toml
# crates/provenance_ssg/Cargo.toml
[dependencies]
proofdown_parser = { path = "../proofdown_parser/crates/proofdown_parser", optional = true }

[features]
external_pml = ["proofdown_parser"]
```

In code, gate parser usage:

```rust
#[cfg(feature = "external_pml")]
{
    let doc = proofdown_parser::parse(pml_text)?;
    // ... render ...
}
```

## License

MIT OR Apache-2.0 (see `Cargo.toml`).

## Maintainers

- Owner: @veighnsche
