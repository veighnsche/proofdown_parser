# Releasing Proofdown

- Versioning: SemVer. Breaking JSON/API changes bump MAJOR.
- Process:
  1. Update CHANGELOG with notable changes and any migrations.
  2. Ensure CI is green (tests, coverage, audit, schema checks, wasm jobs).
  3. Tag the repo: `git tag -a vX.Y.Z -m "Release vX.Y.Z" && git push --tags`.
  4. crates.io:
     - `cargo publish -p proofdown_ast`
     - `cargo publish -p proofdown_parser`
     - `cargo publish -p proofdown_validate`
     - `cargo publish -p proofdown_cli`
  5. npm (WASM):
     - Ensure NPM_TOKEN is set in repo secrets.
     - GH Action `npm-publish.yml` will publish `examples/wasm/pkg-node`.
- Post-release:
  - Update documentation links, examples, and pin `comrak` range if needed.
