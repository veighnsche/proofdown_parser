# Limits and Safety

- Limits
  - Depth ≤ 16, Nodes ≤ 50k, Input ≤ 1 MiB (configurable via API in CLI; defaults enforced in parser/validator).
  - Exceeding a limit yields a `LimitExceeded` parse/validate error with clear messaging.
- Determinism
  - Same input ⇒ same AST byte-for-byte under serde JSON; golden tests verify this.
- Safety model
  - No scripting/HTML content; any rendering layers must escape content.
  - Artifacts are content-addressed; callers verify digest before render.
  - Fail closed on digest mismatches or missing artifacts.
