# Testing and Golden Artifacts

- Golden tests compare serialized AST JSON for fixtures under `tests/fixtures/*.pml` and goldens in `tests/golden/*.json`.
- Regenerate goldens only via environment switch:

```bash
UPDATE_GOLDEN=1 cargo test -p proofdown_parser
```

- Parser limits and error-path tests ensure safe failures with structured errors.
- Validator unit tests enforce attribute bounds and whitelist behavior.
- Fuzzing and property tests ensure robustness on adversarial inputs.
