# Fuzzing Artifacts

Fuzzing explores program behavior with large numbers of generated inputs, often guided by coverage. Artifacts include the input corpus, crash/minimized reproducer files, logs, sanitizer traces, and optional coverage reports.

## Common producers

- AFL/AFL++ (coverage-guided, forkserver)
- libFuzzer (LLVM, in-process)
- Honggfuzz
- Jazzer (JVM libFuzzer integration)
- Go fuzzing (`go test -fuzz`), cargo-fuzz (Rust, libFuzzer)
- OSS-Fuzz (CI for open source; aggregates above)

## Artifact types and layout (typical)

- Corpus directory: `queue/` (interesting inputs)
- Crashes: `crashes/` (files triggering SIGSEGV/ASan/etc.)
- Hangs/Timeouts: `hangs/`
- Minimization outputs: `minimized/`
- Logs: stdout/stderr with stack traces and sanitizer messages
- Coverage: `llvm-cov` JSON / HTML from instrumented builds

## Example: AFL run directory

```
findings/
  queue/
  crashes/
  hangs/
  fuzzer_stats
```

## Proofdown viewer mapping

- Crash reproducers → `artifact.link` to raw files; summarize stack traces and addresses as `artifact.json` snippets.
- Corpus stats and coverage → `artifact.json` plus link to `llvm-cov` HTML via `artifact.link`.
- Logs/sanitizer output → `artifact.markdown` (truncated) and full logs via `artifact.link`.

## Implications for Proofdown Language Spec

- Treat crash inputs and corpora as opaque binaries via `artifact.link`; keep pages deterministic by summarizing key details in `artifact.json`.
- Encourage small, redacted JSON summaries of stack traces and sanitizer outputs; avoid embedding huge logs.
- Coverage visualizations (llvm-cov HTML) should be linked; present minimal JSON rollups (files covered, functions hit) inline.
- No new syntax needed: use existing components (`artifact.json`, `artifact.link`, `artifact.markdown`) with bounds like `collapsed=true`.
- Enforce digest-addressed artifacts to guarantee reproducibility across fuzzing runs.

## Related specs and schemas

- Proofdown v1 grammar: ../../.specs/01_proofdown_language_v1.md
- Artifact-first semantics: ../../.specs/02_artifact_first_language_spec.md
- Proofdown v2 (additive): ../../.specs/03_proofdown_language_v2.md
- Authoring guide: ../proofdown-authoring-guide.md
- Table schema (fuzzing): ../../.specs/schemas/fuzzing.schema.json

## References

- AFL docs: <https://afl-1.readthedocs.io/en/latest/fuzzing.html>
- AFL++: <https://aflplus.plus/>
- libFuzzer (LLVM): <https://llvm.org/docs/LibFuzzer.html>
- Reproducing libFuzzer crashes: <https://chromium.googlesource.com/chromium/src/+/lkgr/testing/libfuzzer/reproducing.md>
