# TODO — proofdown_cli

Scope: User-facing CLI for the Proofdown toolchain. Provide deterministic outputs, stable exit codes, great UX, and comprehensive tests. No grammar or validation logic here; it orchestrates parse/validate calls and presents results.

## Now / Next / Later

- [x] NOW: Baseline commands and flags
  - [x] `pml parse <file>`
  - [x] `pml validate <file>`
  - [x] `--json`, `--pretty`
  - [x] Limits: `--limits.depth`, `--limits.nodes`, `--limits.input-size`
  - [x] `--help|-h`, `--version|-V`
- [ ] NEXT: UX and output stability
  - [ ] Standardize JSON error payloads (keys: `ok`, `err.code`, `err.msg`, `err.line`, `err.col`)
  - [ ] Align `validate` errors to include `ValidateError::code()` in JSON mode
  - [ ] Human output: concise one-line errors (parse/validate) + hints
  - [ ] Consistent exit codes: 0=OK, 1=parse error, 2=validation error, 3=IO/usage
- [ ] LATER: Ergonomics and packaging
  - [ ] Read from stdin when `<file>` is `-` (document encoding expectations)
  - [ ] Shell completions (bash/zsh/fish) via `clap_complete` (optional)
  - [ ] Prebuilt binaries (GitHub Releases) or `cargo install` instructions
  - [ ] Windows/macOS/Linux smoke tests in CI matrix

## Tests (must have)

- [x] Pretty JSON output for `parse`
- [x] `validate` with JSON output and OK
- [x] Limits: depth enforced (failure)
- [x] Limits: input-size enforced (failure)
- [x] `--help` and `--version`
- [ ] File not found (usage exit code; JSON and human modes)
- [ ] Non-UTF8 input handling (fails clearly, JSON and human modes)
- [ ] Parse error path (unknown/malformed input) -> exit 1, stable stderr/stdout
- [ ] Validate error path (unknown component/attr) -> exit 2
- [ ] CRLF normalization: `parse` output equality under `--json` for LF vs CRLF inputs
- [ ] Snapshot tests for human-readable outputs (minimal, to avoid flakiness)

## UX & Docs

- [ ] Improve `usage()` with examples and limits flags documentation
- [ ] Document exit codes in `--help` and README
- [ ] Add a `--quiet` flag to suppress non-JSON messages (optional)

## CI

- [x] Workspace tests in main workflow
- [ ] Add CLI-focused job to run CLI tests with different shells (optional)

## Security & Safety

- [x] No network calls
- [x] No code execution
- [ ] Handle very large inputs gracefully (document time/memory expectations)

## Non-goals

- No grammar or validation logic in CLI — defer to `proofdown_parser` and `proofdown_validate`.
- No rendering — keep outputs textual/JSON only.
