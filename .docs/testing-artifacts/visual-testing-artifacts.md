# Visual Regression Testing Artifacts

Visual testing detects unintended UI changes by comparing snapshots (images or DOM) against a known baseline. Artifacts include baseline images, current images, diffs, HTML dashboards, and JSON summaries.

## Common producers

- Applitools Eyes (hosted visual AI; dashboards and API results)
- Percy (BrowserStack Percy; snapshots and build artifacts)
- Chromatic (Storybook-driven snapshots and diffs)
- Playwright/Cypress plugins (e.g., `@playwright/test`'s trace + screenshot diffs, `cypress-image-snapshot`)
- Jest Image Snapshot (`jest-image-snapshot`)

## Artifact formats

- Image triplets: `baseline.png`, `actual.png`, `diff.png`
- Snapshot metadata: JSON (per run/per spec), including thresholds and mismatch percentages
- Hosted dashboards: HTML bundles (Applitools, Percy, Chromatic)
- Playwright traces (`trace.zip`) with screenshots and DOM snapshots

## Typical files

- `__image_snapshots__/spec-name.snap.png` and diffs
- `chromatic`/`percy` build URLs (API-accessible)
- `playwright-trace.zip` and `playwright-report/`

## Proofdown viewer mapping

- Showcase key diffs inline via `artifact.image` in a `grid` (baseline, actual, diff) for failing cases.
- Large snapshot sets → package as an archive and link via `artifact.link`; include a JSON summary using `artifact.json`.
- Hosted dashboards → `artifact.link` to the dashboard plus a rollup table (changed/added/removed stories, pass/fail counts).
- Traces → link to trace ZIP and include instructions to open locally; optionally extract a small GIF/PNG preview.

## Implications for Proofdown Language Spec

- Use existing primitives: `grid`/`card` + `artifact.image` are sufficient to present baseline/actual/diff triplets—no new syntax needed.
- Keep reports deterministic: link hosted dashboards (Applitools/Percy/Chromatic) via `artifact.link`; avoid embedding scriptable widgets.
- Encourage JSON summaries of runs (changed/added/removed, pass/fail counts) via `artifact.json` + `artifact.table`; collapse large trees by default.
- Images can be large; recommend `max_height` attribute on `artifact.image` and thumbnail-first strategies in renderer.
- Trace archives (Playwright) should be treated as opaque files; interactive viewing occurs outside Proofdown via allowlisted viewers.

## References

- Applitools Eyes: https://applitools.com/platform/eyes/
- Percy basics: https://docs.percy.io/docs/percy-platform-basics
- Chromatic for Storybook: https://www.chromatic.com/storybook
- Playwright reporters and traces: https://playwright.dev/docs/test-reporters
- `jest-image-snapshot`: https://github.com/americanexpress/jest-image-snapshot
