# proofdown_parser

Submodule placeholder for Proofdown parser.

Proofdown Parser parses Proofdown (Component Markdown, CMD) into a deterministic, typed AST for rendering and fragment extraction within the Provenance system.

This README summarizes the v1 parser scope, grammar, AST, API, limits, security model, component registry, and roadmap, drawing from `.specs/00_proofdown_parser.md`, `.plans/00_workspace_plan.md`, and the current crate implementation at `crates/proofdown_parser/src/lib.rs`.

## Status

- Maturity: MVP in-repo parser crate with a minimal, pure parser.
- Implemented API: `parse(&str) -> anyhow::Result<Document>` and helpers.
- Upcoming: typed error model (`ParseError`/`ErrorKind`), whitelist + attribute validation, limits enforcement, link macro, CLI, and WASM bindings.

## What is Proofdown?

Proofdown is a Markdown-like format with a small, HTML-like component system for typed, securable blocks used across Provenance. It combines:

- Markdown-style blocks: headings (`#`..`####`), paragraphs, lists, and code fences.
- Minimal component tags for structured UI blocks, e.g. `<grid cols=3> ... </grid>`.
- Optional interpolations of verified Index fields via `{{ field }}` (text-only, escaped at render time).
- A link macro `[[...]]` for artifact- and repo-aware deep links (planned).

Files use UTF-8 with `\n` newlines and the `.pml` extension (Proof Markup Language).

## Workspace Layout

This submodule hosts a nested Cargo workspace to allow independent iteration of parser-related crates. Proposed layout (from `.plans/00_workspace_plan.md`):

```
proofdown_parser/
├─ Cargo.toml                 # [workspace] root (nested under parent repo)
├─ crates/
│  ├─ proofdown_ast/          # AST and error types, serde (planned)
│  ├─ proofdown_lexer/        # optional tokenizer (planned)
│  ├─ proofdown_parser/       # parser crate (this)
│  ├─ proofdown_validate/     # whitelist + attribute/limits (planned)
│  ├─ proofdown_wasm/         # wasm-bindgen bindings (planned)
│  └─ proofdown_cli/          # small CLI for dev/testing (planned)
└─ .specs/ and .plans/
```

Current workspace members (from root `Cargo.toml`):

```toml
[workspace]
members = [
  "crates/proofdown_parser",
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

- Heading: `^#{1,4}\s+TEXT$`
- Paragraph: non-empty line until blank or component start.
- Component (block):
  - Start tag: `<name key="value" ...>` or `<name key="value" ... />`
  - End tag: `</name>` (required unless self-closing)
  - Children: zero or more blocks until matching end tag.
- Attributes: `key=value`, quoted with `"..."` or bare until whitespace/`>`.
- Whitelist: `grid|section|card|artifact.(summary|table|json|markdown|image|link)`.

## AST Types (current crate)

The current parser defines the AST in `crates/proofdown_parser/src/lib.rs` and derives `serde` for stable JSON round-tripping:

```rust
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
    pub name: String,
    pub attrs: Vec<Attr>,
    pub children: Vec<Block>,
    pub self_closing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Attr { pub key: String, pub value: String }
```

Note: A separate `proofdown_ast` crate is planned to host these types and a typed error model shared across crates.

## Public API

Current API (MVP):

```rust
pub fn parse(input: &str) -> anyhow::Result<Document>
```

Helper:

```rust
pub fn find_attr<'a>(attrs: &'a [Attr], key: &str) -> Option<&'a str>
```

Planned API additions:

- `validate(doc: &Document, limits: &Limits) -> Result<(), ParseError>`
- WASM surface (via `wasm-bindgen`): `wasm_parse(input: &str) -> JsValue`

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

## Link Macro (ABNF excerpt)

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

Unknown components/attributes MUST error during the validation pass.

## Determinism & Limits

- Pure functions; no environment access.
- Deterministic traversal order and attribute normalization.
- Configurable limits with sensible defaults; all limit breaches produce structured errors.

## Security Model

- Parser recognizes a fixed component whitelist.
- No script execution or HTML emission; output is a structured AST only.
- Text sanitization happens during render; this crate is purely structural.

## Testing Strategy

- Golden fixtures derived from `.specs/00_proofdown_parser.md` examples.
- Error cases: unknown component, bad nesting, bad attributes, depth exceeded.
- Determinism: parse → serialize AST (stable JSON) → compare across runs.

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

## Building

Build and test the parser crate:

```bash
cargo build -p proofdown_parser
cargo test  -p proofdown_parser
```

## CLI and WASM (planned)

- CLI (`proofdown_cli`):
  - `pml parse <file>` → exit non-zero on error; `--json` dumps AST JSON.
  - `pml validate <file>` → runs whitelist/limits validation.
- WASM (`proofdown_wasm`):
  - `parse(input: &str) -> JsValue` returning stable JSON AST and structured errors.

## Integration with Parent Workspace

- Do not include this nested workspace as a member of the parent.
- Depend via `path` on inner crates from the parent.
- Gate usage with features until crates stabilize.

## License

MIT OR Apache-2.0 (see `Cargo.toml`).

## Maintainers

- Owner: @veighnsche
