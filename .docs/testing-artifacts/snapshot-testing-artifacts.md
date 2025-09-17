# Snapshot Testing Artifacts

Snapshot tests compare the current output to a stored baseline. Artifacts include baseline snapshots (text or images), diff images, and test results.

## Common producers

- Jest (`toMatchSnapshot`, `toMatchInlineSnapshot`, `jest-image-snapshot`)
- Storybook/Chromatic visual snapshots
- Playwright/Cypress snapshot matchers
- RSpec snapshot (Ruby), pytest-snapshot (Python)

## Artifact types

- Textual snapshots: `.snap` files (serialized AST/strings/DOM)
- Image snapshots: `baseline.png` / `actual.png` / `diff.png`
- HTML snapshots: serialized DOM dumps for debugging

## Proofdown viewer mapping

- Textual snapshots → `artifact.markdown` (pre-rendered) or `artifact.link` if large; show key diffs via code excerpts.
- Image snapshots → `artifact.image` triplets (baseline/actual/diff) for failing cases in a grid.
- Test result summaries → `artifact.json` + `artifact.table` for changed/added/removed snapshot counts.

## Implications for Proofdown Language Spec

- No special syntax is needed for snapshots; use `artifact.image` and `grid`/`card` to present baseline/actual/diff.
- Encourage small JSON summaries for counts (added/changed/removed) via `artifact.json` + `artifact.table`; renderer handles sorting/filtering.
- For large snapshot sets, default to collapsed views and link full artifacts via `artifact.link` to keep pages deterministic.
- Treat HTML/DOM dumps as data; escape at render time and avoid embedding scriptable content.
- Prefer digest-addressed image and text artifacts to ensure immutability across runs.

## Related specs and schemas

- Proofdown v1 grammar: ../../.specs/01_proofdown_language_v1.md
- Artifact-first semantics: ../../.specs/02_artifact_first_language_spec.md
- Proofdown v2 (additive): ../../.specs/03_proofdown_language_v2.md
- Authoring guide: ../proofdown-authoring-guide.md
- Table schema (snapshots): ../../.specs/schemas/snapshots.schema.json

## References

- Jest snapshots: https://jestjs.io/docs/snapshot-testing
- jest-image-snapshot: https://github.com/americanexpress/jest-image-snapshot
- Chromatic visual tests: https://www.chromatic.com/
- Playwright test reporters: https://playwright.dev/docs/test-reporters
